//! Данный модуль содержит логику для UI главного меню.


use super::settings::SettingsHUD;
use godot::{
    classes::{Button, ColorRect, IColorRect},
    obj::WithBaseField,
    prelude::*,
};

/// Реализация UI главного меню.
#[derive(GodotClass)]
#[class(base = ColorRect)]
pub struct MainMenu {
    base: Base<ColorRect>,
}

#[godot_api]
impl IColorRect for MainMenu {
    fn init(base: Base<ColorRect>) -> Self { 
        Self {
            base,
        }
    }

    fn ready(&mut self) {
        //* Настраиваем меню настроек.
        let mut settings = self.base().get_node_as::<SettingsHUD>("SettingsHUD");

        // Устанавливаем сигнал для открытия меню настроек.
        self
            .base()
            .get_node_as::<Button>("SettingsButton")
            .signals()
            .pressed()
            .connect_obj(self, Self::open_settings_menu);

        // Устанавливает сигнал для закрытия меню настроек.
        settings.signals().settings_closed().connect_obj(self, Self::close_settings_menu);

        // * Настраиваем меню уровней
        
        // Подключаем сигнал для выхода из меню уровней.
        self
            .base()
            .get_node_as::<Button>("LevelHUD/ExitButton")
            .signals()
            .pressed()
            .connect_obj(self, Self::close_level_menu);

        // Подключаем сигнал для входа в меню уровней.
        self
            .base()
            .get_node_as::<Button>("StartButton")
            .signals()
            .pressed()
            .connect_obj(self, Self::open_level_menu);
    }
}

impl MainMenu {
    /// Настраиваем интерфейс на новый язык.
    /// Настраивает меню для старта игры.
    pub fn start_new_game(&mut self) {
        self.base_mut().hide();
    } 

    /// Закрывает меню настроек и открывает главное меню.
    pub fn close_settings_menu(&mut self) {
        self.show_menu();
    }

    /// Открывает меню настроек и закрывает главное меню.
    fn open_settings_menu(&mut self) {
        self.hide_menu();

        self
            .base()
            .get_node_as::<SettingsHUD>("SettingsHUD")
            .bind_mut()
            .open_settings_menu();
    }

    /// Закрывает меню уровней и открывает главное меню.
    fn close_level_menu(&mut self) {
        self.base().get_node_as::<ColorRect>("LevelHUD").hide();
        self.show_menu();
    }
    
    /// Открывает меню уровней и закрывает главное меню.
    pub fn open_level_menu(&mut self) {
        self.base().get_node_as::<ColorRect>("LevelHUD").show(); 
        self.hide_menu();
    }

    /// Делает меню невидимым.
    fn hide_menu(&self) {
        self.base().get_tree().unwrap().call_group("main_menu", "hide", &[]);
    }

    /// Делает меню видимым.
    fn show_menu(&self) {
        self.base().get_tree().unwrap().call_group("main_menu", "show", &[]);
    }
}
