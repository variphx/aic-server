use qdrant_client::{
    Qdrant,
    qdrant::{PrefetchQueryBuilder, Query, QueryPointsBuilder},
};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    constants::{
        QDRANT_KEYFRAME_COLLECTION_IMAGE_VECTOR_NAME, QDRANT_KEYFRAME_COLLECTION_NAME,
        QDRANT_KEYFRAME_COLLECTION_OBJECT_VECTOR_NAME,
    },
    models::{dtos::vectors::keyframes::VectorizedKeyframeDto, states::AppState},
    services::embeddings::texts::TextEmbeddingService,
};

#[derive(Clone, Copy)]
pub struct VectorizedKeyframeService<'a> {
    client: &'a Qdrant,
}

impl<'a> From<&'a AppState> for VectorizedKeyframeService<'a> {
    fn from(value: &'a AppState) -> Self {
        Self {
            client: value.qdrant_client(),
        }
    }
}

impl<'a> VectorizedKeyframeService<'a> {
    async fn embed_text(&self, text: &str) -> anyhow::Result<Vec<f32>> {
        let mut embedding_service = TextEmbeddingService::new().await?;
        embedding_service.embed_text(text).await
    }

    pub async fn find_nearest_top_k_by_text(
        &self,
        text: &str,
        top_k: u64,
    ) -> anyhow::Result<Vec<VectorizedKeyframeDto>> {
        let embeddings = self.embed_text(text).await?;

        let query_result = self
            .client
            .query(
                QueryPointsBuilder::new(QDRANT_KEYFRAME_COLLECTION_NAME)
                    .add_prefetch(
                        PrefetchQueryBuilder::default()
                            .query(Query::new_nearest(embeddings.clone()))
                            .using(QDRANT_KEYFRAME_COLLECTION_IMAGE_VECTOR_NAME)
                            .limit(100u64),
                    )
                    .query(Query::new_nearest(embeddings))
                    .using(QDRANT_KEYFRAME_COLLECTION_OBJECT_VECTOR_NAME)
                    .limit(top_k),
            )
            .await?
            .result;

        query_result
            .into_par_iter()
            .map(VectorizedKeyframeDto::try_from)
            .collect::<Result<_, _>>()
    }
}
