use sowngwala::sun::ecliptic_position_of_the_sun_from_date;
use sowngwala::time::Date;

pub fn longitude_of_the_sun_from_date(date: &Date) -> f64 {
    ecliptic_position_of_the_sun_from_date(&date).lng
}
