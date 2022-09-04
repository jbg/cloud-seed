use crate::source::helpers::get_dmi_id;

#[derive(Debug, Clone, Copy)]
pub struct AmazonSource;

#[async_trait::async_trait]
impl super::Source for AmazonSource {
  async fn try_fetch(&self) -> anyhow::Result<Option<String>> {
    if get_dmi_id("bios_vendor").await?.as_deref() != Some("Amazon EC2") {
      return Ok(None);
    }

    let imds_client = aws_config::imds::Client::builder().build().await?;
    let user_data = imds_client.get("/latest/user-data").await?;
    Ok(Some(user_data))
  }
}
