#[cfg(test)]
use sowngwala::time::Month;

use serde::{ Deserialize, Serialize };
use sowngwala::time::{
    Date,
    Time,
    DateTime,
    julian_day,
    julian_day_from_ut,
    modified_julian_day_from_ut,
};

use crate::constants::{
    HOUR_STEM_TABLE,
    GANZHI_SEXAGESIMAL,
    STEMS,
    BRANCHES,
};
use crate::language::{ Language, LanguageTrait };
use crate::solar_terms::get_lichun;
use crate::time::ut_from_local;
use crate::utils::longitude_of_the_sun_from_date;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stem {
    pub no: u8,
    pub name: Language,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub no: u8,
    pub name: Language,
}

#[derive(Debug)]
pub struct GanZhi<'a> {
    pub stem: &'a Stem,
    pub branch: &'a Branch,
}

#[derive(Debug)]
pub struct Bazi<'a> {
    pub year: GanZhi<'a>,
    pub month: GanZhi<'a>,
    pub day: GanZhi<'a>,
    pub hour: GanZhi<'a>,
}

impl LanguageTrait for Stem {
    fn name(&self) -> Box<Language> {
        Box::new(self.name.clone())
    }
}

impl LanguageTrait for Branch {
    fn name(&self) -> Box<Language> {
        Box::new(self.name.clone())
    }
}

impl GanZhi<'_> {
    /// Concatenate Stem & Branch (for Chinese characters)
    #[allow(dead_code)]
    fn alphabet(&self) -> String {
        format!(
            "{}{}",
            self.stem.alphabet(),
            self.branch.alphabet()
        )
    }

    /// Concatenate Stem & Branch (for Chinese phonetics)
    #[allow(dead_code)]
    fn phonetic(&self) -> String {
        format!(
            "{} {}",
            self.stem.phonetic(),
            self.branch.phonetic()
        )
    }

    /// Concatenate Stem & Branch (for Japanese characters)
    #[allow(dead_code)]
    fn alphabet_ja(&self) -> String {
        format!(
            "{}・{}",
            self.stem.alphabet_ja(),
            self.branch.alphabet_ja()
        )
    }
}

impl<'a> Bazi<'a> {
    fn new(
        year: GanZhi<'a>,
        month: GanZhi<'a>,
        day: GanZhi<'a>,
        hour: GanZhi<'a>
    ) -> Self {
        Bazi {
            year,
            month,
            day,
            hour,
        }
    }

    pub fn from_local(lt: &DateTime, zone: i8) -> Bazi {
        let ut = ut_from_local(&lt, zone);
        println!("ut: {:?}", ut);

        let year = _year_ganzhi(Box::new(ut));
        let month = _month_ganzhi(Box::new(ut), year.stem.no);
        let day = _day_ganzhi(Box::new(ut));
        let hour = _hour_ganzhi(Box::new(Time::from(lt)), day.stem.no);
        Bazi::new(year, month, day, hour)
    }

    pub fn from_ut(ut: &DateTime, t: &Time) -> Bazi<'a> {
        let year = _year_ganzhi(Box::new(*ut));
        let month = _month_ganzhi(Box::new(*ut), year.stem.no);
        let day = _day_ganzhi(Box::new(*ut));
        let hour = _hour_ganzhi(Box::new(*t), day.stem.no);
        Bazi::new(year, month, day, hour)
    }
}

/// Year Ganzhi
fn _year_ganzhi(ut: Box<DateTime>) -> GanZhi<'static> {
    // Year Stem and Branch are easily found.
    // However, we must watch out if it is before
    // or after Lichun. The year begins from Lichun,
    // and it belongs to last year if the date
    // is before Lichun.
    let lichun = get_lichun(ut.year);

    let year = if julian_day_from_ut(&ut) < julian_day(&lichun) {
        ut.year - 1
    } else {
        ut.year
    };

    // Stem is found from the last digit of the year.
    // 0   1   2   3   4   5   6   7   8  9
    // 庚  辛  壬  癸  甲  乙  丙  丁  戊  己
    let digits: Vec<_> = year
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let last = *digits.last().unwrap();

    // Brach is found simply if we know the year,
    // but once again, it depends on Lichun.
    // 0   1   2   3   4   5   6   7   8  9   10  11
    // 申  酉  戌  亥  子  丑  寅  卯  辰  巳  午  未
    GanZhi {
        stem: &STEMS[((last + 6) % 10) as usize],
        branch: &BRANCHES[((year + 8) % 12) as usize],
    }
}

