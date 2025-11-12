use hecs::World;

use crate::components::{Position, Velocity};

pub fn movement_system(world: &mut World) {
    for (_id, (pos, vel)) in world.query_mut::<(&mut Position, &Velocity)>(){
        pos.0 += vel.0;
    }
}