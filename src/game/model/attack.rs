use super::*;

#[derive(Debug, Clone)]
pub struct Attack {
    pub attack_time: Health,
    pub attack_type: AttackType,
}

#[derive(Debug, Clone)]
pub enum AttackType {
    Shoot {
        projectile: Box<EnemyInfo>,
    },
    Bomb {
        projectile: Box<EnemyInfo>,
        projectile_count: usize,
    },
}
