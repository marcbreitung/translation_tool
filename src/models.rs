use crate::schema::translations;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Translation {
    pub id: String,
    pub key: String,
    pub target: String,
    pub language: String,
}

impl Translation {
    pub fn new(id: String, key: String, target: String, language: String) -> Self {
        Self {
            id,
            key,
            target,
            language,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewTranslation {
    pub key: String,
    pub target: String,
    pub language: String,
}
