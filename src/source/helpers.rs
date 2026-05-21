use anyhow::Result;
use tokio::io;

#[tracing::instrument(level = "debug")]
pub async fn get_dmi_id(key: &str) -> Result<Option<String>> {
    match tokio::fs::read_to_string(format!("/sys/devices/virtual/dmi/id/{key}")).await {
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
    use std::{
        io::{Cursor, Read as _},
        sync::LazyLock,
    };

    use anyhow::{anyhow, Context as _};
    use async_compression::tokio::bufread::GzipDecoder;
    use base64::{engine::general_purpose::STANDARD as b64, Engine as _};
    use flate2::read::GzDecoder;
    use futures_util::{pin_mut, stream::StreamExt as _};
    use http_body_util::{BodyExt as _, Empty};
    use hyper::{body::Bytes, StatusCode};
    use hyper_util::{
        client::legacy::{connect::HttpConnector, Client},
        rt::{TokioExecutor, TokioTimer},
    };
    use tokio::io::AsyncReadExt as _;
    use tokio_util::io::StreamReader;

    static CLIENT: LazyLock<Client<HttpConnector, Empty<Bytes>>> = LazyLock::new(|| {
        Client::builder(TokioExecutor::new())
            .pool_timer(TokioTimer::new())
            .build_http()
    });

    let mut builder = hyper::Request::get(url).header("user-agent", "cloud-seed");
    if let Some(headers) = maybe_headers {
        *builder.headers_mut().unwrap() = headers;
    }

    let response = CLIENT.request(builder.body(Empty::new())?).await?;
    let status = response.status();

    if status.is_success() {
        let body_chunks_stream = response
            .into_body()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
            .into_data_stream()
            .peekable();
        pin_mut!(body_chunks_stream);

        let first_chunk = body_chunks_stream
            .as_mut()
            .peek()
            .await
            .context("early EOF")?
            .as_ref()
            .map_err(|_| anyhow!("failed to read response"))?;

        #[derive(Clone, Copy)]
        enum Format {
            Plain,
            Gzip,
            Base64Gzip,
        }

        // Check if the first chunk starts with the gzip magic bytes
        let format = match first_chunk[0..3] {
            [0x1f, 0x8b, _] => Format::Gzip,
            [b'H', b'4', b's'] => Format::Base64Gzip,
            _ => Format::Plain,
        };

        let mut body_reader = StreamReader::new(body_chunks_stream);
        let mut s = String::new();
        match format {
            Format::Gzip => {
                let mut decoder = GzipDecoder::new(body_reader);
                decoder.read_to_string(&mut s).await?;
            }
            Format::Base64Gzip => {
                body_reader.read_to_string(&mut s).await?;
                // TODO: consider ways to do this without buffering + blocking
                let decoded = b64.decode(&s)?;
                let mut decoder = GzDecoder::new(Cursor::new(decoded));
                s.clear();
                decoder.read_to_string(&mut s)?;
            }
            Format::Plain => {
                body_reader.read_to_string(&mut s).await?;
            }
        }
        Ok(Some(s))
    } else if status == StatusCode::NOT_FOUND {
        Ok(None)
    } else {
        Err(anyhow!("HTTP {}", status))
    }
}
