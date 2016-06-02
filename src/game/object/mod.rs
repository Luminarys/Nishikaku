pub mod bomb;
pub mod boss;
pub mod bullet;
pub mod enemy;
pub mod level;
pub mod menu;
pub mod player;

use engine::entity::Entity;

pub enum Object {
    Player(player::Player),
    PlayerBullet(player::Bullet),
}

impl_entity_enum!(Object, Player, PlayerBullet);
