use super::*;

pub struct Wave {
    pub groups: Vec<WaveGroup>,
}

pub struct WaveGroup {
    pub enemies: Vec<EnemyInfo>,
    pub radius: f32,
}

impl Model {
    pub fn next_wave(&mut self) {
        self.current_stage += 1;
        let wave = self.generate_wave();
        for group in wave.groups {
            let group_position = Self::get_random_position_bounds(&self.spawn_bounds);
            self.spawners.push(Spawner::new(group_position, group, 2.0));
        }
        self.area_effects.push(AreaEffect {
            position: Self::get_random_position_bounds(&self.spawn_bounds),
            radius: macroquad::rand::gen_range(5.0, 15.0),
            effect: Effect::Heal { heal: 10.0 },
            lifetime: Health::new(10.0),
        });
        self.events.push(Event::NextWave {
            stage: self.current_stage,
        });
    }

    fn generate_wave(&self) -> Wave {
        // Prepare instances
        let melee = EnemyInfo::new(
            Health::new(150.0),
            5.0,
            2.0,
            25.0,
            MELEE_COLOR,
            EnemyType::Crawler,
        );
        let ranger = EnemyInfo::new(
            Health::new(150.0),
            5.0,
            2.0,
            25.0,
            RANGER_COLOR,
            EnemyType::Attacker {
                attack: Attack {
                    attack_time: Health::new(1.0),
                    attack_type: AttackType::Shoot {
                        target_pos: vec2(0.0, 0.0),
                        projectile: Box::new(EnemyInfo::new(
                            Health::new(1.0),
                            5.0,
                            1.5,
                            30.0,
                            PROJECTILE_COLOR,
                            EnemyType::Projectile {
                                lifetime: Health::new(5.0),
                            },
                        )),
                    },
                },
            },
        );
        let bomber = EnemyInfo::new(
            Health::new(50.0),
            5.0,
            2.0,
            20.0,
            BOMBER_COLOR,
            EnemyType::Attacker {
                attack: Attack {
                    attack_time: Health::new(5.0),
                    attack_type: AttackType::Bomb {
                        projectile_count: 5,
                        projectile: Box::new(EnemyInfo::new(
                            Health::new(1.0),
                            5.0,
                            1.0,
                            40.0,
                            BOMB_COLOR,
                            EnemyType::Projectile {
                                lifetime: Health::new(3.0),
                            },
                        )),
                    },
                },
            },
        );

        // Generate wave
        use macroquad::rand::gen_range;
        let max_groups = (self.current_stage as f32).sqrt().floor() as usize;
        let groups_count = gen_range(max_groups.max(2) - 1, max_groups.max(1));
        let mut wave = Wave {
            groups: Vec::with_capacity(groups_count),
        };
        for _ in 0..groups_count {
            let max_enemies = (self.current_stage as f32).sqrt().floor() as usize;
            let enemies_count = gen_range(max_enemies.max(3) - 2, max_enemies.max(1));
            let mut group = WaveGroup {
                enemies: Vec::with_capacity(enemies_count),
                radius: gen_range(10.0, 15.0),
            };
            let weights = [(2.0, &melee), (1.0, &ranger), (10.5, &bomber)];
            let total_weight: f32 = weights.iter().map(|(weight, _)| weight).sum();
            for _ in 0..enemies_count {
                let mut random = gen_range(0.0, 1.0);
                let mut enemy = None;
                for (weight, enemy_info) in &weights {
                    let chance = *weight / total_weight;
                    random -= chance;
                    if random <= 0.0 {
                        enemy = Some((*enemy_info).clone());
                        break;
                    }
                }
                group.enemies.push(enemy.unwrap());
            }
            wave.groups.push(group);
        }
        wave
    }
}
