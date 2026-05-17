use std::sync::Arc;

use crate::{
    server::{AccessLevelRepo, AttrTemplateRepo, ItemTemplateRepo},
    utils::new_id,
};
use cogs_shared::{
    app::AppResult,
    domain::model::{
        AccessLevel, Id,
        meta::{AttrTemplate, ItemTemplate},
    },
};

#[derive(Clone, Debug)]
pub struct DataMgmt {
    attr_templ_repo: Arc<AttrTemplateRepo>,
    item_templ_repo: Arc<ItemTemplateRepo>,
    access_level_repo: Arc<AccessLevelRepo>,
}

impl DataMgmt {
    //
    pub fn new(
        attr_templ_repo: Arc<AttrTemplateRepo>,
        item_templ_repo: Arc<ItemTemplateRepo>,
        access_level_repo: Arc<AccessLevelRepo>,
    ) -> Self {
        Self {
            attr_templ_repo,
            item_templ_repo,
            access_level_repo,
        }
    }

    // -------------------
    // Attribute Templates
    // -------------------

    pub async fn upsert_attr_template(&self, mut attr_templ: AttrTemplate) -> AppResult<Id> {
        if attr_templ.id.clone().is_zero() {
            attr_templ.id = new_id();
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
            item_templ.id = new_id();
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

    // -------------
    // Access Levels
    // -------------

    pub async fn upsert_access_level(&self, mut access_level: AccessLevel) -> AppResult<Id> {
        if access_level.id.is_zero() {
            access_level.id = new_id();
            self.access_level_repo.insert(&access_level).await?;
        } else {
            self.access_level_repo.update(&access_level).await?;
        }

        Ok(access_level.id)
    }

    pub async fn get_all_access_levels(&self) -> AppResult<Vec<AccessLevel>> {
        self.access_level_repo.get_all().await
    }

    pub async fn delete_access_level(&self, id: Id) -> AppResult<()> {
        self.access_level_repo.delete(id).await
    }
}
