use serde::{Deserialize, Serialize};

pub type BillableRuleId = i32;

/// BillableRule is a struct that represents a rule for how to bill a customer
/// for each operation. It contains a name, operation, and number.
/// Operation is an enum that can be Add, Subtract, Multiply, or Divide.
/// Number is a u32 that represents the number to be used in the operation.
/// For example, if the name is 'cpu', operation is Add and the number is 5,
/// then the customer will be charged 5 times the cpu usage.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct BillableRule {
    pub id: BillableRuleId,
    pub name: String,
    pub operation: BillableOperation,
    pub number: u32,
    pub version: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub enum BillableOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
