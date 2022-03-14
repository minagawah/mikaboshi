# Time

Source: [src/time.rs](../src/time.rs)

All the time related. Currently, has only 1 function.

## time::Date
## time::DateTime
## time::Time

## time::ut_from_local

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
