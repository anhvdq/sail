use datafusion_common::Result;
use serde::{Deserialize, Serialize};

use crate::manager::utils::match_pattern;
use crate::manager::CatalogManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogMetadata {
    pub name: String,
    pub description: Option<String>,
}

impl CatalogMetadata {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None, // Spark code sets all descriptions to None
        }
    }
}

impl CatalogManager<'_> {
    pub fn default_catalog(&self) -> Result<String> {
        let state = self.ctx.state_ref();
        let state = state.read();
        Ok(state.config().options().catalog.default_catalog.clone())
    }

    pub fn set_default_catalog(&self, catalog_name: String) -> Result<()> {
        let state = self.ctx.state_ref();
        let mut state = state.write();
        state.config_mut().options_mut().catalog.default_catalog = catalog_name;
        Ok(())
    }

    pub fn list_catalogs(&self, catalog_pattern: Option<&str>) -> Result<Vec<CatalogMetadata>> {
        let state = self.ctx.state_ref();
        let state = state.read();
        Ok(state
            .catalog_list()
            .catalog_names()
            .into_iter()
            .filter(|name| match_pattern(name.as_str(), catalog_pattern))
            .map(CatalogMetadata::new)
            .collect::<Vec<_>>())
    }
}
