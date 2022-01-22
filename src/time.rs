#[cfg(test)]
extern crate approx_eq;

use sowngwala::time::eot_fortified_ut_from_local;
pub use sowngwala::time::{Date, DateTime, Month, Time};

pub fn ut_from_local(&dt: &DateTime, zone: i8) -> DateTime {
    eot_fortified_ut_from_local(&dt, zone)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx_eq::assert_approx_eq;

    #[test]
    fn ut_from_local_works() {
        let zone: i8 = 9;

        let local = DateTime {
            year: 2021,
            month: Month::Jul,
            day: 7.0,
            hour: 0,
            min: 0,
            sec: 0.0,
        };

        let ut: DateTime = ut_from_local(&local, zone);
        println!("ut: {:?}", ut);

        assert_eq!(ut.year, 2021);
        assert_eq!(ut.month, Month::Jul);
        assert_eq!(ut.day, 6.0);
        assert_eq!(ut.hour, 14);
        assert_eq!(ut.min, 57);
        assert_approx_eq!(
            ut.sec, // 17.137797474861145,
            17.13779, 1e-6
        );
    }
}
