use serde::{Deserialize, Serialize};

use crate::language::{Language, LanguageTrait};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bagua {
    pub num: u8,
    pub name: Language,
    pub direction: String,
    pub element: u8,
}

impl LanguageTrait for Bagua {
    fn name(&self) -> Box<Language> {
        Box::new(self.name.clone())
    }
}
