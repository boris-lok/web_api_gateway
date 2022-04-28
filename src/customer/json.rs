use chrono::{DateTime, Utc};
use common::utils::time::timestamp2datetime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::pb;

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub id: i64,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<pb::Customer> for Customer {
    fn from(pc: pb::Customer) -> Self {
        Self {
            id: pc.id as i64,
            name: pc.name,
            email: pc.email,
            phone: pc.phone,
            created_at: timestamp2datetime(pc.created_at),
            updated_at: pc.updated_at.map(timestamp2datetime),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCustomerRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

impl From<CreateCustomerRequest> for pb::CreateCustomerRequest {
    fn from(c: CreateCustomerRequest) -> Self {
        Self {
            name: c.name,
            email: c.email,
            phone: c.phone,
        }
    }
}
