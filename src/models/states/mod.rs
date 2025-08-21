use std::sync::Arc;

use candle_core::{DType, Device};
use candle_transformers::models::{
    clip::{ClipConfig, ClipModel},
    mimi::candle_nn::VarBuilder,
};
use deadpool_diesel::{
    Runtime,
    postgres::{Manager, Pool},
};
use hf_hub::api::tokio::ApiRepo;
use qdrant_client::Qdrant;
use tokenizers::Tokenizer;

const MODEL_NAME: &str = "openai/clip-vit-base-patch32";

#[derive(Clone)]
pub struct AppState {
    device: Arc<Device>,
    diesel_pool: Pool,
    model: Arc<ClipModel>,
    qdrant_client: Arc<Qdrant>,
    tokenizer: Arc<Tokenizer>,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        let api = hf_hub::api::tokio::Api::new()?.model(MODEL_NAME.to_owned());
        let device = Device::cuda_if_available(0)?;

        Ok(Self {
            diesel_pool: Self::diesel_pool_helper()?,
            qdrant_client: Self::qdrant_client_helper()?,
            model: Self::model_helper(&api, &device).await?,
            tokenizer: Self::tokenizer_helper(&api).await?,
            device: Arc::new(device),
        })
    }

    fn diesel_pool_helper() -> anyhow::Result<Pool> {
        Ok(Pool::builder(Manager::new(
            std::env::var("DATABASE_URL")?,
            Runtime::Tokio1,
        ))
        .build()?)
    }

    fn qdrant_client_helper() -> anyhow::Result<Arc<Qdrant>> {
        Ok(Arc::new(
            Qdrant::from_url(&std::env::var("QDRANT_URL")?).build()?,
        ))
    }

    async fn model_helper(api: &ApiRepo, device: &Device) -> anyhow::Result<Arc<ClipModel>> {
        let model_path = api.get("pytorch_model.bin").await?;
        let vb = VarBuilder::from_pth(model_path, DType::F32, device)?;
        let config = ClipConfig::vit_base_patch32();

        Ok(Arc::new(ClipModel::new(vb, &config)?))
    }

    async fn tokenizer_helper(api: &ApiRepo) -> anyhow::Result<Arc<Tokenizer>> {
        let tokenizer_file = api.get("tokenizer.json").await?;
        Ok(Arc::new(
            Tokenizer::from_file(tokenizer_file).map_err(anyhow::Error::msg)?,
        ))
    }

    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn diesel_pool(&self) -> &Pool {
        &self.diesel_pool
    }

    pub fn model(&self) -> &ClipModel {
        &self.model
    }

    pub fn qdrant_client(&self) -> &Qdrant {
        &self.qdrant_client
    }

    pub fn tokenizer(&self) -> &Tokenizer {
        &self.tokenizer
    }
}
