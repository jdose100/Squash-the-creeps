//! Данный модуль содержит логику уровней.
//! BaseLevel - это класс, который реализует минимум функций для работы уровня,
//! является полностью самостоятельным. Подмодули же содержат доп. логику, а 
//! BaseLevel в этом случае выступает в качестве родителя.

use crate::{mob::Mob, player::Player, ui::main_menu::MainMenu};
use godot::{
    classes::{Label, Marker3D, Path3D, PathFollow3D},
    global::randf_range,
    obj::{BaseRef, WithBaseField},
    prelude::*,
};

mod level2;

/// Класс, реализующий минимальную логику
/// для самостоятельности уровня.
#[derive(GodotClass)]
#[class(base = Node)]
struct BaseLevel {
    /// Как много мобов на уровне.
    #[export]
    all_mobs_on_level: i64,

    /// Сколько мобов было убито на уровне.
    squashed_mobs: i64,

    base: Base<Node>,
}

#[godot_api]
impl INode for BaseLevel {
    fn init(base: Base<Node>) -> Self {
        Self {
            all_mobs_on_level: 0,
            squashed_mobs: 0,
            base,
        }
    }

    fn ready(&mut self) {
        Self::init_node(&self.base());

        // Инициализируем всех мобов.
        Self::default_mobs_init(
            &self.base(),
            self.all_mobs_on_level,
            |this, mob| {
                mob.signals()
                    .squashed()
                    .connect_obj(this, Self::on_mob_squashed);
            }
        );

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
impl BaseLevel {
    /// Настраивает игрока и IU с параметрами BaseLevel, 
    /// необходим для удаления шаблонного кода в дочерних классах. 
    pub fn init_node<T: GodotClass + INode>(node: &BaseRef<T>) {
        if let Some(mut player) = node.try_get_node_as::<Player>("Player") {
            player.bind_mut().alive();
        }

        if let Some(mut ui) = node.try_get_node_as::<MainMenu>("UserInterface") {
            ui.bind_mut().start_new_game();
        }
    }

    /// Инициировать и вызвать «метод» для всех мобов в сцене, указанных счетчиком. 
    pub fn default_mobs_init<T, F>(node: &BaseRef<T>, mob_count: i64, method: F)
    where
        T: GodotClass + INode + Inherits<Node>,
        F: Fn(&Gd<T>, &mut Gd<Mob>),
    {
        for i in 0..mob_count {
            let mob = node.try_get_node_as::<Mob>(&get_text_mob_name(i));

            if let Some(mut mob) = mob {
                method(&node.to_godot().cast(), &mut mob);

                //* The following code is needed to be able to move along a given path!

                let follow_path =
                    node.try_get_node_as::<PathFollow3D>(&get_text_mob_follow_path(i));

                let path = node.try_get_node_as::<Path3D>(&get_text_mob_path(i));
                let follow_speed = randf_range(0.1, 0.34);

                mob.bind_mut().initialize(follow_path, path, follow_speed);
            }
        }
    }

    /// Обновляет «squashed_mobs» для раздавленных мобов, если все мобы на уровне раздавлены. 
    fn on_mob_squashed(&mut self) {
        self.squashed_mobs += 1;

        if self.squashed_mobs == self.all_mobs_on_level {
            // TODO
            godot_print!("all mob squashed!");

            //* Для визуального вывода, ТОЛЬКО ДЛЯ РАЗРАБОТКИ!
            let mut label = self
                .base()
                .get_node_as::<MainMenu>("UserInterface")
                .get_node_as::<Label>("Improvement");

            label.show();
            label.set_text("End of level!");
        }
    }

    /// Авто оживление игрока.
    // ! ТОЛЬКО ДЛЯ РАЗРАБОТКИ!
    fn on_player_hit(&mut self) {
        self.base()
            .get_node_as::<Player>("Player")
            .bind_mut()
            .alive();

        // Оживляет всех мобов.
        for i in 0..self.all_mobs_on_level {
            let mob = self.base().try_get_node_as::<Mob>(&format!("Mobs/Mob{i}"));

            if let Some(mut mob) = mob {
                mob.bind_mut().alive();
            }
        }

        self.squashed_mobs = 0;

        self.base()
            .get_node_as::<MainMenu>("UserInterface")
            .get_node_as::<Label>("Improvement")
            .hide();
    }
}

// functions providing a string API for loading data from a scene

/// Get mob path in scene with respecting the hierarchy of names.
/// Return path to 'Mob' class in scene.
#[inline(always)]
fn get_text_mob_name(mob_num: i64) -> String {
    format!("Mobs/Mob{mob_num}")
}

/// Get mob path in scene with respecting the hierarchy of names.
/// Return path to 'PathFollow3D' class in scene.
#[inline(always)]
fn get_text_mob_follow_path(mob_num: i64) -> String {
    format!("Mobs/Mob{mob_num}Path/PathFollow3D")
}

/// Get mob path in scene with respecting the hierarchy of names.
/// Return path to 'Path3D' class in scene.
#[inline(always)]
fn get_text_mob_path(mob_num: i64) -> String {
    format!("Mobs/Mob{mob_num}Path")
}
