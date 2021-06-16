use super::*;

#[derive(Debug, Clone, Copy)]
pub struct PhysicsMaterial {
    pub drag: f32,
    pub bounciness: f32,
}

impl PhysicsMaterial {
    pub fn new(drag: f32, bounciness: f32) -> Self {
        assert!(
            bounciness >= 0.0 && bounciness <= 1.0,
            "bounciness must be in range 0..=1, received: {}",
            bounciness
        );
        Self { drag, bounciness }
    }
}
