//! Логика для 2 уровня. Уникальность в том, что на данном уровне дверь открывается только
//! в том случае, если все мобы из группы 'ArenaMobs' мертвы.

use godot::{
    classes::{Area3D, Marker3D},
    prelude::*,
};

use super::BaseLevel;
use crate::{mob::Mob, player::Player};

/// Реализация логики 2 уровня.
#[derive(GodotClass)]
#[class(base = Node)]
struct Level2 {
    /// Сколько мобов на уровне.
    #[export]
    all_mobs_on_level: i64,

    base: Base<Node>,
}

#[godot_api]
impl INode for Level2 {
    fn init(base: Base<Node>) -> Self {
        Self {
            all_mobs_on_level: 0,
            base,
        }
    }

    fn ready(&mut self) {
        BaseLevel::init_node(&self.base());

        // Инициализирует всех мобов.
        BaseLevel::default_mobs_init(
            &self.base(), 
            self.all_mobs_on_level, 
            |this, mob| {
                mob.signals()
                    .squashed()
                    .connect_obj(this, Self::on_mob_squashed);
            }
        );

        // подключаем сигнал для выхода с уровня.
        self
            .base()
            .get_node_as::<Player>("Player")
            .get_node_as::<Area3D>("ExitDetector")
            .signals()
            .body_entered()
            .connect_obj(self, Self::on_exit_body_entered);

        // Подключаем 'hit' сигнал игрока.
        self.base()
            .get_node_as::<Player>("Player")
            .signals()
            .hit()
            .connect_obj(self, Self::on_player_hit);
    }

    fn process(&mut self, _delta: f64) {
        self.base()
            .get_node_as::<Marker3D>("CameraPivot")
            .set_position(self.base().get_node_as::<Player>("Player").get_position());
    }
}

#[godot_api]
impl Level2 {
    fn on_exit_body_entered(&mut self, _body: Gd<Node3D>) {
        self.base().get_tree().unwrap().change_scene_to_file("res://scenes/main.tscn");
    }

    fn on_player_hit(&mut self) {
        self.base()
            .get_node_as::<Player>("Player")
            .bind_mut()
            .alive();
    }

    fn on_mob_squashed(&mut self) {
        let mobs = self
            .base()
            .get_tree()
            .unwrap()
            .get_nodes_in_group("ArenaMobs");

        let mut mob_dead_count: usize = 0;

        for mob in mobs.iter_shared() {
            let mob = mob.cast::<Mob>();

            if mob.bind().is_die {
                mob_dead_count += 1;
            }
        }

        if mob_dead_count == mobs.len() {
            let mut door = self
                .base()
                .get_tree()
                .unwrap()
                .get_nodes_in_group("door")
                .at(0)
                .cast::<Node3D>();

            let position = door.get_position() + Vector3::new(0.0, 7.0, 0.0);
            door.set_position(position);
        }
    }
}
