use macroquad::prelude::*;

// Environment visual constants
pub const BACKGROUND_COLOR: Color = BLACK;
pub const BORDER_COLOR: Color = GRAY;
pub const SPAWNER_COLOR: Color = RED;
pub const CORPSE_ALPHA: f32 = 0.5;

// Player visual constants
pub const PLAYER_COLOR: Color = BLUE;
pub const PLAYER_BORDER_COLOR: Color = DARKBLUE;
pub const PLAYER_LIFE_COLOR: Color = DARKBLUE;

// Enemies visual constants
pub const MELEE_COLOR: Color = YELLOW;
pub const RANGER_COLOR: Color = ORANGE;
pub const BOMBER_COLOR: Color = WHITE;
pub const BOMB_COLOR: Color = RED;
pub const PROJECTILE_COLOR: Color = ORANGE;

// Player gameplay constants
pub const PLAYER_HP: f32 = 250.0;
pub const BODY_RADIUS: f32 = 2.0;
pub const BODY_MASS: f32 = 5.0;
pub const BODY_SPEED: f32 = 50.0;
pub const BODY_ACCELERATION: f32 = 3.0;
pub const BODY_HIT_STRENGTH: f32 = 150.0;
pub const HEAD_RADIUS: f32 = 3.0;
pub const HEAD_MASS: f32 = 10.0;
pub const HEAD_SPEED: f32 = 150.0;
pub const HEAD_ACCELERATION: f32 = 10.0;
pub const HEAD_CONNECTION_LENGTH: f32 = 20.0;

// General gameplay constants
pub const DRAG: f32 = 1.0;
pub const BOUNCINESS: f32 = 0.2;
pub const CORPSE_LIFETIME: f32 = 2.5;
pub const PARTICLE_LIFETIME: f32 = 1.0;
pub const GROUP_SPAWN_TIME: f32 = 2.0;
pub const MELEE_SPEED: f32 = 25.0;
pub const MELEE_ACCELERATION: f32 = 2.0;
pub const RANGER_SPEED: f32 = 25.0;
pub const RANGER_ACCELERATION: f32 = 2.0;
