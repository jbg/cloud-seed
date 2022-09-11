use anyhow::Context;
use hyper::header::{HeaderName, HeaderValue};

use crate::source::helpers::{get_dmi_id, http_get};

#[derive(Debug, Clone, Copy)]
pub struct OracleSource;

#[async_trait::async_trait]
impl super::Source for OracleSource {
  async fn try_fetch(&self) -> anyhow::Result<Option<String>> {
    if get_dmi_id("chassis_asset_tag").await?.as_deref() != Some("OracleCloud.com") {
      return Ok(None);
    }

    let headers = [(
      HeaderName::from_static("authorization"),
      HeaderValue::from_static("Bearer Oracle"),
    )]
    .into_iter()
    .collect();
    Ok(Some(
      http_get(
        "http://169.254.169.254/opc/v2/instance/metadata/user_data",
        Some(headers),
      )
      .await?
      .context("no user data")?,
    ))
  }
}
