pub enum Event {
    Sound { sound: EventSound },
}

pub enum EventSound {
    BodyHit,
    HeadHit,
    Death,
}
