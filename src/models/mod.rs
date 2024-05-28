use futures::Stream;
use std::error::Error;
use std::future::Future;

pub mod cohere;

pub trait GenerativeModel {
    type Config;

    fn new(config: Self::Config) -> Self;

    fn generate_response(
        &self,
        system_prompt: &str,
        message: &str,
    ) -> impl Future<
        Output = Result<impl Stream<Item = Result<String, Box<dyn Error>>>, Box<dyn Error>>,
    > + Send;
}
