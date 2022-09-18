use crate::{
    error::{EquationError, EquationErrorType},
    item::Item,
    item::{FunctionalOperator, LeftAssociativeOperator},
};

/// A parse stream represents a string that is to be parsed into an equation.
#[derive(Debug)]
pub struct ParseStream(String);

impl ParseStream {
    /// Creates a new parse stream from a string.
    pub fn new(stream: String) -> ParseStream {
        ParseStream(stream)
    }
    /// Parses all items in the parse stream into a [`Vec<Item>`](crate::item::Item).
    pub fn parse_items(&self) -> Result<Vec<Item>, EquationError> {
        let mut depths: Vec<(Vec<Item>, Option<char>)> = Vec::new();
        depths.push((Vec::new(), None));
        let mut i = 0;
        let mut current_numeric: Option<String> = None;
        while i < self.0.len() {
            let current_group = depths.last_mut().unwrap();
            let c = self.0.chars().nth(i).ok_or(EquationError::new(
                "Unexpected end of stream".to_string(),
                EquationErrorType::UnexpectedToken,
            ))?;
            i += 1;

            match (&current_numeric, c) {
                (Some(val), c) if c == '.' || c.is_numeric() => {
                    current_numeric = Some(
                        {
                            let mut x = val.clone();
                            x.push(c);
                            x
                        }
                        .to_owned(),
                    );
                    continue;
                }
                (None, c) if c.is_numeric() => {
                    current_numeric = Some(c.to_string());
                    continue;
                }
                (Some(val), _) => {
                    current_group.0.push(Item::Value(val.parse().map_err(|_| {
                        EquationError::new(
                            "Invalid number".to_string(),
                            EquationErrorType::UnexpectedToken,
                        )
                    })?));
                    current_numeric = None;
                    i -= 1;
                    continue;
                }
                (None, c) if c.is_alphabetic() => {
                    // peak the next character
                    let mut temp_i = i;
                    while temp_i < self.0.len() {
                        let temp_c = self.0.chars().nth(temp_i).ok_or(EquationError::new(
                            "Unexpected end of stream".to_string(),
                            EquationErrorType::UnexpectedToken,
                        ))?;
                        temp_i += 1;
                        if temp_c.is_alphabetic() {
                            continue;
                        } else {
                            break;
                        }
                    }
                    // special case
                    if temp_i >= self.0.len() - 1 {
                        temp_i = self.0.len() + 1;
                    }
                    let var_name = self.0.get((i - 1)..(temp_i - 1)).unwrap();
                    dbg!(var_name, &self.0, i, temp_i, self.0.len());
                    current_group.0.push(match var_name {
                        "log" => FunctionalOperator::Log.into(),
                        "ln" => FunctionalOperator::Ln.into(),
                        "sin" => FunctionalOperator::Sin.into(),
                        "cos" => FunctionalOperator::Cos.into(),
                        "tan" => FunctionalOperator::Tan.into(),
                        "cot" => FunctionalOperator::Cot.into(),
                        "sec" => FunctionalOperator::Sec.into(),
                        "csc" => FunctionalOperator::Csc.into(),
                        "arcsin" => FunctionalOperator::Arcsin.into(),
                        "arccos" => FunctionalOperator::Arccos.into(),
                        "arctan" => FunctionalOperator::Arctan.into(),
                        "arccot" => FunctionalOperator::Arccot.into(),
                        "arcsec" => FunctionalOperator::Arcsec.into(),
                        "arccsc" => FunctionalOperator::Arccsc.into(),
                        _ => var_name.to_string().into(),
                    });
                    i = temp_i - 1;
                }
                (None, c) if c == '(' || c == '[' => {
                    depths.push((Vec::new(), Some(if c == '(' { ')' } else { ']' })));
                }
                (None, c) if Some(c) == current_group.1 => {
                    let current_group = depths.pop().unwrap();
                    if let Some(parent_group) = depths.last_mut() {
                        parent_group.0.push(current_group.0.into());
                    }
                }
                (None, c) => current_group.0.push(match c {
                    '+' => LeftAssociativeOperator::Add.into(),
                    '-' => LeftAssociativeOperator::Subtract.into(),
                    '*' => LeftAssociativeOperator::Multiply.into(),
                    '/' => LeftAssociativeOperator::Divide.into(),
                    '^' => LeftAssociativeOperator::Power.into(),
                    ' ' => continue,
                    _ => {
                        return Err(EquationError::new(
                            "Unexpected token".to_string(),
                            EquationErrorType::UnexpectedToken,
                        ))
                    }
                }),
            }

            if c.is_numeric() && current_numeric.is_some() || c == '.' {
                continue;
            } else if c.is_numeric() {
                current_numeric = Some(c.to_string());
                continue;
            }

            if c == ' ' {
                continue;
            }
        }
        if let Some(val) = current_numeric {
            depths
                .last_mut()
                .unwrap()
                .0
                .push(Item::Value(val.parse().map_err(|_| {
                    EquationError::new(
                        "Invalid number".to_string(),
                        EquationErrorType::UnexpectedToken,
                    )
                })?));
        }
        if depths.len() != 1 {
            return Err(EquationError::new(
                "Unexpected end of stream (Missing Closing Delimiter)".to_string(),
                EquationErrorType::MissingItems,
            ));
        }
        Ok(depths.pop().unwrap().0)
    }
}
