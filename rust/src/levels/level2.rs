//! Логика для 2 уровня. Уникальность в том, что на данном уровне дверь открывается только
//! в том случае, если все мобы из группы 'ArenaMobs' мертвы.

use godot::{
    classes::{CollisionShape3D, InputEvent, Marker3D},
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

    /// Все мобы на уровне которые были найдены.
    mobs: Vec<Gd<Mob>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for Level2 {
    fn init(base: Base<Node>) -> Self {
        Self {
            all_mobs_on_level: 0,
            mobs: Vec::new(),
            base,
        }
    }

    fn ready(&mut self) { 
        // Инициализируем всех мобов.
        self.mobs =
            BaseLevel::default_mobs_init(
                &self.base(),
                self.all_mobs_on_level,
                |this, mob| {
                    mob.signals()
                        .squashed()
                        .connect_obj(this, Self::on_mob_squashed);
                }
            );

        // Инициализируем уровень.
        BaseLevel::init_node(
            &self.base(), 
            self.mobs.clone(), 
            self.base().get_scene_file_path(), 
            2
        );

        // Подключаем 'hit' сигнал игрока.
        self.base()
            .get_node_as::<Player>("Player")
            .signals()
            .hit()
            .connect_obj(self, Self::on_player_hit);

        self.base().get_node_as::<Node3D>("Objects/Exit").show();
        self.base().get_node_as::<CollisionShape3D>(
            "Objects/Exit/CharacterBody3D/CollisionShape3D"
        ).set_disabled(false);
    }

    fn process(&mut self, _delta: f64) {
        self.base()
            .get_node_as::<Marker3D>("CameraPivot")
            .set_position(self.base().get_node_as::<Player>("Player").get_position());
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("pause") {
            BaseLevel::pause(self.base().clone().cast::<Self>(), &mut self.mobs);
        }
    }
}

#[godot_api]
impl Level2 {
    fn on_player_hit(&mut self) {
        self
            .base()
            .get_node_as::<Player>("Player")
            .bind_mut()
            .alive();

        self.base().get_node_as::<Node3D>("Objects/Exit").show();
        
        self
            .base()
            .get_node_as::<CollisionShape3D>("Objects/Exit/CharacterBody3D/CollisionShape3D")
            .set_disabled(false);
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
