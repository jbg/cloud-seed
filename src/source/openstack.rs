use anyhow::Context;

use crate::source::helpers::{get_dmi_id, http_get};

#[derive(Debug, Clone, Copy)]
pub struct OpenstackSource;

#[async_trait::async_trait]
impl super::Source for OpenstackSource {
  async fn try_fetch(&self) -> anyhow::Result<Option<String>> {
    if get_dmi_id("sys_vendor").await?.as_deref() != Some("OpenStack Foundation") {
      return Ok(None);
    }

    Ok(Some(
      http_get("http://169.254.169.254/latest/user-data", None)
        .await?
        .context("no user data")?,
    ))
  }
}
