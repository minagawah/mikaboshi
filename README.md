# mikaboshi

## About

`mikaboshi` provides basic calculations for Chinese astrology, known as _Bazi_.
The name `mikaboshi` derives from a Marvel character,
[Amatsu-mikaboshi](https://marvel.fandom.com/wiki/Amatsu-Mikaboshi_(Earth-616)),
or one of goddesses in Shinto who equates to Lucifer in the west.
It literally means "the shinning star in the sky".


## Important

`mikaboshi` depends on
[sowngwala](https://github.com/minagawah/sowngwala/)
for many structs and functions.
When you work with `mikaboshi`, you need some structs from `sowngwala`.
For some functions in `mikaboshi` expects `DateTime` of `sowngwala` as arguments.
As such, `mikaboshi` is re-exporting date & time related structs from `sowngwala`:

```rust
pub use sowngwala::time::{
    Month,
    Date,
    Time,
    DateTime,
};
```

Refer to the source codes of `sowngwala` for specifications:  
https://github.com/minagawah/sowngwala/blob/main/src/time.rs


## Calculation

- Longitude of the sun is that of 0:00 midnight for the given day
- Day changes at 0:00 midnight

## Usage

### `ut_from_local()`

You may convert your local time into _UT_:

```rust
use mikaboshi::time::{
    Month,
    DateTime,
    ut_from_local,
};

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

// {
//     year: 2021,
//     month: Jul,
//     day: 6.0,
//     hour: 14,
//     min: 57,
//     sec: 17.13778432735566
// }
```

### `Bazi::from_local()`

You can easily calculate for _Bazi_:

```rust
use mikaboshi::time::{ Month, DateTime };
use mikaboshi::ganzhi::{ Bazi, GanZhi };

let zone: i8 = 9;

let lt = DateTime {
    year: 2021,
    month: Month::Jul,
    day: 7.0,
    hour: 0,
    min: 0,
    sec: 0.0,
};
let bazi: Bazi = Bazi::from_local(&lt, zone);

let year: GanZhi = bazi.year;
let month: GanZhi = bazi.month;
let day: GanZhi = bazi.day;
let hour: GanZhi = bazi.hour;

println!("年: {} ({})", year.alphabet(), year.alphabet_ja());
println!("月: {} ({})", month.alphabet(), month.alphabet_ja());
println!("日: {} ({})", day.alphabet(), day.alphabet_ja());
println!("時: {} ({})", hour.alphabet(), hour.alphabet_ja());

// 年: 辛丑 (かのと・うし)
// 月: 甲午 (きのえ・うま)
// 日: 乙卯 (きのと・う)
// 時: 癸未 (みずのと・ひつじ)
```


## Dislaimer

There is absolutely no gurantee about the accuracy of the service,
information, or calculated results provided by the program,
and the author of the program cannot be held responsible
in any ways for any adverse consequences.
It is solely for entertaniment only, and your use of the service,
information, or calculated results is entirely at your own risks,
for which the author of the program shall not be liable.
It shall be your own responsibility to ensure the service,
information, or calculated results meet your specific requirements.

## License

MIT license ([LICENSE](LICENSE))