/// Month Ganzhi
#[allow(clippy::boxed_local)]
fn _month_ganzhi(ut: Box<DateTime>, year_stem_no: u8) -> GanZhi<'static> {
    let lng: f64 = longitude_of_the_sun_from_date(&Date::from(&*ut));

    // Branch is easily found by looking at the longitude of the sun.
    let branch_index: usize = if (315.0..345.0).contains(&lng) {
        0 // 立春 (lichun) + 雨水 (yushui) ---> 寅 (yin)
    } else if !(15.0..345.0).contains(&lng) {
        1 // 啓蟄 (jingzhe) + 春分 (chunfen) ---> 卯 (mao)
    } else if (15.0..45.0).contains(&lng) {
        2 // 清明 (qingming) + 穀雨 (guyu) ---> 辰 (chen)
    } else if (45.0..75.0).contains(&lng) {
        3 // 立夏 (lixia) + 小滿 (xiaoman) ---> 巳 (si)
    } else if (75.0..105.0).contains(&lng) {
        4 // 芒種 (mangzhong) + 夏至 (xiazhi) ---> 午 (wu)
    } else if (105.0..135.0).contains(&lng) {
        5 // 小暑 (xiaoshu) + 大暑 (dashu) ---> 未 (wei)
    } else if (135.0..165.0).contains(&lng) {
        6 // 立秋 (liqiu) + 處暑 (chushu) ---> 申 (shen)
    } else if (165.0..195.0).contains(&lng) {
        7 // 白露 (bailu) + 秋分 (qiufen) ---> 酉 (you)
    } else if (195.0..225.0).contains(&lng) {
        8 // 寒露 (hanlu) + 霜降 (shuangjiang) ---> 戌 (xu)
    } else if (225.0..255.0).contains(&lng) {
        9 // 立冬 (lidong) + 小雪 (xiaoxue) ---> 亥 (hai)
    } else if (255.0..285.0).contains(&lng) {
        10 // 大雪 (daxue) + 冬至 (dongzhi) ---> 子 (zi)
    } else {
        // lng >= 285.0 || lng < 315.0
        11 // 小寒 (xiaohan) + 大寒 (dahan) ---> 丑 (chou)
    };

    // Stem is found using the Year Stem.
    // For a given year, you can find the first Month Stem.
    // Once you find the first Month Stem,
    // you simply count up to the current month.
    // This is done by adding 'branch_id' because 'branch_id'
    // is nothing but how many month from the beginning (Lichun).
    let stem_index: usize = if year_stem_no == 1 || year_stem_no == 6 {
        2 // 甲(jia:1) or 己(ji:6) ---> 丙(bing:3)
    } else if year_stem_no == 2 || year_stem_no == 7 {
        4 // 乙(yi:2) or 庚(geng:7) ---> 戊(wu:5)
    } else if year_stem_no == 3 || year_stem_no == 8 {
        6 // 丙(bing:3) or 辛(xin:8) ---> 庚(geng:7)
    } else if year_stem_no == 4 || year_stem_no == 9 {
        8 // 丁(ding:4) or 壬(ren:9) ---> 壬(ren:9)
    } else {
        0 // 戊(wu:5) or 癸(gui:10) ---> 甲(jia:1)
    };

    GanZhi {
        stem: &STEMS[(stem_index + branch_index) % 10],
        branch: &BRANCHES[(branch_index + 2) % 12],
    }
}

