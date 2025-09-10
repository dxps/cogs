mod attr_templ_repo;
pub use attr_templ_repo::*;

mod item_templ_repo;
pub use item_templ_repo::*;

mod api;
pub use api::*;

use cogs_shared::{
    app::AppResult,
    domain::model::{
        Id,
        meta::{AttrTemplate, ItemTemplate},
    },
};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct DataMgmt {
    attr_templ_repo: Arc<AttrTemplateRepo>,
    item_templ_repo: Arc<ItemTemplateRepo>,
}

impl DataMgmt {
    //
    pub fn new(
        attr_templ_repo: Arc<AttrTemplateRepo>,
        item_templ_repo: Arc<ItemTemplateRepo>,
    ) -> Self {
        Self {
            attr_templ_repo,
            item_templ_repo,
        }
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

    pub async fn upsert_item_templ(&self, mut item_templ: ItemTemplate) -> AppResult<Id> {
        if item_templ.id.clone().is_zero() {
            item_templ.id = Id::from(crate::domain::model::Id::new().0);
        }
        self.item_templ_repo.upsert(&item_templ).await?;
        Ok(item_templ.id)
    }
}
