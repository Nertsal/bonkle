use super::*;

#[derive(Debug, Clone)]
pub enum EnemyType {
    Corpse { lifetime: Health },
    Crawler,
    Attacker { attack: Attack },
    Projectile { lifetime: Health },
}
