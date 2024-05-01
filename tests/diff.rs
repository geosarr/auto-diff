use auto_diff::{Operation, Variable};

#[test]
fn test_backward() {
    let x = Variable::Const(-5);
    let y = Variable::Var("y", 10);
    assert_eq!(0, x.backward(&x));
    assert_eq!(0, x.backward(&y));
    assert_eq!(1, y.backward(&y));
    assert_eq!(0, y.backward(&x));
}
