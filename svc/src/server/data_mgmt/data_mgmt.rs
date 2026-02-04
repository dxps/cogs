use std::sync::Arc;

use crate::server::{AttrTemplateRepo, ItemTemplateRepo};
use cogs_shared::{
    app::AppResult,
    domain::model::{
        Id,
        meta::{AttrTemplate, ItemTemplate},
    },
};

#[derive(Clone, Debug)]
pub struct DataMgmt {
    attr_templ_repo: Arc<AttrTemplateRepo>,
    item_templ_repo: Arc<ItemTemplateRepo>,
}

impl DataMgmt {
    //
    pub fn new(attr_templ_repo: Arc<AttrTemplateRepo>, item_templ_repo: Arc<ItemTemplateRepo>) -> Self {
        Self {
            attr_templ_repo,
            item_templ_repo,
        }
    }

    // -------------------
    // Attribute Templates
    // -------------------

    pub async fn upsert_attr_template(&self, mut attr_templ: AttrTemplate) -> AppResult<Id> {
        if attr_templ.id.clone().is_zero() {
            attr_templ.id = Id::default();
        }
        self.attr_templ_repo.upsert(&attr_templ).await?;
        Ok(attr_templ.id)
    }

    pub async fn get_all_attr_templates(&self) -> AppResult<Vec<AttrTemplate>> {
        self.attr_templ_repo.get_all().await
    }

    pub async fn delete_attr_template(&self, id: Id) -> AppResult<()> {
        self.attr_templ_repo.delete(id).await
    }

    // --------------
    // Item Templates
    // --------------

    pub async fn upsert_item_template(&self, mut item_templ: ItemTemplate) -> AppResult<Id> {
        if item_templ.id.clone().is_zero() {
            item_templ.id = Id::default();
        }
        self.item_templ_repo.upsert(&item_templ).await?;
        Ok(item_templ.id)
    }

    pub async fn get_all_item_templates(&self) -> AppResult<Vec<ItemTemplate>> {
        self.item_templ_repo.get_all().await
    }

    pub async fn delete_item_template(&self, id: Id) -> AppResult<()> {
        self.item_templ_repo.delete(id).await
    }
}
