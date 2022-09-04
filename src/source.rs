pub mod helpers;

#[cfg(feature = "source-alibaba")]
pub mod alibaba;
#[cfg(feature = "source-amazon")]
pub mod amazon;
#[cfg(feature = "source-exoscale")]
pub mod exoscale;
#[cfg(feature = "source-gcorelabs")]
pub mod gcorelabs;
#[cfg(feature = "source-google")]
pub mod google;
#[cfg(feature = "source-vultr")]
pub mod vultr;

#[async_trait::async_trait]
pub trait Source {
  async fn try_fetch(&self) -> anyhow::Result<Option<String>>;
}
