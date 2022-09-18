use crate::equation::Group;

/// An item represents a single item in an equation.
#[derive(Debug, Clone)]
pub enum Item {
    /// A value is a number.
    Value(f64),
    /// A variable is a variable that can be replaced later.
    Variable(String),
    /// An operator is an operator that can be used in an equation (i.e. +).
    Operator(Operator),
    /// A group is a group of items (like those inside of bracets).
    Group(Group),
}

/// An operator is an operator that can be used in an equation (i.e. +).
#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    /// A left-associative operator is an operator that requires a left and right of the equation (i.e. +).
    LeftAssociative(LeftAssociativeOperator),
    /// A functional operator is an operator that requires only a right of the equation (i.e. sin).
    Functional(FunctionalOperator),
}

/// A left-associative operator is an operator that requires a left and right of the equation (i.e. +).
#[derive(Debug, Clone, PartialEq)]
pub enum LeftAssociativeOperator {
    /// The addition operator.
    Add,
    /// The subtraction operator.
    Subtract,
    /// The multiplication operator.
    Multiply,
    /// The division operator.
    Divide,
    /// The exponentiation operator.
    Power,
    /// The root operator. (not used)
    Root,
}

/// A functional operator is an operator that requires only a right of the equation (i.e. sin).
#[derive(Debug, Clone, PartialEq)]
pub enum FunctionalOperator {
    /// The Log (base 10)
    Log,
    /// The natural log (base e)
    Ln,
    /// The sine function.
    Sin,
    /// The cosine function.
    Cos,
    /// The tangent function.
    Tan,
    /// The cotangent function.
    Cot,
    /// The secant function.
    Sec,
    /// The cosecant function.
    Csc,
    /// The inverse sin function.
    Arcsin,
    /// The inverse cosine function.
    Arccos,
    /// The inverse tangent function.
    Arctan,
    /// The inverse cotangent function.
    Arccot,
    /// The inverse secant function.
    Arcsec,
    /// The inverse cosecant function.
    Arccsc,
}

impl From<f64> for Item {
    fn from(val: f64) -> Self {
        Item::Value(val)
    }
}

impl From<String> for Item {
    fn from(val: String) -> Self {
        Item::Variable(val)
    }
}

impl From<Operator> for Item {
    fn from(val: Operator) -> Self {
        Item::Operator(val)
    }
}

impl From<Group> for Item {
    fn from(val: Group) -> Self {
        Item::Group(val)
    }
}

impl From<LeftAssociativeOperator> for Operator {
    fn from(val: LeftAssociativeOperator) -> Self {
        Operator::LeftAssociative(val)
    }
}

impl From<FunctionalOperator> for Operator {
    fn from(val: FunctionalOperator) -> Self {
        Operator::Functional(val)
    }
}

impl From<LeftAssociativeOperator> for Item {
    fn from(val: LeftAssociativeOperator) -> Self {
        Item::Operator(val.into())
    }
}

impl From<FunctionalOperator> for Item {
    fn from(val: FunctionalOperator) -> Self {
        Item::Operator(val.into())
    }
}

impl From<Vec<Item>> for Item {
    fn from(val: Vec<Item>) -> Self {
        Item::Group(val.into())
    }
}

impl FunctionalOperator {
    /// Returns the value of the operator given the right value.
    pub fn evaluate(&self, x: f64) -> f64 {
        match self {
            FunctionalOperator::Log => x.log10(),
            FunctionalOperator::Ln => x.ln(),
            FunctionalOperator::Sin => x.sin(),
            FunctionalOperator::Cos => x.cos(),
            FunctionalOperator::Tan => x.tan(),
            FunctionalOperator::Cot => x.tan().recip(),
            FunctionalOperator::Sec => x.cos().recip(),
            FunctionalOperator::Csc => x.sin().recip(),
            FunctionalOperator::Arcsin => x.asin(),
            FunctionalOperator::Arccos => x.acos(),
            FunctionalOperator::Arctan => x.atan(),
            FunctionalOperator::Arccot => x.atan().recip(),
            FunctionalOperator::Arcsec => x.acos().recip(),
            FunctionalOperator::Arccsc => x.asin().recip(),
        }
    }
}

impl LeftAssociativeOperator {
    /// Checks weather the order of the operator is correct. (for pemdas)
    pub fn is_of_order(&self, order: &u8) -> bool {
        // pemdas
        // pe = 0
        // md = 1
        // as = 2

        match self {
            LeftAssociativeOperator::Add | LeftAssociativeOperator::Subtract => *order == 2,
            LeftAssociativeOperator::Multiply | LeftAssociativeOperator::Divide => *order == 1,
            LeftAssociativeOperator::Power | LeftAssociativeOperator::Root => *order == 0,
        }
    }
    /// Returns the value of the operator given the left and right values.
    pub fn eval(&self, lhs: f64, rhs: f64) -> f64 {
        match self {
            LeftAssociativeOperator::Add => lhs + rhs,
            LeftAssociativeOperator::Subtract => lhs - rhs,
            LeftAssociativeOperator::Multiply => lhs * rhs,
            LeftAssociativeOperator::Divide => lhs / rhs,
            LeftAssociativeOperator::Power => lhs.powf(rhs),
            LeftAssociativeOperator::Root => rhs.powf(lhs.recip()),
        }
    }
}
