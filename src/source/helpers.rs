use anyhow::Result;
use tokio::io;

#[tracing::instrument(level = "debug")]
pub async fn get_dmi_id(key: &str) -> Result<Option<String>> {
  match tokio::fs::read_to_string(format!("/sys/devices/virtual/dmi/id/{}", key)).await {
    Ok(value) => Ok(Some(value.trim().into())),
    Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
    Err(e) => Err(e.into()),
  }
}

#[cfg(feature = "helper-http")]
#[tracing::instrument(level = "debug")]
pub async fn http_get(
  url: &str,
  maybe_headers: Option<hyper::HeaderMap<hyper::header::HeaderValue>>,
) -> Result<Option<String>> {
  use anyhow::{anyhow, Context};
  use async_compression::tokio::bufread::GzipDecoder;
  use futures_util::{
    pin_mut,
    stream::{StreamExt, TryStreamExt},
  };
  use hyper::{Client, StatusCode};
  use once_cell::sync::Lazy;
  use tokio::io::AsyncReadExt as _;
  use tokio_util::io::StreamReader;

  static CLIENT: Lazy<Client<hyper::client::HttpConnector>> = Lazy::new(Client::new);

  let mut builder = hyper::Request::get(url).header("user-agent", "cloud-seed");
  if let Some(headers) = maybe_headers {
    *builder.headers_mut().unwrap() = headers;
  }

  let response = CLIENT.request(builder.body(hyper::Body::empty())?).await?;
  let status = response.status();

  if status.is_success() {
    let body_chunks_stream = response
      .into_body()
      .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
      .peekable();
    pin_mut!(body_chunks_stream);

    let first_chunk = body_chunks_stream
      .as_mut()
      .peek()
      .await
      .context("early EOF")?
      .as_ref()
      .map_err(|_| anyhow!("failed to read response"))?;

    // Check if the first chunk starts with the gzip magic bytes
    let is_gzipped = first_chunk[0..2] == [0x1f, 0x8b];

    let mut body_reader = StreamReader::new(body_chunks_stream);
    let mut s = String::new();
    if is_gzipped {
      let mut decoder = GzipDecoder::new(body_reader);
      decoder.read_to_string(&mut s).await?;
    }
    else {
      body_reader.read_to_string(&mut s).await?;
    }
    Ok(Some(s))
  }
  else if status == StatusCode::NOT_FOUND {
    Ok(None)
  }
  else {
    Err(anyhow!("HTTP {}", status))
  }
}
