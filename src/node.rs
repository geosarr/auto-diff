use std::ops;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Variable<'a, T> {
    Const(T),
    Var(&'a str, T),
}
impl<'a, T> Variable<'a, T> {
    pub fn name(&self) -> Option<&'a str> {
        if let Variable::Var(name, _) = self {
            return Some(name);
        } else {
            None
        }
    }
    pub fn value(&self) -> &T {
        match self {
            Variable::Const(value) => return value,
            Variable::Var(_, value) => return value,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operator<X, Y> {
    Sum(X, Y),
    Sub(X, Y),
    Mul(X, Y),
    Div(X, Y),
}
impl<X, Y> Operator<X, Y> {
    pub fn left(&self) -> &X {
        match self {
            Self::Sum(x, _) => x,
            Self::Sub(x, _) => x,
            Self::Mul(x, _) => x,
            Self::Div(x, _) => x,
        }
    }
    pub fn right(&self) -> &Y {
        match self {
            Self::Sum(_, y) => y,
            Self::Sub(_, y) => y,
            Self::Mul(_, y) => y,
            Self::Div(_, y) => y,
        }
    }
}

macro_rules! node_op_ty {
    ($($path:ident)::+, $fn:ident, $ty:ty) => {
        impl<'a> $($path)::+<Variable<'a, $ty>> for Variable<'a, $ty> {
            type Output = Variable<'a, $ty>;
            fn $fn(self, other: Variable<'a, $ty>) -> Self::Output {
                match self {
                    Variable::Const(c0) => {
                        match other {
                            // const op var
                            Variable::Var(name, var) => {
                                return Variable::Var(name, c0.$fn(var));
                            },
                            // const op const
                            Variable::Const(c1) => {
                                return Variable::Const(c0.$fn(c1));
                            }
                        }
                    },
                    Variable::Var(name1, var1) => {
                        match other {
                            // var op var
                            Variable::Var(_, var2) => {
                                return Variable::Var(name1, var1.$fn(var2));
                            },
                            // var op const
                            Variable::Const(c) => {
                                return Variable::Var(name1, var1.$fn(c));
                            }
                        }
                    }
                }

            }
        }
        impl<'a> $($path)::+<$ty> for Variable<'a, $ty> {
            type Output = Variable<'a, $ty>;
            fn $fn(self, other: $ty) -> Self::Output {
                match self {
                    Variable::Const(c) => {
                        return Variable::Const(c.$fn(other));
                    },
                    Variable::Var(name, value) => {
                        return Variable::Var(name, value.$fn(other));
                    }
                }
            }
        }
        impl<'a> $($path)::+<Variable<'a, $ty>> for $ty {
            type Output = Variable<'a, $ty>;
            fn $fn(self, other: Variable<'a, $ty>) -> Self::Output {
                match other {
                    Variable::Const(c) => {
                        return Variable::Const(self.$fn(c));
                    },
                    Variable::Var(name, value) => {
                        return Variable::Var(name, self.$fn(value));
                    }
                }
            }
        }
    };
}

macro_rules! node_op (
    ($( $ty:ty ),* )=> {
        $(
            node_op_ty!(ops::Add, add, $ty);
            node_op_ty!(ops::Sub, sub, $ty);
            node_op_ty!(ops::Mul, mul, $ty);
            node_op_ty!(ops::Div, div, $ty);
        )*
    }
);
node_op!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);
