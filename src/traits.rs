pub trait Zero {
    fn zero() -> Self;
}
pub trait One {
    fn one() -> Self;
}
pub trait Operation<V, O> {
    // fn backward(&self, variable: &Variable<V>) -> Self;
    fn backward(&self, variable: &V) -> O;
    // fn compute(&self) -> Self;
}
