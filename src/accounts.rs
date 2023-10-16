use serde::{Deserialize, Serialize};

/// Struct representing an Nylas account.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    pub object: String,
    pub account_id: String,
    pub name: String,
    pub provider: String,
    pub organization_unit: String,
    pub sync_state: String,
    pub linked_at: i32,
    pub email_address: String,
}
