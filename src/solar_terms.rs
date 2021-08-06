use sowngwala::time::{ Month, Date, add_date };

use crate::language::{ Language, LanguageTrait };
use crate::utils::longitude_of_the_sun_from_date;

#[derive(Debug)]
pub struct SolarTerm {
    pub id: u8,
    pub name: Language,
    pub angle: u16,
}

impl LanguageTrait for SolarTerm {
    fn name(&self) -> Box<Language> {
        Box::new(self.name.clone())
    }
}

#[allow(clippy::many_single_char_names)]
pub fn get_last_term(date: &Date) -> (f64, Date) {
    let lng_0: f64 = longitude_of_the_sun_from_date(&date);
    // For the unit of 15, we want the last term.
    // Ex.
    //   317.435511 --> 315.0
    let target = (lng_0 / 15.0).abs().floor() * 15.0;

    let mut d = Date {
        year: date.year,
        month: date.month,
        day: date.day,
    };

    let mut term: Option<Date> = None;

    // Go back by one day a time.
    while term.is_none() {
        let lng: f64 = longitude_of_the_sun_from_date(&d);
        // See if the target falls in the current date.
        if lng <= target && lng > (target - 1.0) {
            term = Some(d);
        }
        d = add_date(&d, -1.0);
    }
    (target, term.unwrap())
}

/// Given the year, returns the year's Lichun in date.
/// * `date` - &Date
#[allow(clippy::many_single_char_names)]
pub fn get_lichun(year: i16) -> Date {
    let d: Date = Date { year, month: Month::Feb, day: 6.0 };
    let (_lng, lichun) = get_last_term(&d);
    lichun
}
