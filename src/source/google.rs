use std::iter;

use anyhow::Context;
use base64::{engine::general_purpose::STANDARD as b64, Engine};
use hyper::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;

use crate::source::helpers::{get_dmi_id, http_get};

#[derive(Debug, Clone, Copy)]
pub struct GoogleSource;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
enum UserDataEncoding {
  Base64,
}

#[async_trait::async_trait]
impl super::Source for GoogleSource {
  async fn try_fetch(&self) -> anyhow::Result<Option<String>> {
    if get_dmi_id("product_name").await?.as_deref() != Some("Google Compute Engine") {
      return Ok(None);
    }

    let headers: HeaderMap = iter::once((
      HeaderName::from_static("metadata-flavor"),
      HeaderValue::from_static("Google"),
    ))
    .collect();

    let encoding = http_get(
      "http://metadata.google.internal/computeMetadata/v1/instance/attributes/user-data-encoding",
      Some(headers.clone()),
    )
    .await?;
    let encoding: Option<UserDataEncoding> = encoding
      .map(|body| serde_json::from_str(&body))
      .transpose()?;

    let mut user_data = http_get(
      "http://metadata.google.internal/computeMetadata/v1/instance/attributes/user-data",
      Some(headers),
    )
    .await?
    .context("no user data")?;
    if encoding == Some(UserDataEncoding::Base64) {
      user_data = String::from_utf8(b64.decode(&user_data)?)?;
    }
    Ok(Some(user_data))
  }
}
