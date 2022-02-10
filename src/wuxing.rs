use serde::{Deserialize, Serialize};

use crate::language::{Language, LanguageData, LanguageTrait, NameDataTrait};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WuXing {
    pub no: u8,
    pub name: Language,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WuXingData {
    pub no: u8,
    pub name: LanguageData,
}

impl LanguageTrait for WuXing {
    fn name(&self) -> Box<Language> {
        Box::new(self.name.clone())
    }
}

impl NameDataTrait for WuXingData {
    fn name(&self) -> Box<LanguageData> {
        Box::new(self.name.clone())
    }
}
