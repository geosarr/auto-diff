use crate::{Const, One, Operation, Var, Zero};

impl<V, T: Zero> Operation<V, T> for Const<T> {
    fn backward(&self, _: &V) -> T {
        T::zero()
    }
}

impl<'a, C, T: Zero + One + PartialEq> Operation<Const<C>, T> for Var<'a, T> {
    fn backward(&self, _: &Const<C>) -> T {
        T::zero()
    }
}
impl<'a, 'b, T: Zero + One + PartialEq> Operation<Var<'b, T>, T> for Var<'a, T> {
    fn backward(&self, variable: &Var<'b, T>) -> T {
        if self == variable {
            T::one()
        } else {
            T::zero()
        }
    }
}

macro_rules! impl_zero_one(
    ( $( $t:ident ),* )=> {
        $(
            impl Zero for $t {
                fn zero() -> Self {
                    0 as $t
                }
            }

            impl One for $t {
                fn one() -> Self {
                    1 as $t
                }
             }

        )*
    }
);
impl_zero_one!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);
