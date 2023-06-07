use crate::models::billable_rules::billable_rule::BillableRule;

use super::{TokioBroadcastReceiver, TokioBroadcastSender};

pub type BillableRuleSender = TokioBroadcastSender<BillableRule>;
pub type BillableRuleReceiver = TokioBroadcastReceiver<BillableRule>;
