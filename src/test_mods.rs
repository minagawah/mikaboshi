/**
 * Modules used ONLY from tests.
 */
use serde::{Deserialize, Serialize};
use sowngwala::time::{Date, DateTime, Month};
use std::convert::{From, TryFrom};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct DateParams {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl From<&DateParams> for Date {
    fn from(&params: &DateParams) -> Self {
        Date {
            year: params.year as i16,
            month: Month::try_from(params.month as i32).unwrap(),
            day: params.day as f64,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct DateTimeParams {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub zone: i8,
}

impl From<&DateTimeParams> for DateTime {
    fn from(&params: &DateTimeParams) -> Self {
        DateTime {
            year: params.year as i16,
            month: Month::try_from(params.month as i32).unwrap(),
            day: params.day as f64,
            hour: params.hour as i16,
            min: params.min as i16,
            sec: params.sec as f64,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XiaGuaTuParams {
    pub unpan_xing_center: usize,
    pub unpan_xing_order: Vec<usize>,
    pub xiang_xing_direction: String,
    pub xiang_xing_sector: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShengSiParams {
    pub unpan_id: usize,
    pub unpan_xing_chart: Vec<usize>,
}
