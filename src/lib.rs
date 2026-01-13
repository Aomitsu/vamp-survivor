#[macro_export]
#[allow(clippy::crate_in_macro_def)]
macro_rules! despawn_phys_entity {
    ($world:expr, $entity:expr) => {
        
        if let Err(e) = $world.insert_one($entity, crate::components::tags::Despawn) {
            log::error!("Failed to despawn entity {:?}: {:?}", $entity, e);
        }
    };
}
