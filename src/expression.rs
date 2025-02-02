use crate::object::GQLObject;
use regex::Regex;

use crate::transformation::TRANSFORMATIONS;

pub trait Expression {
    fn evaluate(&self, object: &GQLObject) -> String;
}

pub struct StringExpression {
    pub value: String,
}

impl Expression for StringExpression {
    fn evaluate(&self, _object: &GQLObject) -> String {
        return self.value.to_owned();
    }
}

pub struct SymbolExpression {
    pub value: String,
}

impl Expression for SymbolExpression {
    fn evaluate(&self, object: &GQLObject) -> String {
        return object.attributes.get(&self.value).unwrap().to_string();
    }
}

pub struct NotExpression {
    pub right: Box<dyn Expression>,
}

impl Expression for NotExpression {
    fn evaluate(&self, object: &GQLObject) -> String {
        let value = self.right.evaluate(object);
        return (!value.eq("true")).to_string();
    }
}

#[derive(PartialEq)]
pub enum ComparisonOperator {
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Equal,
    NotEqual,
}

pub struct ComparisonExpression {
    pub left: Box<dyn Expression>,
    pub operator: ComparisonOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for ComparisonExpression {
    fn evaluate(&self, object: &GQLObject) -> String {
        let value = self.left.evaluate(object);
        let expected = self.right.evaluate(object);
        let result = value.cmp(&expected);
        return match self.operator {
            ComparisonOperator::Greater => result.is_gt(),
            ComparisonOperator::GreaterEqual => result.is_ge(),
            ComparisonOperator::Less => result.is_lt(),
            ComparisonOperator::LessEqual => result.is_le(),
            ComparisonOperator::Equal => result.is_eq(),
            ComparisonOperator::NotEqual => !result.is_eq(),
        }
        .to_string();
    }
}

#[derive(PartialEq)]
pub enum CheckOperator {
    Contains,
    StartsWith,
    EndsWith,
    Matches,
}

pub struct CheckExpression {
    pub left: Box<dyn Expression>,
    pub operator: CheckOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for CheckExpression {
    fn evaluate(&self, object: &GQLObject) -> String {
        let value = self.left.evaluate(object);
        let expected = self.right.evaluate(object);

        return match self.operator {
            CheckOperator::Contains => value.contains(&expected),
            CheckOperator::StartsWith => value.starts_with(&expected),
            CheckOperator::EndsWith => value.ends_with(&expected),
            CheckOperator::Matches => {
                let regex = Regex::new(&expected);
                if regex.is_err() {
                    return "false".to_owned();
                }
                regex.unwrap().is_match(&value)
            }
        }
        .to_string();
    }
}

#[derive(PartialEq)]
pub enum LogicalOperator {
    Or,
    And,
    Xor,
}

pub struct LogicalExpression {
    pub left: Box<dyn Expression>,
    pub operator: LogicalOperator,
    pub right: Box<dyn Expression>,
}

impl Expression for LogicalExpression {
    fn evaluate(&self, object: &GQLObject) -> String {
        let lhs = self.left.evaluate(object).eq("true");

        if self.operator == LogicalOperator::And && !lhs {
            return "false".to_owned();
        }

        if self.operator == LogicalOperator::Or && lhs {
            return "true".to_owned();
        }

        let rhs = self.right.evaluate(object).eq("true");

        return match self.operator {
            LogicalOperator::And => lhs && rhs,
            LogicalOperator::Or => lhs || rhs,
            LogicalOperator::Xor => lhs ^ rhs,
        }
        .to_string();
    }
}

pub struct CallExpression {
    pub left: Box<dyn Expression>,
    pub function_name: String,
}

impl Expression for CallExpression {
    fn evaluate(&self, object: &GQLObject) -> String {
        let lhs = self.left.evaluate(object);
        let transformation = TRANSFORMATIONS.get(self.function_name.as_str()).unwrap();
        return transformation(lhs);
    }
}
