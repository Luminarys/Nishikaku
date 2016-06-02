pub mod bomb;
pub mod boss;
pub mod bullet;
pub mod controller;
pub mod enemy;
pub mod level;
pub mod menu;
pub mod mouse;
pub mod player;

use engine::entity::Entity;

pub enum Object {
    Controller(controller::Controller),
    Mouse(mouse::Mouse),
    MainMenu(menu::MainMenu),
    Player(player::Player),
    PlayerBullet(player::Bullet),
}

impl_entity_enum!(Object, Player, PlayerBullet, MainMenu, Mouse, Controller);
