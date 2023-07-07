use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionData {
  pub from: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub to: Option<String>,
  pub content: String,
  pub timestamp: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewAccountData {
  pub account: String,
}

// KiB
pub type KibAccounts = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct KibFields {
  pub accounts: KibAccounts,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Kib {
  pub kib: KibFields
}