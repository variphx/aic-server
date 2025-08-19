use deadpool_diesel::postgres::Pool;

use crate::{
    dtos::keyframes::KeyframeDto, repositories::keyframes::KeyframeRepository, states::AppState,
};

#[derive(Clone, Copy)]
pub struct KeyframeService<'a> {
    pool: &'a Pool,
}

impl<'a> From<&'a AppState> for KeyframeService<'a> {
    fn from(value: &'a AppState) -> Self {
        Self { pool: value.pool() }
    }
}

impl<'a> KeyframeService<'a> {
    pub const fn pool(&self) -> &'a Pool {
        self.pool
    }
}

impl<'a> KeyframeService<'a> {
    pub async fn find_by_id(&self, id: i64) -> anyhow::Result<Option<KeyframeDto>> {
        let repository = KeyframeRepository::from(self);
        Ok(repository
            .find_by_id_and_join_video(id)
            .await?
            .map(Into::into))
    }
}
