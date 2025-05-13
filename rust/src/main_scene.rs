//! В данном модуля расположена реализация `MainScene`.
//! Задачи и зоны ответственности `MainScene`:
//!     * Работа с файлами - отвечает за сохранение и чтение данных в файлах (сохранение прогресса/настроек).
//!     * Запуск уровней - отвечает за запуск уровней игры.
//!     * Взаимодействие с UI - взаимодействует с UI частью, предназначенной для `MainScene`.

use crate::ui::main_menu::MainMenu;
use godot::{
    classes::Button,
    obj::WithBaseField,
    prelude::*,
};

// Данный класс содержит реализацию логики работы `MainScene`.
#[derive(GodotClass)]
#[class(base = Node)]
struct MainScene {
    base: Base<Node>,
}

#[godot_api]
impl INode for MainScene {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        // Получаем пользовательский интерфейс.
        let main_menu = self.base().get_node_as::<MainMenu>("MainMenu");
        
        // * Подключаем сигналы UI для старта уровней.

        // Для 1 уровня.
        main_menu
            .get_node_as::<Button>("LevelHUD/Level1Button")
            .signals()
            .pressed()
            .connect_obj(self, |this: &mut Self| {
                this.base().get_node_as::<MainMenu>("MainMenu").bind_mut().start_new_game();
                this.base().get_tree().unwrap().change_scene_to_file("res://scenes/levels/level_1.tscn")
            });

        // Для 2 уровня.
        main_menu
            .get_node_as::<Button>("LevelHUD/Level2Button")
            .signals()
            .pressed()
            .connect_obj(self, |this: &mut Self| {
                this.base().get_node_as::<MainMenu>("MainMenu").bind_mut().start_new_game();
                this.base().get_tree().unwrap().change_scene_to_file("res://scenes/levels/level_2.tscn")
            });
    }
}

impl MainScene {}
