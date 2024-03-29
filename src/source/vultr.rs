use std::iter;

use anyhow::Context;
use hyper::header::{HeaderName, HeaderValue};
use serde::Deserialize;

use crate::source::helpers::{get_dmi_id, http_get};

#[derive(Debug, Clone, Copy)]
pub struct VultrSource;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct Metadata {
  user_data: Option<String>,
}

#[async_trait::async_trait]
impl super::Source for VultrSource {
  async fn try_fetch(&self) -> anyhow::Result<Option<String>> {
    if get_dmi_id("bios_vendor").await?.as_deref() != Some("Vultr") {
      return Ok(None);
    }

    let headers = iter::once((
      HeaderName::from_static("metadata-token"),
      HeaderValue::from_static("cloudinit"),
    ))
    .collect();
    let body = http_get("http://169.254.169.254/v1.json", Some(headers))
      .await?
      .context("no user data")?;
    let metadata: Metadata = serde_json::from_str(&body)?;
    Ok(metadata.user_data)
  }
}
