/// Resource to handle Fixed Update Logic (Tick)
pub struct GameTick {
    pub tick_rate: f32,
    pub ticks_elapsed: u32,
    pub accumulator: f32,
}

impl Default for GameTick {
    fn default() -> Self {
        Self {
            tick_rate: 1.0 / 32.0,
            ticks_elapsed: 0,
            accumulator: 0.0,
        }
    }
}
