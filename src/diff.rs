use crate::{One, Operation, Variable, Zero};

impl<'a, T: Zero + One + PartialEq> Operation<Variable<'a, T>, T> for Variable<'a, T> {
    fn backward(&self, obj: &Variable<'a, T>) -> T {
        if let Variable::Const(_) = self {
            // Derivative of a constant wrt any object is zero.
            return T::zero();
        } else {
            // Derivative of a variable wrt to itself is one, otherwise 0.
            if obj == self {
                T::one()
            } else {
                T::zero()
            }
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
