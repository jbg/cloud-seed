use anyhow::Result;
use tokio::{fs, io::ErrorKind};

#[tracing::instrument(level = "debug")]
pub async fn get_dmi_id(key: &str) -> Result<Option<String>> {
  match fs::read_to_string(format!("/sys/devices/virtual/dmi/id/{}", key)).await {
    Ok(value) => Ok(Some(value.trim().into())),
    Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
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
