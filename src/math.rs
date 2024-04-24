use std::ops;

pub fn abs(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}

pub fn acos(x: f64) -> f64 {
    x.acos()
}

pub fn acosh(x: f64) -> f64 {
    x.acosh()
}

pub fn acot(x: f64) -> f64 {
    (1.0 / x).atan()
}

pub fn acoth(x: f64) -> f64 {
    (1.0 / x).atanh()
}

pub enum AggregateFunction {
    Average,
    Count,
    CountA,
    Max,
    Min,
    Product,
    StdevS,
    StdevP,
    Sum,
    VarS,
    VarP,
    Median,
    ModeSNGL,
    Large,
    Small,
    PercentileInc,
    QuartileInc,
    PercentileExc,
    QuartileExc,
}

pub fn aggregate<T: ops::Add + ops::Sub + ops::Mul + ops::Div>(
    function: AggregateFunction,
    options: u8,
    array: Vec<T>,
) {
}

pub fn average<T: ops::Add<Output = T> + ops::Div<f64, Output = f64> + Clone>(
    array: Vec<T>,
) -> f64 {
    let mut avg = array[0].clone();

    for i in array[1..].iter() {
        avg = avg + i.clone();
    }

    avg / array.len() as f64
}

pub fn count<T>(array: Vec<T>) -> usize {
    array.len()
}

pub fn sum<T: ops::Add<Output = T> + Clone>(array: Vec<T>) -> impl ops::Add<Output = T> {
    let mut r = array[0].clone();

    for i in array[1..].iter() {
        r = r + i.clone();
    }

    r
}
