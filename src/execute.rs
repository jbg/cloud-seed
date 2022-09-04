use anyhow::{Context, Result};
use async_compression::tokio::write::GzipDecoder;
use futures_util::stream::{self, StreamExt as _, TryStreamExt as _};
use nix::unistd::{chown, Group, User};
use tokio::{
  fs,
  io::{AsyncWriteExt as _, BufWriter},
};

use crate::schema::{Encoding, UserData};

#[tracing::instrument(level = "debug")]
pub async fn execute_user_data(user_data: UserData<'_>) -> Result<()> {
  stream::iter(user_data.files)
    .map(Ok::<_, anyhow::Error>)
    .try_for_each_concurrent(None, |file| async move {
      if let Some(parent) = file.path.parent() {
        fs::create_dir_all(parent).await?;
      }

      let mut writer = BufWriter::new(
        fs::OpenOptions::new()
          .create(true)
          .truncate(!file.append)
          .append(file.append)
          .mode(u32::from_str_radix(&file.permissions, 8)?)
          .write(true)
          .read(false)
          .open(&file.path)
          .await?,
      );

      if let Some(owner) = file.owner {
        if let Some((user, group)) = owner.split_once(':') {
          let user = User::from_name(user)?.context("no such user")?;
          let group = Group::from_name(group)?.context("no such group")?;
          chown(file.path.as_ref(), Some(user.uid), Some(group.gid))?;
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
        },
      }

      Ok(())
    })
    .await?;

  Ok(())
}
