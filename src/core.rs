#[cfg(test)]
extern crate approx_eq;

use sowngwala::time::{
    Date,
    Month,
    julian_day,
};

pub fn foo() -> f64 {
    let date = Date {
        year: 1985,
        month: Month::Feb,
        day: 17.25,
    };
    julian_day(&date)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx_eq::assert_approx_eq;

    #[test]
    fn foo_works() {
        assert_approx_eq!(
            foo(),
            2_446_113.75,
            1e-3
        );
    }
}
