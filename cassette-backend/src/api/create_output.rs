use std::sync::Arc;

use axum::{extract, Json};
use hecs::{World, Entity};
use tokio::sync::{RwLockWriteGuard};

use super::EntityResponse;



fn get_new_output(world: RwLockWriteGuard<World>) -> Json<EntityResponse> {

    // spawn new effects
    
    // spawn new mixers
    // spawn new output
    todo!()
}