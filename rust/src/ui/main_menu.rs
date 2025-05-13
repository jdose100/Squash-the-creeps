//! Данный модуль содержит логику для UI главного меню.

use super::{settings::SettingsHUD, translation::*};
use godot::{
    classes::{Button, ColorRect, IColorRect, Label},
    obj::WithBaseField,
    prelude::*,
};

/// Реализация UI главного меню.
#[derive(GodotClass)]
#[class(base = ColorRect)]
pub struct MainMenu {
    /// Язык, который сейчас используется.
    pub current_language: &'static LanguageText,

    base: Base<ColorRect>,
}

#[godot_api]
impl IColorRect for MainMenu {
    fn init(base: Base<ColorRect>) -> Self { 
        Self {
            current_language: &RU_LANGUAGE,
            base,
        }
    }

    fn ready(&mut self) {
        self.update_text_from_language();

        // Настраиваем меню настроек.
        let gd = self.to_gd();
        self.
            base()
            .get_node_as::<SettingsHUD>("SettingsHUD")
            .bind_mut()
            .connect_main_menu_signals(&gd);

        // Устанавливаем сигнал для открытия меню настроек.
        self
            .base()
            .get_node_as::<Button>("SettingsButton")
            .signals()
            .pressed()
            .connect_obj(self, Self::open_settings_menu);

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

#[godot_api]
impl MainMenu {
    /// Настраиваем интерфейс на новый язык.
    pub fn update_text_from_language(&mut self) {
        let language = self.current_language;

        self
            .base()
            .get_node_as::<SettingsHUD>("SettingsHUD")
            .bind_mut()
            .update_text_from_language(language);

        // * Обновляет MainHUD.
        self.base().get_node_as::<Button>("StartButton").set_text(language.start_button);
        self.base().get_node_as::<Button>("SettingsButton").set_text(language.settings);

        // * Обновляет LevelHUD.
        self.base().get_node_as::<Button>("LevelHUD/ExitButton").set_text(language.exit);
        self.base().get_node_as::<Label>("LevelHUD/MenuName").set_text(language.levels);
    }

    /// Настраивает меню для старта игры.
    pub fn start_new_game(&mut self) {
        self.base_mut().hide();
    } 

    /// Закрывает меню настроек и открывает главное меню.
    pub fn close_settings_menu(&mut self) {
        self.base().get_node_as::<SettingsHUD>("SettingsHUD").bind_mut().close_settings_menu();
        self.show();
    }

    /// Открывает меню настроек и закрывает главное меню.
    fn open_settings_menu(&mut self) {
        self.hide();

        let mut settings = self.base().get_node_as::<SettingsHUD>("SettingsHUD");
        settings.bind_mut().open_settings_menu();
        settings.bind_mut().close_all_settings_menus();
    }

    /// Закрывает меню уровней и открывает главное меню.
    fn close_level_menu(&mut self) {
        self.base().get_node_as::<ColorRect>("LevelHUD").hide();
        self.show();
    }
    
    /// Открывает меню уровней и закрывает главное меню.
    fn open_level_menu(&mut self) {
        self.base().get_node_as::<ColorRect>("LevelHUD").show(); 
        self.hide();
    }

    /// Делает меню невидимым.
    fn hide(&self) {
        self.base().get_tree().unwrap().call_group("main_menu", "hide", &[]);
    }

    /// Делает меню видимым.
    fn show(&self) {
        self.base().get_tree().unwrap().call_group("main_menu", "show", &[]);
    }
}
