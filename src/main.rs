mod execute;
mod schema;
mod source;

use anyhow::{bail, Result};
use source::Source;
use tracing::{debug, info};

use crate::execute::execute_user_data;

const SOURCES: &[&dyn Source] = &[
  #[cfg(feature = "source-alibaba")]
  &source::alibaba::AlibabaSource,
  #[cfg(feature = "source-amazon")]
  &source::amazon::AmazonSource,
  #[cfg(feature = "source-exoscale")]
  &source::exoscale::ExoscaleSource,
  #[cfg(feature = "source-gcorelabs")]
  &source::gcorelabs::GcorelabsSource,
  #[cfg(feature = "source-google")]
  &source::google::GoogleSource,
  #[cfg(feature = "source-vultr")]
  &source::vultr::VultrSource,
];

const ALLOWED_SHEBANGS: [&str; 2] = [
  "#cloud-seed",
  // Compatibility with cloud-init's write_files
  "#cloud-config",
];

#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();

  for source in SOURCES {
    debug!("Trying {:?}", source);
    if let Some(user_data) = source.try_fetch().await? {
      info!("Got user data from {:?}", source);
      if let Some((shebang, content)) = user_data.split_once('\n') {
        if ALLOWED_SHEBANGS.contains(&shebang.trim()) {
          let user_data = serde_yaml::from_str(content)?;
          info!("Executing user data from {:?}", source);
          return execute_user_data(user_data).await;
        }
        else {
          bail!("Unhandled shebang in user data");
        }
      }
      else {
        bail!("Malformed user data");
      }
    }
  }

  bail!("No suitable source found");
}
