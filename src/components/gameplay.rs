pub struct Speed(pub f32);
pub struct Health {
    pub actual: f32,
    #[allow(dead_code)]
    pub max: f32,
}
#[allow(dead_code)]
pub struct Damage(pub f32);
pub struct DamagePlayer(pub bool);
//pub struct Parent(pub Entity);
