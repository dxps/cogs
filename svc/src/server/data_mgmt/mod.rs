mod attr_templ_repo;
pub use attr_templ_repo::*;

mod api;
pub use api::*;

use cogs_shared::{
    app::AppResult,
    domain::model::{Id, meta::AttrTemplate},
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct DataMgmt {
    attr_templ_repo: Arc<AttrTemplateRepo>,
}

impl DataMgmt {
    pub fn new(attr_templ_repo: Arc<AttrTemplateRepo>) -> Self {
        Self { attr_templ_repo }
    }

    pub async fn upsert_attr_templ(&self, mut attr_templ: AttrTemplate) -> AppResult<Id> {
        if attr_templ.id.clone().is_zero() {
            attr_templ.id = Id::from(crate::domain::model::Id::new().0);
        }
        self.attr_templ_repo.upsert(&attr_templ).await?;
        Ok(attr_templ.id)
    }

    pub async fn get_all_attr_templ(&self) -> AppResult<Vec<AttrTemplate>> {
        self.attr_templ_repo.get_all().await
    }

    pub async fn delete_attr_templ(&self, id: Id) -> AppResult<()> {
        self.attr_templ_repo.delete(id).await
    }
}
