use anyhow::Context;

use crate::source::helpers::{get_dmi_id, http_get};

#[derive(Debug, Clone, Copy)]
pub struct AlibabaSource;

#[async_trait::async_trait]
impl super::Source for AlibabaSource {
  async fn try_fetch(&self) -> anyhow::Result<Option<String>> {
    if get_dmi_id("product_name").await?.as_deref() != Some("Alibaba Cloud ECS") {
      return Ok(None);
    }

    Ok(Some(
      http_get("http://100.100.100.200/latest/user-data", None)
        .await?
        .context("no user data")?,
    ))
  }
}
