use anyhow::{Ok, Result};

use crate::services::rule::RuleService;

#[derive(Debug)]
pub struct RuleController {
    //Config & stateful info
    _rule_service: RuleService,
}

impl RuleController {
    pub fn new() -> Result<Self> {
        Ok(Self {
            //Config & stateful info
            _rule_service: RuleService::new()?,
        })
    }

    pub fn _start(&self) -> Result<()> {
        //TODO Start http server
        Ok(())
    }
}

//TODO Implement graphql trait
