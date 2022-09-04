use anyhow::Result;
use tokio::{
  fs::File,
  io::{AsyncReadExt as _, ErrorKind},
};

#[tracing::instrument(level = "debug")]
pub async fn check_dmi_id<const L: usize>(key: &str, expected_value: &[u8; L]) -> Result<bool> {
  match File::open(format!("/sys/devices/virtual/dmi/id/{}", key)).await {
    Ok(mut file) => {
      let mut buf = [0u8; L];
      file.read_exact(&mut buf).await?;
      Ok(&buf == expected_value)
    },
    Err(e) if e.kind() == ErrorKind::NotFound => Ok(false),
    Err(e) => Err(e.into()),
  }
}

#[cfg(feature = "helper-http")]
#[tracing::instrument(level = "debug")]
pub async fn http_get(
  url: &str,
  maybe_headers: Option<hyper::HeaderMap<hyper::header::HeaderValue>>,
) -> Result<String> {
  use hyper::Client;
  use once_cell::sync::Lazy;

  static CLIENT: Lazy<Client<hyper::client::HttpConnector>> = Lazy::new(Client::new);

  let mut builder = hyper::Request::get(url).header("user-agent", "cloud-seed");
  if let Some(headers) = maybe_headers {
    *builder.headers_mut().unwrap() = headers;
  }
  let response = CLIENT.request(builder.body(hyper::Body::empty())?).await?;
  if response.status().is_success() {
    let body = hyper::body::to_bytes(response.into_body()).await?;
    Ok(std::str::from_utf8(&body)?.to_owned())
  }
  else {
    anyhow::bail!("HTTP {}", response.status());
  }
}
