use std::collections::HashSet;

use crate::{
    error::{EquationError, EquationErrorType},
    item::{Item, Operator},
    parse::ParseStream,
};

/// The equation Struct is used to solve an equation.
#[derive(Debug)]
pub struct Equation {
    inner: Group,
}

/// The Group struct is used to represent a group of items (like those inside of bracets).
#[derive(Debug, Clone)]
pub struct Group {
    items: Vec<Item>,
}

// private little thing for equation validation
enum Previous {
    Function,
    Value,
    None,
}

impl Group {
    /// Creates an empty group.
    pub fn new() -> Group {
        Group { items: Vec::new() }
    }
    /// Checks if the group is a valid equation.
    pub fn validate(&self) -> Result<(), EquationError> {
        let mut previous = Previous::None;
        let mut x = self.items.iter();
        while let Some(next) = x.next() {
            match (&previous, next) {
                (Previous::None, Item::Value(_) | Item::Variable(_) | Item::Group(_)) => {
                    previous = Previous::Value
                }
                (Previous::None, Item::Operator(Operator::Functional(_))) => {
                    previous = Previous::Function
                }
                (Previous::Value, Item::Operator(Operator::LeftAssociative(_))) => {
                    previous = Previous::Function
                }
                (Previous::Function, Item::Group(_) | Item::Variable(_) | Item::Value(_)) => {
                    previous = Previous::Value
                }

                (_, _) => {
                    return Err(EquationError::new(
                        "Unexpected token".to_string(),
                        EquationErrorType::UnexpectedToken,
                    ))
                }
            }
        }
        Ok(())
    }
    /// Sets the value of a variable in the group.
    pub fn set_value(&mut self, variable: &str, item: Item) {
        let mut x = self.items.iter_mut();
        while let Some(next) = x.next() {
            match next {
                Item::Variable(val) if val == variable => {
                    *next = item.clone();
                }
                Item::Group(val) => {
                    val.set_value(variable, item.clone());
                }
                _ => {}
            }
        }
    }
    /// Evaluates the value of the group
    pub fn evaluate(&self) -> Result<f64, EquationError> {
        // first lets iter over all groups and evaluate them into values
        let mut items = self.items.clone();
        let mut i = 0;
        while i < items.len() {
            if let Some(Item::Group(val)) = items.get(i) {
                // this is recursive but should never exceed the limit as long as someone doesnt do something stupid...
                let val = val.evaluate()?;
                items[i] = Item::Value(val);
            }
            i += 1;
        }
        // now we evaluate all functions
        let mut i = 0;
        while i < items.len() - 1 {
            let next = items.get(i).unwrap().clone();
            if let Item::Operator(Operator::Functional(func)) = next {
                let to_eval = items.remove(i + 1);
                if let Item::Value(to_eval) = to_eval {
                    let val = func.evaluate(to_eval);
                    items[i] = Item::Value(val);
                } else if let Item::Variable(var) = to_eval {
                    return Err(EquationError::new(
                        format!("Variable {} not set", var),
                        EquationErrorType::UnsetVariable,
                    ));
                } else {
                    return Err(EquationError::new(
                        format!("Unexpected token {:?}", to_eval),
                        EquationErrorType::UnexpectedToken,
                    ));
                }
            }
            i += 1;
        }
        // functions
        let mut i = 1;
        for order in 0..3 {
            while i < items.len() - 1 {
                let next = items.get(i).unwrap().clone();
                if let Item::Operator(Operator::LeftAssociative(op)) = next {
                    if op.is_of_order(&order) {
                        let right = items.remove(i + 1);
                        let left = items.remove(i - 1);
                        let x = (left, right);
                        // println!("{:?}", x);
                        if let (Item::Value(left), Item::Value(right)) = x.clone() {
                            let val = op.eval(left, right);
                            items[i - 1] = Item::Value(val);
                        } else {
                            return Err(EquationError::new(
                                "Incorrect Tokens".to_string(),
                                EquationErrorType::UnexpectedToken,
                            ));
                        }
                        continue;
                    }
                }
                i += 1;
            }
            i = 1;
        }
        if let Some(Item::Value(val)) = items.get(0) {
            Ok(*val)
        } else {
            Err(EquationError::new(
                "Unexpected Tokens".to_string(),
                EquationErrorType::UnexpectedToken,
            ))
        }
    }
    /// Lists the variables that are not set in an equation
    pub fn list_vars(&self) -> HashSet<String> {
        let mut vars = HashSet::new();
        for item in self.items.iter() {
            match item {
                Item::Variable(val) => {
                    vars.insert(val.clone());
                }
                Item::Group(val) => {
                    vars.extend(val.list_vars());
                }
                _ => {}
            }
        }
        vars
    }
}

impl From<Vec<Item>> for Group {
    fn from(val: Vec<Item>) -> Self {
        Group { items: val }
    }
}

impl Equation {
    /// Creates a new equation from a string.
    pub fn new(s: impl Into<String>) -> Result<Equation, EquationError> {
        let parse_stream = ParseStream::new(s.into());
        let mut group: Group = parse_stream.parse_items()?.into();
        // group.validate()?;
        // set consts
        group.set_value("pi", std::f64::consts::PI.into());
        group.set_value("e", std::f64::consts::E.into());
        group.set_value("tau", std::f64::consts::TAU.into());
        group.set_value("deg", (std::f64::consts::PI / 180.0).into());
        Ok(Equation { inner: group })
    }
    /// Sets the value of a variable in the equation.
    pub fn set_value(&mut self, variable: &str, value: f64) -> &mut Self {
        self.inner.set_value(variable, Item::Value(value));
        self
    }
    /// Sets the value of a variable to an equation. Note: The equation's variables used in the original equation will be used, and no already set values will be overwriten.
    pub fn set_equation(&mut self, variable: &str, equation: Self) -> &mut Self {
        self.inner.set_value(variable, Item::Group(equation.inner));
        self
    }
    /// Evaluates the equation.
    pub fn evaluate(&self) -> Result<f64, EquationError> {
        self.inner.evaluate()
    }
    /// Gives a HashSet of all variables (that are not set) in the equation.
    pub fn list_vars(&self) -> HashSet<String> {
        self.inner.list_vars()
    }
}
