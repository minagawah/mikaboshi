//! A module for "二十四节气" (Er-Shi-Si Jie-Qi).
//! Or, for calculating "立春" (Li-Chun).
use serde::{Deserialize, Serialize};
use sowngwala::time::{add_date, Date, Month};

use crate::language::{Language, LanguageData, LanguageTrait, NameDataTrait};
use crate::utils::{get_json, longitude_of_the_sun_from_date};

#[derive(Debug)]
pub struct SolarTerm {
    pub id: u8,
    pub name: Language,
    pub angle: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarTermRawData {
    pub id: u8,
    pub name: LanguageData,
    pub angle: u16,
}

impl LanguageTrait for SolarTerm {
    fn name(&self) -> Box<Language> {
        Box::new(self.name.clone())
    }
}

impl NameDataTrait for SolarTermRawData {
    fn name(&self) -> Box<LanguageData> {
        Box::new(self.name.clone())
    }
}

lazy_static! {
    pub static ref SOLAR_TERMS: Vec<SolarTerm> = {
        let json = &include_str!("../json/solar_terms.json");
        let data: Vec<SolarTermRawData> = get_json::<SolarTermRawData>(json);
        data.iter()
            .map(|item| {
                let item = item.clone();
                SolarTerm {
                    id: item.id,
                    name: item.language_from_data(),
                    angle: item.angle,
                }
            })
            .collect()
    };
}

#[allow(clippy::many_single_char_names)]
pub fn get_last_term(date: &Date) -> (f64, Date) {
    let lng_0: f64 = longitude_of_the_sun_from_date(date);
    // For the unit of 15, we want the last term.
    // Ex.
    //   317.435511 --> 315.0
    let target = (lng_0 / 15.0).abs().floor() * 15.0;

    let mut next = Date {
        year: date.year,
        month: date.month,
        day: date.day,
    };

    let mut prev: Option<Date> = None;
    let mut term: Option<Date> = None;

    // Go back by one day a time.
    while term.is_none() {
        let lng: f64 = longitude_of_the_sun_from_date(&next);
        // See if the target falls in the current date.
        if lng <= target && lng > (target - 1.0) {
            term = prev;
        } else {
            prev = Some(next);
            next = add_date(&next, -1.0);
        }
    }
    (target, term.unwrap())
}

/// Example:
/// ```rust
/// use mikaboshi::solar_terms::get_lichun;
/// use wasm_bindgen::prelude::*;
///
/// #[wasm_bindgen]
/// pub fn xx(year: i16) -> JsValue {
///     let lichun = get_lichun(year);
///     JsValue::from_str(&format!(
///         "{:04}-{:02}-{:02}",
///         lichun.year as u16, lichun.month as u8, lichun.day as u8
///     ))
/// }
/// ```
#[allow(clippy::many_single_char_names)]
pub fn get_lichun(year: i16) -> Date {
    let d: Date = Date {
        year,
        month: Month::Feb,
        day: 6.0,
    };
    let (_lng, lichun) = get_last_term(&d);
    lichun
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_last_term() {
        let date = Date {
            year: 2022,
            month: Month::Feb,
            day: 6.0,
        };

        let (_lng, term) = get_last_term(&date);

        assert_eq!(term.year, 2022);
        assert_eq!(term.month, Month::Feb);
        assert_eq!(term.day, 4.0);
    }
}
