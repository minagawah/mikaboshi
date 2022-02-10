use serde::{Deserialize, Serialize};

use crate::language::{Language, LanguageTrait};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiuXing {
    pub num: u8,
    pub name: Language,
    pub color: String,
    pub element: u8,
    pub planet: u8,
}

impl LanguageTrait for JiuXing {
    fn name(&self) -> Box<Language> {
        Box::new(self.name.clone())
    }
}
