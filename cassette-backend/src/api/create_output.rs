use std::sync::Arc;

use axum::{extract, Json};
use hecs::{World, Entity, EntityBuilder, Component};
use tokio::sync::RwLock;

use crate::animation_pipeline::{effect::{Effect, expanding_squares::ExpandingSquares, rainbow_wheel::RainbowWheel}, output::Output};
struct EntityResponse {
    id: u64
}

async fn get_new_output(world: Arc<RwLock<World>>) -> Json<EntityResponse> {
    // let world = world.write().await;

    // let output_entity = world.spawn();
    // // spawn new effects

    // let output = Output {
    //     name: todo!(),
    //     entity: todo!(),
    //     effects_a: todo!(),
    //     effects_b: todo!(),
    //     output_mixer: todo!(),
    //     mixer_a: todo!(),
    //     mixer_b: todo!(),
    // };
    
    // spawn new mixers
    // spawn new output
    todo!()
}

fn create_new_effect_set(mut world: World) -> Vec<Entity> {
    let effects = [
        { Effect::ExpandingSquares(ExpandingSquares::new()) },
        { Effect::RainbowWheel(RainbowWheel::new()) },
    ];

    let mut entities = Vec::new();
    for effect in effects {
        entities.push(world.spawn((effect,)));
    }
    entities
}