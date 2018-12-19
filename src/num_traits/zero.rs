pub trait Zero {
    fn zero() -> Self;
}

impl Zero for i32 {
    fn zero() -> i32 {
        0
    }
}

impl Zero for f32 {
    fn zero() -> f32 {
        0.0
    }
}
