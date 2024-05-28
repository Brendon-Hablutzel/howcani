use futures::Stream;
use std::future::Future;

pub mod cohere;

pub trait GenerativeModel {
    type Config;

    fn new(config: Self::Config) -> Self;

    fn generate_response(
        &self,
        system_prompt: &str,
        message: &str,
    ) -> impl Future<Output = anyhow::Result<impl Stream<Item = anyhow::Result<String>>>> + Send;
}
