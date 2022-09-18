use std::{borrow::Cow, path::Path};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserData<'a> {
  #[serde(default, borrow, alias = "fqdn")]
  pub hostname: Option<Cow<'a, str>>,
  #[serde(default, borrow, alias = "write_files")]
  pub files: Vec<File<'a>>,
}

fn default_permissions() -> Cow<'static, str> {
  "0644".into()
}

#[derive(Deserialize, Debug)]
pub struct File<'a> {
  #[serde(borrow)]
  pub path: Cow<'a, Path>,
  #[serde(borrow, default)]
  pub content: Cow<'a, str>,
  #[serde(borrow, default)]
  pub owner: Option<Cow<'a, str>>,
  #[serde(borrow, default = "default_permissions")]
  pub permissions: Cow<'a, str>,
  #[serde(default)]
  pub encoding: Encoding,
  #[serde(default)]
  pub append: bool,
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
pub enum Encoding {
  #[serde(alias = "text/plain")]
  #[default]
  Plain,
  #[serde(alias = "b64")]
  Base64,
  #[serde(alias = "gz+base64")]
  #[serde(alias = "gzip+base64")]
  #[serde(alias = "gz+b64")]
  #[serde(alias = "gzip+b64")]
  Base64Gzip,
}
