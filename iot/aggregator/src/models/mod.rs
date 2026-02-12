use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::prelude::*;

#[derive(FromRow, Serialize)]
pub struct Node {
    pub id: i32,
    pub avg_speed: i32,
    pub timestamp: NaiveDateTime,
}
