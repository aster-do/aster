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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, sqlx::Type)]
#[sqlx(type_name = "billable_operation")]
pub enum BillableOperationPersistent {
    Add,
    Subtract,
    Multiply,
    Divide,
}
