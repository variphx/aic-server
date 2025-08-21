use candle_core::{Device, Tensor};
use candle_transformers::models::clip::ClipModel;
use qdrant_client::{Qdrant, qdrant::SearchPointsBuilder};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use tokenizers::Tokenizer;

use crate::models::{dtos::vectors::keyframes::VectorizedKeyframeDto, states::AppState};

#[derive(Clone, Copy)]
pub struct VectorizedKeyframeService<'a> {
    client: &'a Qdrant,
    device: &'a Device,
    model: &'a ClipModel,
    tokenizer: &'a Tokenizer,
}

impl<'a> From<&'a AppState> for VectorizedKeyframeService<'a> {
    fn from(value: &'a AppState) -> Self {
        Self {
            model: value.model(),
            tokenizer: value.tokenizer(),
            device: value.device(),
            client: value.qdrant_client(),
        }
    }
}

impl<'a> VectorizedKeyframeService<'a> {
    fn embed_text(&self, text: &str) -> anyhow::Result<Tensor> {
        let input_ids = {
            let input_ids = self
                .tokenizer
                .encode_fast(text, true)
                .map_err(anyhow::Error::msg)?;
            Tensor::new(input_ids.get_ids(), self.device)?
        };

        Ok(self
            .model
            .get_text_features(&input_ids.unsqueeze(0)?)
            .map(|x| x.squeeze(0))??)
    }

    pub async fn find_nearest_top_k_by_text(
        &self,
        text: &str,
        top_k: u64,
    ) -> anyhow::Result<Vec<VectorizedKeyframeDto>> {
        let embeddings = self.embed_text(text)?;

        let query_result = self
            .client
            .search_points(SearchPointsBuilder::new(
                "keyframes",
                embeddings.squeeze(0)?.to_vec1::<f32>()?,
                top_k,
            ))
            .await?
            .result;

        query_result
            .into_par_iter()
            .map(VectorizedKeyframeDto::try_from)
            .collect::<Result<_, _>>()
    }
}
