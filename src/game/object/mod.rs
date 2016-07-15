pub mod bomb;
pub mod boss;
pub mod bullet;
pub mod controller;
pub mod enemy;
pub mod level;
pub mod menu;
pub mod mouse;
pub mod player;
pub mod widget;

pub enum Object {
    Controller(controller::Controller),
    Mouse(mouse::Mouse),
    MainMenu(menu::MainMenu),
    Level(level::Level),
    LevelStatus(widget::LevelStatus),
    Player(player::Player),
    PlayerBullet(player::Bullet),
    Enemy(enemy::Enemy),
    Bullet(bullet::Bullet),
}

impl_entity_enum!(Object, Player, PlayerBullet, MainMenu, Mouse, Controller, Level, Enemy, Bullet, LevelStatus);
