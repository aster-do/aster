use super::billable_rule::BillableRule;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct BillableRuleUpdate {
    pub action: BillableRuleUpdateAction,
    pub rule: BillableRule,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum BillableRuleUpdateAction {
    Create,
    Update,
    Delete,
}
