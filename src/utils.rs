use sowngwala::time::Date;
use sowngwala::sun::ecliptic_position_of_the_sun_from_date;

pub fn longitude_of_the_sun_from_date(date: &Date) -> f64 {
    ecliptic_position_of_the_sun_from_date(&date).lng
}
