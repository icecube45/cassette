use serde::Serialize;

pub mod create;

#[derive(Serialize)]
pub struct EntityResponse {
    id: u64
}