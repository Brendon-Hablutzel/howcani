use cohere_rust::api::chat::{ChatRequest, ChatStreamResponse};
use cohere_rust::Cohere;
use futures::{Stream, StreamExt};

use super::GenerativeModel;

const COHERE_API_BASE_URL: &str = "https://api.cohere.ai";
const COHERE_API_V1: &str = "v1";

pub struct CohereModel {
    client: Cohere,
}

pub struct CohereModelConfig {
    pub api_key: String,
}

impl GenerativeModel for CohereModel {
    type Config = CohereModelConfig;

    fn new(config: CohereModelConfig) -> Self {
        Self {
            client: Cohere::new(
                format!("{COHERE_API_BASE_URL}/{COHERE_API_V1}"),
                config.api_key,
            ),
        }
    }

    async fn generate_response(
        &self,
        system_prompt: &str,
        message: &str,
    ) -> anyhow::Result<impl Stream<Item = anyhow::Result<String>>> {
        let request = ChatRequest {
            message: &message,
            preamble_override: Some(system_prompt.to_owned()),
            ..Default::default()
        };

        let response_receiver = self.client.chat(&request).await?;

        // cohere returns a mpsc receiver, so we need to convert it to a stream
        let stream = futures::stream::unfold(response_receiver, |mut receiver| async {
            receiver.recv().await.map(|msg| (msg, receiver))
        })
        .filter_map(|stream_response| {
            futures::future::ready(
                stream_response
                    .map(|stream_response| match stream_response {
                        ChatStreamResponse::ChatTextGeneration {
                            is_finished: _,
                            text,
                        } => Some(text),
                        _ => None,
                    })
                    .map_err(|err| err.into())
                    .transpose(),
            )
        });

        Ok(stream)
    }
}
