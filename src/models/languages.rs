use crate::schema::languages;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Language {
    pub id: String, 
    pub name: String, 
    pub lang:  String,
    pub territory: String, 
}


impl Language {
    pub fn new(id: String, name: String, lang: String, territory: String) -> Self {
        Self {
            id,
            name,
            lang,
            territory,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewLanguage {
    pub name: String, 
    pub lang:  String,
    pub territory: String, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteLanguage {
    pub id: Uuid,
    pub status: usize,
}

impl DeleteLanguage {
    pub fn new(id: Uuid, status: usize) -> Self {
        Self { id, status }
    }
}
