pub enum Event {
    NextWave { stage: usize },
    Sound { sound: EventSound },
}

pub enum EventSound {
    BodyHit,
    HeadHit,
    Death,
    Bounce,
    Explosion,
}
