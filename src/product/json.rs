use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

use common::json::product::Product;
use common::utils::time::timestamp2datetime;

use crate::pb;

impl From<pb::Product> for Product {
    fn from(e: pb::Product) -> Self {
        Self {
            id: e.id as i64,
            name: e.name,
            currency: e.currency as i16,
            price: Decimal::from_f64(e.price).unwrap(),
            created_at: timestamp2datetime(e.created_at),
            updated_at: e.updated_at.map(timestamp2datetime),
            deleted_at: e.deleted_at.map(timestamp2datetime),
        }
    }
}
