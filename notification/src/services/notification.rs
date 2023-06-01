use anyhow::{Ok, Result};

use crate::models::notification::Notification;

#[derive(Debug)]
pub struct Notificationservice {
    //Config & stateful info
}

impl Notificationservice {
    pub fn new() -> Result<Self> {
        Ok(Self {
            //Config & stateful info
        })
    }

    pub fn _handle_notification(&self, _notification: Notification) -> Result<()> {
        //TODO Handle notification
        Ok(())
    }
}
