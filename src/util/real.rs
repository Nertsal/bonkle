use geng::prelude::{r32, vec2, Float, R32};

pub trait RealConversions {
    fn as_r32(&self) -> R32;
}

impl<T: Float> RealConversions for T {
    fn as_r32(&self) -> R32 {
        r32(self.as_f32())
    }
}

pub trait Vec2RealConversions {
    fn as_f32(&self) -> vec2<f32>;
    fn as_r32(&self) -> vec2<R32>;
}

impl<T: Float> Vec2RealConversions for vec2<T> {
    fn as_f32(&self) -> vec2<f32> {
        self.map(|x| x.as_f32())
    }
    fn as_r32(&self) -> vec2<R32> {
        self.map(|x| r32(x.as_f32()))
    }
}
