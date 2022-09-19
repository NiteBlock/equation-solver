use equation_solver::Equation;

fn main() {
    println!("Hello, enter an equation:");

    let equation = read_user_input();

    let equation = Equation::new(equation);

    if let Ok(mut equation) = equation {
        for var in equation.list_vars() {
            println!("Enter a value for {}", var);
            let val = read_user_input();
            let val = val.parse::<f64>();
            if let Ok(val) = val {
                equation.set_value(&var, val);
            } else {
                panic!("Invalid value.");
            }
        }
        let solution = equation.evaluate();
        if let Ok(val) = solution {
            println!("Solution: {}", val);
        } else if let Err(err) = solution {
            println!("Error: {}", err);
        }
    } else if let Err(equation) = equation {
        println!("Error: {}", equation);
    }
}

fn read_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().into()
}
