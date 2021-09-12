use crate::language::{ Language, LanguageDetails };
use crate::ganzhi::{ Stem, Branch };
use crate::solar_terms::SolarTerm;

lazy_static! {
    pub static ref SOLAR_TERMS: Vec<SolarTerm> = vec![
        // 立春 (lichun)
        SolarTerm {
            id: 1,
            name: Language {
                en: "lichun".into(),
                ja: LanguageDetails::new("立春", "risshun"),
                ja2: LanguageDetails::new("りっしゅん", "risshun"),
                vi: LanguageDetails::new("lập xuân", "lập xuân"),
                zh_cn: LanguageDetails::new("立春", "lìchūn"),
                zh_tw: LanguageDetails::new("立春", "lìchūn"),
            },
            angle: 315,
        },
        // 雨水 (yushui)
        SolarTerm {
            id: 2,
            name: Language {
                en: "yushui".into(),
                ja: LanguageDetails::new("雨水", "usui"),
                ja2: LanguageDetails::new("うすい", "usui"),
                vi: LanguageDetails::new("vũ thủy", "vũ thủy"),
                zh_cn: LanguageDetails::new("雨水", "yǔshuǐ"),
                zh_tw: LanguageDetails::new("雨水", "yǔshuǐ"),
            },
            angle: 330,
        },
        // 驚蟄 (啓蟄) (jingzhe)
        SolarTerm {
            id: 3,
            name: Language {
                en: "jingzhe".into(),
                ja: LanguageDetails::new("啓蟄", "keichitsu"),
                ja2: LanguageDetails::new("けいちつ", "keichitsu"),
                vi: LanguageDetails::new("kinh trập", "kinh trập"),
                zh_cn: LanguageDetails::new("驚蟄", "jīngzhé"),
                zh_tw: LanguageDetails::new("驚蟄", "jīngzhé"),
            },
            angle: 345,
        },
        // 春分 (chunfen)
        SolarTerm {
            id: 4,
            name: Language {
                en: "chunfen".into(),
                ja: LanguageDetails::new("春分", "shunbun"),
                ja2: LanguageDetails::new("しゅんぶん", "shunbun"),
                vi: LanguageDetails::new("xuân phân", "xuân phân"),
                zh_cn: LanguageDetails::new("春分", "chūnfēn"),
                zh_tw: LanguageDetails::new("春分", "chūnfēn"),
            },
            angle: 0,
        },
        // 清明 (qingming)
        SolarTerm {
            id: 5,
            name: Language {
                en: "qingming".into(),
                ja: LanguageDetails::new("清明", "seimei"),
                ja2: LanguageDetails::new("せいめい", "seimei"),
                vi: LanguageDetails::new("thanh minh", "thanh minh"),
                zh_cn: LanguageDetails::new("清明", "qīngmíng"),
                zh_tw: LanguageDetails::new("清明", "qīngmíng"),
            },
            angle: 15,
        },
        // 穀雨 (guyu)
        SolarTerm {
            id: 6,
            name: Language {
                en: "guyu".into(),
                ja: LanguageDetails::new("穀雨", "kokuu"),
                ja2: LanguageDetails::new("こくう", "kokuu"),
                vi: LanguageDetails::new("cốc vũ", "cốc vũ"),
                zh_cn: LanguageDetails::new("穀雨", "gǔyǔ"),
                zh_tw: LanguageDetails::new("穀雨", "gǔyǔ"),
            },
            angle: 30,
        },
        // 立夏 (lixia)
        SolarTerm {
            id: 7,
            name: Language {
                en: "lixia".into(),
                ja: LanguageDetails::new("立夏", "rikka"),
                ja2: LanguageDetails::new("りっか", "rikka"),
                vi: LanguageDetails::new("lập hạ", "lập hạ"),
                zh_cn: LanguageDetails::new("立夏", "lìxià"),
                zh_tw: LanguageDetails::new("立夏", "lìxià"),
            },
            angle: 45,
        },
        // 小滿 (xiaoman)
        SolarTerm {
            id: 8,
            name: Language {
                en: "xiaoman".into(),
                ja: LanguageDetails::new("小満", "shouman"),
                ja2: LanguageDetails::new("しょうまん", "shouman"),
                vi: LanguageDetails::new("tiểu mãn", "tiểu mãn"),
                zh_cn: LanguageDetails::new("小滿", "xiǎomǎn"),
                zh_tw: LanguageDetails::new("小滿", "xiǎomǎn"),
            },
            angle: 60,
        },
        // 芒種 (mangzhong)
        SolarTerm {
            id: 9,
            name: Language {
                en: "mangzhong".into(),
                ja: LanguageDetails::new("芒種", "boushu"),
                ja2: LanguageDetails::new("ぼうしゅ", "boushu"),
                vi: LanguageDetails::new("mang chủng", "mang chủng"),
                zh_cn: LanguageDetails::new("芒種", "mángzhòng"),
                zh_tw: LanguageDetails::new("芒種", "mángzhòng"),
            },
            angle: 75,
        },
        // 夏至 (xiazhi)
        SolarTerm {
            id: 10,
            name: Language {
                en: "xiazhi".into(),
                ja: LanguageDetails::new("夏至", "geshi"),
                ja2: LanguageDetails::new("げし", "geshi"),
                vi: LanguageDetails::new("hạ chí", "hạ chí"),
                zh_cn: LanguageDetails::new("夏至", "xiàzhì"),
                zh_tw: LanguageDetails::new("夏至", "xiàzhì"),
            },
            angle: 90,
        },
        // 小暑 (xiaoshu)
        SolarTerm {
            id: 11,
            name: Language {
                en: "xiaoshu".into(),
                ja: LanguageDetails::new("小暑", "shousho"),
                ja2: LanguageDetails::new("しょうしょ", "shousho"),
                vi: LanguageDetails::new("tiểu thử", "tiểu thử"),
                zh_cn: LanguageDetails::new("小暑", "xiǎoshǔ"),
                zh_tw: LanguageDetails::new("小暑", "xiǎoshǔ"),
            },
            angle: 105,
        },
        // 大暑 (dashu)
        SolarTerm {
            id: 12,
            name: Language {
                en: "dashu".into(),
                ja: LanguageDetails::new("大暑", "taisho"),
                ja2: LanguageDetails::new("たいしょ", "taisho"),
                vi: LanguageDetails::new("đại thử", "đại thử"),
                zh_cn: LanguageDetails::new("大暑", "dàshǔ"),
                zh_tw: LanguageDetails::new("大暑", "dàshǔ"),
            },
            angle: 120,
        },
        // 立秋 (liqiu)
        SolarTerm {
            id: 13,
            name: Language {
                en: "liqiu".into(),
                ja: LanguageDetails::new("立秋", "risshuu"),
                ja2: LanguageDetails::new("りっしゅう", "risshuu"),
                vi: LanguageDetails::new("lập thu", "lập thu"),
                zh_cn: LanguageDetails::new("立秋", "lìqiū"),
                zh_tw: LanguageDetails::new("立秋", "lìqiū"),
            },
            angle: 135,
        },
        // 處暑 (chushu)
        SolarTerm {
            id: 14,
            name: Language {
                en: "chushu".into(),
                ja: LanguageDetails::new("処暑", "shosho"),
                ja2: LanguageDetails::new("しょしょ", "shosho"),
                vi: LanguageDetails::new("xử thử", "xử thử"),
                zh_cn: LanguageDetails::new("處暑", "chǔshǔ"),
                zh_tw: LanguageDetails::new("處暑", "chǔshǔ"),
            },
            angle: 150,
        },
        // 白露 (bailu)
        SolarTerm {
            id: 15,
            name: Language {
                en: "bailu".into(),
                ja: LanguageDetails::new("白露", "hakuro"),
                ja2: LanguageDetails::new("はくろ", "hakuro"),
                vi: LanguageDetails::new("bạch lộ", "bạch lộ"),
                zh_cn: LanguageDetails::new("白露", "báilù"),
                zh_tw: LanguageDetails::new("白露", "báilù"),
            },
            angle: 165,
        },
        // 秋分 (qiufen)
        SolarTerm {
            id: 16,
            name: Language {
                en: "qiufen".into(),
                ja: LanguageDetails::new("秋分", "shubun"),
                ja2: LanguageDetails::new("しゅうぶん", "shubun"),
                vi: LanguageDetails::new("thu phân", "thu phân"),
                zh_cn: LanguageDetails::new("秋分", "qiūfēn"),
                zh_tw: LanguageDetails::new("秋分", "qiūfēn"),
            },
            angle: 180,
        },
        // 寒露 (hanlu)
        SolarTerm {
            id: 17,
            name: Language {
                en: "hanlu".into(),
                ja: LanguageDetails::new("寒露", "kanro"),
                ja2: LanguageDetails::new("かんろ", "kanro"),
                vi: LanguageDetails::new("hàn lộ", "hàn lộ"),
                zh_cn: LanguageDetails::new("寒露", "hánlù"),
                zh_tw: LanguageDetails::new("寒露", "hánlù"),
            },
            angle: 195,
        },
        // 霜降 (shuangjiang)
        SolarTerm {
            id: 18,
            name: Language {
                en: "shuanjiang".into(),
                ja: LanguageDetails::new("霜降", "soukou"),
                ja2: LanguageDetails::new("そうこう", "soukou"),
                vi: LanguageDetails::new("sương giáng", "sương giáng"),
                zh_cn: LanguageDetails::new("霜降", "shuāngjiàng"),
                zh_tw: LanguageDetails::new("", ""),
            },
            angle: 210,
        },
        // 立冬 (lidong)
        SolarTerm {
            id: 19,
            name: Language {
                en: "lidong".into(),
                ja: LanguageDetails::new("立冬", "rittou"),
                ja2: LanguageDetails::new("りっとう", "rittou"),
                vi: LanguageDetails::new("lập đông", "lập đông"),
                zh_cn: LanguageDetails::new("立冬", "lìdōng"),
                zh_tw: LanguageDetails::new("立冬", "lìdōng"),
            },
            angle: 225,
        },
        // 小雪 (xiaoxue)
        SolarTerm {
            id: 20,
            name: Language {
                en: "xiaoxue".into(),
                ja: LanguageDetails::new("小雪", "shousetsu"),
                ja2: LanguageDetails::new("しょうせつ", "shousetsu"),
                vi: LanguageDetails::new("tiểu tuyết", "tiểu tuyết"),
                zh_cn: LanguageDetails::new("小雪", "xiǎoxuě"),
                zh_tw: LanguageDetails::new("小雪", "xiǎoxuě"),
            },
            angle: 240,
        },
        // 大雪 (daxue)
        SolarTerm {
            id: 21,
            name: Language {
                en: "daxue".into(),
                ja: LanguageDetails::new("大雪", "taisetsu"),
                ja2: LanguageDetails::new("たいせつ", "taisetsu"),
                vi: LanguageDetails::new("đại tuyết", "đại tuyết"),
                zh_cn: LanguageDetails::new("大雪", "dàxuě"),
                zh_tw: LanguageDetails::new("大雪", "dàxuě"),
            },
            angle: 255,
        },
        // 冬至 (dongzhi)
        SolarTerm {
            id: 22,
            name: Language {
                en: "dongzhi".into(),
                ja: LanguageDetails::new("冬至", "touji"),
                ja2: LanguageDetails::new("とうじ", "touji"),
                vi: LanguageDetails::new("Đông chí", "Đông chí"),
                zh_cn: LanguageDetails::new("冬至", "dōngzhì"),
                zh_tw: LanguageDetails::new("冬至", "dōngzhì"),
            },
            angle: 270,
        },
        // 小寒 (xiaohan)
        SolarTerm {
            id: 23,
            name: Language {
                en: "xiaohan".into(),
                ja: LanguageDetails::new("小寒", "shoukan"),
                ja2: LanguageDetails::new("しょうかん", "shoukan"),
                vi: LanguageDetails::new("tiểu hàn", "tiểu hàn"),
                zh_cn: LanguageDetails::new("小寒", "xiǎohán"),
                zh_tw: LanguageDetails::new("小寒", "xiǎohán"),
            },
            angle: 285,
        },
        // 大寒 (dahan)
        SolarTerm {
            id: 24,
            name: Language {
                en: "dahan".into(),
                ja: LanguageDetails::new("大寒", "daikan"),
                ja2: LanguageDetails::new("だいかん", "daikan"),
                vi: LanguageDetails::new("Đại hàn", "Đại hàn"),
                zh_cn: LanguageDetails::new("大寒", "dàhán"),
                zh_tw: LanguageDetails::new("大寒", "dàhán"),
            },
            angle: 300,
        },
    ];

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

    pub static ref STEMS: Vec<Stem> = vec![
        // 甲 (jia)
        Stem {
            no: 1,
            name: Language {
                en: "jia".into(),
                ja: LanguageDetails::new("きのえ", "kinoe"),
                ja2: LanguageDetails::new("コウ", "kou"),
                vi: LanguageDetails::new("giáp", "giap"),
                zh_cn: LanguageDetails::new("甲", "jiǎ"),
                zh_tw: LanguageDetails::new("甲", "jiǎ"),
            },
        },
        // 乙 (yi)
        Stem {
            no: 2,
            name: Language {
                en: "yi".into(),
                ja: LanguageDetails::new("きのと", "kinoto"),
                ja2: LanguageDetails::new("オツ", "otsu"),
                vi: LanguageDetails::new("ất", "ất"),
                zh_cn: LanguageDetails::new("乙", "yǐ"),
                zh_tw: LanguageDetails::new("乙", "yǐ"),
            },
        },
        // 丙 (bing)
        Stem {
            no: 3,
            name: Language {
                en: "bing".into(),
                ja: LanguageDetails::new("ひのえ", "hinoe"),
                ja2: LanguageDetails::new("ヘイ", "hei"),
                vi: LanguageDetails::new("bính", "bính"),
                zh_cn: LanguageDetails::new("丙", "bǐng"),
                zh_tw: LanguageDetails::new("丙", "bǐng"),
            },
        },
        // 丁 (ding)
        Stem {
            no: 4,
            name: Language {
                en: "ding".into(),
                ja: LanguageDetails::new("ひのと", "hinoto"),
                ja2: LanguageDetails::new("テイ", "tei"),
                vi: LanguageDetails::new("đinh", "đinh"),
                zh_cn: LanguageDetails::new("丁", "dīng"),
                zh_tw: LanguageDetails::new("丁", "dīng"),
            },
        },
        // 戊 (wu)
        Stem {
            no: 5,
            name: Language {
                en: "wu".into(),
                ja: LanguageDetails::new("つちのえ", "tsuchinoe"),
                ja2: LanguageDetails::new("ボ", "bo"),
                vi: LanguageDetails::new("mậu", "mậu"),
                zh_cn: LanguageDetails::new("戊", "wù"),
                zh_tw: LanguageDetails::new("戊", "wù"),
            },
        },
        // 己 (ji)
        Stem {
            no: 6,
            name: Language {
                en: "ji".into(),
                ja: LanguageDetails::new("つちのと", "tsuchinoe"),
                ja2: LanguageDetails::new("キ", "ki"),
                vi: LanguageDetails::new("kỷ", "kỷ"),
                zh_cn: LanguageDetails::new("己", "jǐ"),
                zh_tw: LanguageDetails::new("己", "jǐ"),
            },
        },
        // 庚 (geng)
        Stem {
            no: 7,
            name: Language {
                en: "geng".into(),
                ja: LanguageDetails::new("かのえ", "kanoe"),
                ja2: LanguageDetails::new("コウ", "kou"),
                vi: LanguageDetails::new("canh", "canh"),
                zh_cn: LanguageDetails::new("庚", "gēng"),
                zh_tw: LanguageDetails::new("庚", "gēng"),
            },
        },
        // 辛 (xin)
        Stem {
            no: 8,
            name: Language {
                en: "xin".into(),
                ja: LanguageDetails::new("かのと", "kanoto"),
                ja2: LanguageDetails::new("シン", "shin"),
                vi: LanguageDetails::new("tân", "tân"),
                zh_cn: LanguageDetails::new("辛", "xīn"),
                zh_tw: LanguageDetails::new("辛", "xīn"),
            },
        },
        // 壬 (ren)
        Stem {
            no: 9,
            name: Language {
                en: "ren".into(),
                ja: LanguageDetails::new("みずのえ", "mizunoe"),
                ja2: LanguageDetails::new("ジン", "jin"),
                vi: LanguageDetails::new("nhâm", "nhâm"),
                zh_cn: LanguageDetails::new("壬", "rén"),
                zh_tw: LanguageDetails::new("壬", "rén"),
            },
        },
        // 癸 (gui)
        Stem {
            no: 10,
            name: Language {
                en: "gui".into(),
                ja: LanguageDetails::new("みずのと", "mizunoto"),
                ja2: LanguageDetails::new("キ", "ki"),
                vi: LanguageDetails::new("quý", "quý"),
                zh_cn: LanguageDetails::new("癸", "guǐ"),
                zh_tw: LanguageDetails::new("癸", "guǐ"),
            },
        },
    ];

    pub static ref BRANCHES: Vec<Branch> = vec![
        // 子 (zi)
        Branch {
            no: 1,
            name: Language {
                en: "zi".into(),
                ja: LanguageDetails::new("ね", "ne"),
                ja2: LanguageDetails::new("し", "shi"),
                vi: LanguageDetails::new("tí", "tí"),
                zh_cn: LanguageDetails::new("子", "zǐ"),
                zh_tw: LanguageDetails::new("子", "zǐ"),
            },
        },
        // 丑 (chou)
        Branch {
            no: 2,
            name: Language {
                en: "chou".into(),
                ja: LanguageDetails::new("うし", "ushi"),
                ja2: LanguageDetails::new("ちゅう", "chu"),
                vi: LanguageDetails::new("sửu", "sửu"),
                zh_cn: LanguageDetails::new("丑", "chǒu"),
                zh_tw: LanguageDetails::new("丑", "chǒu"),
            },
        },
        // 寅 (yin)
        Branch {
            no: 3,
            name: Language {
                en: "".into(),
                ja: LanguageDetails::new("とら", "tora"),
                ja2: LanguageDetails::new("いん", "in"),
                vi: LanguageDetails::new("dần", "dần"),
                zh_cn: LanguageDetails::new("寅", "yín"),
                zh_tw: LanguageDetails::new("寅", "yín"),
            },
        },
        // 卯 (mao)
        Branch {
            no: 4,
            name: Language {
                en: "mao".into(),
                ja: LanguageDetails::new("う", "u"),
                ja2: LanguageDetails::new("ぼう", "bou"),
                vi: LanguageDetails::new("mão", "mão"),
                zh_cn: LanguageDetails::new("卯", "mǎo"),
                zh_tw: LanguageDetails::new("卯", "mǎo"),
            },
        },
        // 辰 (chen)
        Branch {
            no: 5,
            name: Language {
                en: "".into(),
                ja: LanguageDetails::new("たつ", "tatsu"),
                ja2: LanguageDetails::new("しん", "shin"),
                vi: LanguageDetails::new("thần", "thần"),
                zh_cn: LanguageDetails::new("辰", "chén"),
                zh_tw: LanguageDetails::new("辰", "chén"),
            },
        },
        // 巳 (si)
        Branch {
            no: 6,
            name: Language {
                en: "si".into(),
                ja: LanguageDetails::new("み", "mi"),
                ja2: LanguageDetails::new("し", "shi"),
                vi: LanguageDetails::new("tị", "tị"),
                zh_cn: LanguageDetails::new("巳", "sì"),
                zh_tw: LanguageDetails::new("巳", "sì"),
            },
        },
        // 午 (wu)
        Branch {
            no: 7,
            name: Language {
                en: "wu".into(),
                ja: LanguageDetails::new("うま", "uma"),
                ja2: LanguageDetails::new("ご", "go"),
                vi: LanguageDetails::new("ngọ", "ngọ"),
                zh_cn: LanguageDetails::new("午", "wǔ"),
                zh_tw: LanguageDetails::new("午", "wǔ"),
            },
        },
        // 未 (wei)
        Branch {
            no: 8,
            name: Language {
                en: "wei".into(),
                ja: LanguageDetails::new("ひつじ", "hitsuji"),
                ja2: LanguageDetails::new("び", "bi"),
                vi: LanguageDetails::new("vị", "vị"),
                zh_cn: LanguageDetails::new("未", "wèi"),
                zh_tw: LanguageDetails::new("未", "wèi"),
            },
        },
        // 申 (shen)
        Branch {
            no: 9,
            name: Language {
                en: "shen".into(),
                ja: LanguageDetails::new("さる", "saru"),
                ja2: LanguageDetails::new("しん", "shin"),
                vi: LanguageDetails::new("thân", "thân"),
                zh_cn: LanguageDetails::new("申", "shēn"),
                zh_tw: LanguageDetails::new("申", "shēn"),
            },
        },
        // 酉 (you)
        Branch {
            no: 10,
            name: Language {
                en: "you".into(),
                ja: LanguageDetails::new("とり", "tori"),
                ja2: LanguageDetails::new("ゆう", "yuu"),
                vi: LanguageDetails::new("dậu", "dậu"),
                zh_cn: LanguageDetails::new("酉", "yǒu"),
                zh_tw: LanguageDetails::new("酉", "yǒu"),
            },
        },
        // 戌 (xu)
        Branch {
            no: 11,
            name: Language {
                en: "xu".into(),
                ja: LanguageDetails::new("いぬ", "inu"),
                ja2: LanguageDetails::new("じゅつ", "jutsu"),
                vi: LanguageDetails::new("tuất", "tuất"),
                zh_cn: LanguageDetails::new("戌", "xū"),
                zh_tw: LanguageDetails::new("戌", "xū"),
            },
        },
        // 亥 (hai)
        Branch {
            no: 12,
            name: Language {
                en: "hai".into(),
                ja: LanguageDetails::new("い", "i"),
                ja2: LanguageDetails::new("がい", "gai"),
                vi: LanguageDetails::new("hợi", "hợi"),
                zh_cn: LanguageDetails::new("亥", "hài"),
                zh_tw: LanguageDetails::new("亥", "hài"),
            },
        },
    ];

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
