use anyhow::Context;
use async_compression::tokio::write::GzipDecoder;
use futures_util::stream::{self, StreamExt as _};
use nix::unistd::{chown, Group, User};
use tokio::{fs, io::AsyncWriteExt as _};
use tracing::error;

use crate::schema::{Encoding, UserData};

#[tracing::instrument(level = "debug")]
pub async fn execute_user_data(user_data: UserData<'_>) {
  if let Some(hostname) = user_data.hostname {
    if let Err(e) = hostname::set(&*hostname) {
      error!("Failed to set hostname: {:?}", e);
    }
  }

  stream::iter(user_data.files)
    .for_each_concurrent(None, |file| async move {
      let path = file.path.as_ref();
      let write_file = async move {
        if let Some(parent) = path.parent() {
          fs::create_dir_all(parent).await?;
        }

        let mut writer = fs::OpenOptions::new()
          .create(true)
          .truncate(!file.append)
          .append(file.append)
          .mode(u32::from_str_radix(&file.permissions, 8)?)
          .write(true)
          .read(false)
          .open(&path)
          .await?;

        if let Some(owner) = file.owner {
          if let Some((user, group)) = owner.split_once(':') {
            let user = User::from_name(user)?.context("no such user")?;
            let group = Group::from_name(group)?.context("no such group")?;
            chown(path, Some(user.uid), Some(group.gid))?;
          }
        }

        match file.encoding {
          Encoding::Plain => writer.write_all(file.content.as_bytes()).await?,
          Encoding::Base64 => {
            let decoded = base64::decode(file.content.as_ref())?;
            writer.write_all(&decoded).await?;
          },
          Encoding::Base64Gzip => {
            let decoded = base64::decode(file.content.as_ref())?;
            let mut decompresser = GzipDecoder::new(writer);
            decompresser.write_all(&decoded).await?;
            decompresser.flush().await?;
            writer = decompresser.into_inner();
          },
        }

        writer.flush().await?;

        Ok::<_, anyhow::Error>(())
      };

      if let Err(e) = write_file.await {
        error!("Failed to write file {:?}: {:?}", path, e);
      }
    })
    .await;
}
