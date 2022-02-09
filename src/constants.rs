use serde::{Deserialize, Serialize};

use crate::ganzhi::{Branch, Stem};
use crate::language::{Language, LanguageDetails};
use crate::solar_terms::SolarTerm;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageData {
    pub en: String,
    pub ja: Vec<String>,
    pub vi: Vec<String>,
    pub zh_cn: Vec<String>,
    pub zh_tw: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolarTermData {
    pub id: u8,
    pub name: LanguageData,
    pub angle: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StemData {
    pub no: u8,
    pub name: LanguageData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchData {
    pub no: u8,
    pub name: LanguageData,
}

lazy_static! {
    pub static ref SOLAR_TERMS: Vec<SolarTerm> = {
        let json = &include_str!("../json/solar_terms.json");
        let data: Vec<SolarTermData> = match serde_json::from_str(json) {
            Ok(json) => json,
            Err(err) => panic!("Error: {}", err),
        };
        data.iter().map(|item| {
            let item = item.clone();
            SolarTerm {
                id: item.id,
                name: Language {
                    en: item.name.en,
                    ja: LanguageDetails::new(&item.name.ja[0], &item.name.ja[1]),
                    vi: LanguageDetails::new(&item.name.vi[0], &item.name.vi[1]),
                    zh_cn: LanguageDetails::new(&item.name.zh_cn[0], &item.name.zh_cn[1]),
                    zh_tw: LanguageDetails::new(&item.name.zh_tw[0], &item.name.zh_tw[1]),
                },
                angle: item.angle,
            }
        }).collect()
    };

    // Combination of Stems (10) and Branches (12) which makes 60 patterns.
    pub static ref GANZHI_SEXAGESIMAL: Vec<(usize, usize)> = {
        let mut v = vec![];
        for i in 0..60 {
            let stem = (i % 10) as usize;
            let branch = (i % 12) as usize;
            v.push((stem, branch));
        }
        v
    };

    pub static ref STEMS: Vec<Stem> = {
        let json = &include_str!("../json/ganzhi_stems.json");
        let data: Vec<StemData> = match serde_json::from_str(json) {
            Ok(json) => json,
            Err(err) => panic!("Error: {}", err),
        };
        data.iter().map(|item| {
            let item = item.clone();
            Stem {
                no: item.no,
                name: Language {
                    en: item.name.en,
                    ja: LanguageDetails::new(&item.name.ja[0], &item.name.ja[1]),
                    vi: LanguageDetails::new(&item.name.vi[0], &item.name.vi[1]),
                    zh_cn: LanguageDetails::new(&item.name.zh_cn[0], &item.name.zh_cn[1]),
                    zh_tw: LanguageDetails::new(&item.name.zh_tw[0], &item.name.zh_tw[1]),
                },
            }
        }).collect()
    };

    pub static ref BRANCHES: Vec<Branch> = {
        let json = &include_str!("../json/ganzhi_branches.json");
        let data: Vec<BranchData> = match serde_json::from_str(json) {
            Ok(json) => json,
            Err(err) => panic!("Error: {}", err),
        };

        data.iter().map(|item| {
            let item = item.clone();
            Branch {
                no: item.no,
                name: Language {
                    en: item.name.en,
                    ja: LanguageDetails::new(&item.name.ja[0], &item.name.ja[1]),
                    vi: LanguageDetails::new(&item.name.vi[0], &item.name.vi[1]),
                    zh_cn: LanguageDetails::new(&item.name.zh_cn[0], &item.name.zh_cn[1]),
                    zh_tw: LanguageDetails::new(&item.name.zh_tw[0], &item.name.zh_tw[1]),
                },
            }
        }).collect()
    };

    /// This is a table used when finding Hour Stem.
    /// Columns represents Day Stem groups, and there are 5 groups.
    /// For insntace, if you have "甲" for Day Stem,
    /// you are looking into the first column (group).
    /// Rows represents Hour Branches, and there are 12.
    /// For instance, if you have "子" for Hour Branch,
    /// you are looking into the first row.
    /// Therefore, when you have "甲" for Day Stem,
    /// and "子" for Hour Branch, Hour Stem is located
    /// in the first column in the first row, which is "甲".
    ///
    /// 　  甲乙丙丁戊
    /// 　  己庚辛壬癸
    /// -------------
    /// 子: 甲丙戊庚壬
    /// 丑: 乙丁己辛癸
    /// 寅: 丙戊庚壬甲
    /// 卯: 丁己辛癸乙
    /// 辰: 戊庚壬甲丙
    /// 巳: 己辛癸乙丁
    /// 午: 庚壬甲丙戊
    /// 未: 辛癸乙丁己
    /// 申: 壬甲丙戊庚
    /// 酉: 癸乙丁己辛
    /// 戌: 甲丙戊庚壬
    /// 亥: 乙丁己辛癸
    pub static ref HOUR_STEM_TABLE: [[usize; 5]; 12] = [
        // 子
        [0, 2, 4, 6, 8], // 甲丙戊庚壬
        // 丑
        [1, 3, 5, 7, 9], // 乙丁己辛癸
        // 寅
        [2, 4, 6, 8, 0], // 丙戊庚壬甲
        // 卯
        [3, 5, 7, 9, 1], // 丁己辛癸乙
        // 辰
        [4, 6, 8, 0, 2], // 戊庚壬甲丙
        // 巳
        [5, 7, 9, 1, 3], // 己辛癸乙丁
        // 午
        [6, 8, 0, 2, 4], // 庚壬甲丙戊
        // 未
        [7, 9, 1, 3, 5], // 辛癸乙丁己
        // 申
        [8, 0, 2, 4, 6], // 壬甲丙戊庚
        // 酉
        [9, 1, 3, 5, 7], // 癸乙丁己辛
        // 戌
        [0, 2, 4, 6, 8], // 甲丙戊庚壬
        // 亥
        [1, 3, 5, 7, 9], // 乙丁己辛癸
    ];
}
