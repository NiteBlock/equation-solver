use equation_solver::equation::Equation;

#[test]
fn test() {
    let mut eq = Equation::new("sin (x + 132^y) / z").unwrap();
    let value = eq
        .set_value("x", 3.)
        .set_value("y", 2.3)
        .set_value("z", 6.9)
        .evaluate();

    assert_eq!(value, Ok((3.0f64 + 132.0f64.powf(2.3)).sin() / 6.9));
}
