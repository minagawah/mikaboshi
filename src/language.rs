use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDetails {
    pub alphabet: String, // Ex. "甲"
    pub phonetic: String, // Ex. "jiǎ"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub en: String,
    pub ja: LanguageDetails,
    pub vi: LanguageDetails,
    pub zh_cn: LanguageDetails,
    pub zh_tw: LanguageDetails,
}

impl LanguageDetails {
    pub fn new(alphabet: &str, phonetic: &str) -> Self {
        LanguageDetails {
            alphabet: alphabet.to_string(),
            phonetic: phonetic.to_string(),
        }
    }
}

pub trait LanguageTrait {
    fn name(&self) -> Box<Language>;

    /// Chinese character in Taiwanese
    fn alphabet(&self) -> String {
        self.name().zh_tw.alphabet
    }

    /// Chinese phonetic in Taiwanese
    fn phonetic(&self) -> String {
        self.name().zh_tw.phonetic
    }

    /// Japanese character
    fn alphabet_ja(&self) -> String {
        self.name().ja.alphabet
    }
}