/// Day Ganzhi
#[allow(clippy::boxed_local)]
fn _day_ganzhi(ut: Box<DateTime>) -> GanZhi<'static> {
    let mjd: f64 = modified_julian_day_from_ut(&*ut);
    let index = ((mjd - 10.0) % 60.0).floor() as usize;
    
    let (stem_id, branch_id) = GANZHI_SEXAGESIMAL[index];

    GanZhi {
        stem: &STEMS[stem_id],
        branch: &BRANCHES[branch_id],
    }
}

/// Hour Ganzhi
#[allow(clippy::boxed_local)]
fn _hour_ganzhi(t: Box<Time>, day_stem_no: u8) -> GanZhi<'static> {
    // The branch is easily found by looking at the hour range of the day.
    let branch_id: usize = if t.hour == 23 || t.hour == 0 {
        0
    } else if t.hour < 3 {
        1
    } else if t.hour <= 4 {
        2
    } else if t.hour <= 6 {
        3
    } else if t.hour <= 8 {
        4
    } else if t.hour <= 10 {
        5
    } else if t.hour <= 12 {
        6
    } else if t.hour <= 14 {
        7
    } else if t.hour <= 16 {
        8
    } else if t.hour <= 18 {
        9
    } else if t.hour <= 20 {
        10
    } else {
        11 // if t.hour <= 22
    };

    // The stem is found by looking at a special table.
    // Read comments for 'HOUR_STEM_TABLE' for details.

    let group_id: usize = if day_stem_no == 1 || day_stem_no == 6 {
        0
    } else if day_stem_no == 2 || day_stem_no == 7 {
        1
    } else if day_stem_no == 3 || day_stem_no == 8 {
        2
    } else if day_stem_no == 4 || day_stem_no == 9 {
        3
    } else {
        4 // day_stem_no == 5 || day_stem_no == 10
    };

    let stem_id: usize = HOUR_STEM_TABLE[branch_id][group_id];

    GanZhi {
        stem: &STEMS[stem_id],
        branch: &BRANCHES[branch_id],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bazi_from_local_works() {
        let zone: i8 = 9;

        let lt = DateTime {
            year: 2021,
            month: Month::Jul,
            day: 6.0,
            hour: 14,
            min: 57,
            sec: 17.13779,
        };

        let bazi: Bazi = Bazi::from_local(&lt, zone);

        let year: GanZhi = bazi.year;
        let month: GanZhi = bazi.month;
        let day: GanZhi = bazi.day;
        let hour: GanZhi = bazi.hour;

        println!("lt: {:?}", lt);
        println!("年: {} ({})", year.alphabet(), year.alphabet_ja());
        println!("月: {} ({})", month.alphabet(), month.alphabet_ja());
        println!("日: {} ({})", day.alphabet(), day.alphabet_ja());
        println!("時: {} ({})", hour.alphabet(), hour.alphabet_ja());

        assert_eq!(year.alphabet(), "辛丑");
        assert_eq!(month.alphabet(), "甲午");
        assert_eq!(day.alphabet(), "乙卯");
        assert_eq!(hour.alphabet(), "癸未");
    }

    #[test]
    fn bazi_from_ut_works() {
        let ut = DateTime {
            year: 2021,
            month: Month::Jul,
            day: 6.0,
            hour: 5,
            min: 54,
            sec: 34.27557,
        };

        // Local Time
        let t = Time {
            hour: 14,
            min: 57,
            sec: 17.13779,
        };

        let bazi: Bazi = Bazi::from_ut(&ut, &t);

        let year: GanZhi = bazi.year;
        let month: GanZhi = bazi.month;
        let day: GanZhi = bazi.day;
        let hour: GanZhi = bazi.hour;

        println!("ut: {:?}", ut);
        println!("t: {:?}", t);

        println!("年: {} ({})", year.alphabet(), year.alphabet_ja());
        println!("月: {} ({})", month.alphabet(), month.alphabet_ja());
        println!("日: {} ({})", day.alphabet(), day.alphabet_ja());
        println!("時: {} ({})", hour.alphabet(), hour.alphabet_ja());

        assert_eq!(year.alphabet(), "辛丑");
        assert_eq!(month.alphabet(), "甲午");
        assert_eq!(day.alphabet(), "乙卯");
        assert_eq!(hour.alphabet(), "癸未");
    }
}
