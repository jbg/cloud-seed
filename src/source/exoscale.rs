use crate::source::helpers::{check_dmi_id, http_get};

#[derive(Debug, Clone, Copy)]
pub struct ExoscaleSource;

#[async_trait::async_trait]
impl super::Source for ExoscaleSource {
  async fn try_fetch(&self) -> anyhow::Result<Option<String>> {
    if !check_dmi_id("product_name", b"Exoscale Compute Platform").await? {
      return Ok(None);
    }

    Ok(Some(
      http_get("http://169.254.169.254/latest/user-data", None).await?,
    ))
  }
}
