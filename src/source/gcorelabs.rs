use crate::source::helpers::{get_dmi_id, http_get};

#[derive(Debug, Clone, Copy)]
pub struct GcorelabsSource;

#[async_trait::async_trait]
impl super::Source for GcorelabsSource {
  async fn try_fetch(&self) -> anyhow::Result<Option<String>> {
    // TODO DMI string is untested
    if get_dmi_id("product_name").await?.as_deref() != Some("GCore Labs") {
      return Ok(None);
    }

    Ok(Some(
      http_get("http://169.254.169.254/latest/user-data", None).await?,
    ))
  }
}
