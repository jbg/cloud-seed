use crate::source::helpers::{check_dmi_id, http_get};

#[derive(Debug, Clone, Copy)]
pub struct AlibabaSource;

#[async_trait::async_trait]
impl super::Source for AlibabaSource {
  async fn try_fetch(&self) -> anyhow::Result<Option<String>> {
    if !check_dmi_id("product_name", "Alibaba Cloud ECS").await? {
      return Ok(None);
    }

    Ok(Some(
      http_get("http://100.100.100.200/latest/user-data", None).await?,
    ))
  }
}
