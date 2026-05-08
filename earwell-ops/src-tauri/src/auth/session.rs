use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role { Admin, Bookkeeper, PartnerView }

impl Role {
    pub fn as_db(&self) -> &'static str {
        match self { Role::Admin => "admin", Role::Bookkeeper => "bookkeeper", Role::PartnerView => "partner_view" }
    }
    pub fn from_db(s: &str) -> Option<Self> {
        match s {
            "admin" => Some(Role::Admin),
            "bookkeeper" => Some(Role::Bookkeeper),
            "partner_view" => Some(Role::PartnerView),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Session {
    pub user_id:    String,
    pub username:   String,
    pub full_name:  String,
    pub role:       Role,
    pub started_at: DateTime<Utc>,
}
