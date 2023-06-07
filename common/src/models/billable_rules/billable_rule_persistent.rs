use anyhow::Ok;

use super::{billable_rule::BillableOperation, billable_rule_dto::BillableRuleDto};

/// BillableRule is a struct that represents a rule for how to bill a customer
/// for each operation. It contains a name, operation, and number.
/// Operation is an enum that can be Add, Subtract, Multiply, or Divide.
/// Number is a u32 that represents the number to be used in the operation.
/// For example, if the name is 'cpu', operation is Add and the number is 5,
/// then the customer will be charged 5 times the cpu usage.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct BillableRulePersistent {
    pub id: i32,
    pub name: String,
    pub operation: BillableOperationPersistent,
    pub number: i32,
    pub version: i32,
}

impl BillableRulePersistent {
    pub fn from(rule: &BillableRuleDto) -> Result<Self, anyhow::Error> {
        let billable_rule_persistent = Self {
            id: -1,
            name: rule.name.clone(),
            operation: BillableOperationPersistent::from(&rule.operation),
            number: rule.number as i32,
            version: -1,
        };

        Ok(billable_rule_persistent)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, sqlx::Type)]
#[sqlx(type_name = "billable_operation")]
pub enum BillableOperationPersistent {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl BillableOperationPersistent {
    pub fn from(operation: &BillableOperation) -> Self {
        match operation {
            BillableOperation::Add => Self::Add,
            BillableOperation::Subtract => Self::Subtract,
            BillableOperation::Multiply => Self::Multiply,
            BillableOperation::Divide => Self::Divide,
        }
    }
}
