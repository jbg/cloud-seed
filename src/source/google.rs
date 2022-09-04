use anyhow::bail;
use serde::Deserialize;

use crate::source::helpers::{get_dmi_id, http_get};

#[derive(Debug, Clone, Copy)]
pub struct GoogleSource;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
struct InstanceAttributes {
  user_data: Option<String>,
  user_data_encoding: Option<UserDataEncoding>,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
enum UserDataEncoding {
  Base64,
}

#[async_trait::async_trait]
impl super::Source for GoogleSource {
  async fn try_fetch(&self) -> anyhow::Result<Option<String>> {
    if get_dmi_id("bios_vendor").await?.as_deref() != Some("Google Compute Engine") {
      return Ok(None);
    }

    let body = http_get(
      "http://metadata.google.internal/computeMetadata/v1/instance/attributes",
      None,
    )
    .await?;
    let instance_attributes: InstanceAttributes = serde_json::from_str(&body)?;
    if let Some(mut user_data) = instance_attributes.user_data {
      if instance_attributes.user_data_encoding == Some(UserDataEncoding::Base64) {
        user_data = String::from_utf8(base64::decode(user_data.as_bytes())?)?;
      }
      Ok(Some(user_data))
    }
    else {
      bail!("No user data in instance attributes");
    }
  }
}
