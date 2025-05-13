//! В данном файле находится логика работы UI с настройками игры. 
//! Данный UI может менять настройки игры, но ответственность за
//! сохранение настроек несет MainScene.

use godot::{classes::{AudioServer, Button, ColorRect, HSlider, Label}, prelude::*};
use super::{main_menu::MainMenu, translation::*};

/// Реализация интерфейса настроек.
#[derive(GodotClass)]
#[class(init, base = ColorRect)]
pub(super) struct SettingsHUD {
    base: Base<ColorRect>,
}

impl SettingsHUD { 
    pub fn connect_main_menu_signals(&mut self, main: &Gd<MainMenu>) {
        // * Подключаем кнопки для переключения языка.
        self
            .base()
            .get_node_as::<Button>("LanguageSettings/SetLanguageRU")
            .signals()
            .pressed()
            .connect_obj(main, Self::set_ru_language);

        self
            .base()
            .get_node_as::<Button>("LanguageSettings/SetLanguageEN")
            .signals()
            .pressed()
            .connect_obj(main, Self::set_en_language);

        // * Сигналы для открытия разных меню.
        self
            .base()
            .get_node_as::<Button>("LanguageSettingsButton")
            .signals()
            .pressed()
            .connect_obj(self, Self::open_language_setting_menu);
        
        self
            .base()
            .get_node_as::<Button>("SoundSettingsButton")
            .signals()
            .pressed()
            .connect_obj(self, Self::open_sound_setting_menu);

        // * Сигнал для закрытия меню.
        self
            .base()
            .get_node_as::<Button>("ExitButton")
            .signals()
            .pressed()
            .connect_obj(main, MainMenu::close_settings_menu);

        // * Сигналы для настройки звука.
        self
            .base()
            .get_node_as::<HSlider>("SoundSettings/MusicVolumeHSlider")
            .signals()
            .value_changed()
            .connect(Self::on_music_volume_slider_changed);
        
        self
            .base()
            .get_node_as::<HSlider>("SoundSettings/SoundEffectsVolumeHSlider")
            .signals()
            .value_changed()
            .connect(Self::on_sound_effects_volume_slider_changed);
    }

    /// Переводит данный интерфейс на новый язык.
    pub fn update_text_from_language(&self, language: &LanguageText) {
        // * Обновляет настройки языка на новый язык.
        self
            .base()
            .get_node_as::<Button>("LanguageSettingsButton")
            .set_text(language.language_button);

        self
            .base()
            .get_node_as::<Label>("LanguageSettings/SetLanguageLabel")
            .set_text(language.set_current_language);

        self
            .base()
            .get_node_as::<Label>("LanguageSettings/CurrentLanguageLabel")
            .set_text(
                &format!("{}: {}", language.current_language, language.language.to_string())
            );
        
        // * Обновляет настройки звука на новый язык.
        self
            .base()
            .get_node_as::<Button>("SoundSettingsButton")
            .set_text(language.sound_button);

        self
            .base()
            .get_node_as::<Label>("SoundSettings/MusicVolumeLabel")
            .set_text(language.music_volume);

        self
            .base()
            .get_node_as::<Label>("SoundSettings/SoundEffectsVolumeLabel")
            .set_text(language.sound_effect_volume);

        // * Обновляем название меню и кнопку выходу на новый язык.
        self
            .base()
            .get_node_as::<Label>("MenuName")
            .set_text(language.settings);

        self
            .base()
            .get_node_as::<Button>("ExitButton")
            .set_text(language.exit)
    }

    /// Закрывает все открытые меню настроек.
    pub fn close_all_settings_menus(&self) {
        self.base().get_node_as::<Node2D>("LanguageSettings").hide();
        self.base().get_node_as::<Node2D>("SoundSettings").hide();
    }

    // * Логика для настроек звука.

    /// Открываем меню настроек звука.
    pub fn open_sound_setting_menu(&mut self) {
        self.close_all_settings_menus();
        self.base().get_node_as::<Node2D>("SoundSettings").show();
    }

    /// Изменяет громкость музыки, данные получаются от HSlider.
    fn on_music_volume_slider_changed(volume_db: f64) {
        let mut audio_server = AudioServer::singleton();
        let bus_idx = audio_server.get_bus_index("Music");
        audio_server.set_bus_volume_db(bus_idx, volume_db as f32);
    }

    /// Изменяет громкость звуковых эффектов, данные получаются от HSlider.
    fn on_sound_effects_volume_slider_changed(volume_db: f64) {
        let mut audio_server = AudioServer::singleton();
        let bus_idx = audio_server.get_bus_index("SoundEffects");
        audio_server.set_bus_volume_db(bus_idx, volume_db as f32);
    }

    // * Логика для настроек языка.

    /// Открывает меню настроек языка.
    pub fn open_language_setting_menu(&mut self) {
        self.close_all_settings_menus();
        self.base().get_node_as::<Node2D>("LanguageSettings").show();
    }

    /// Устанавливает язык на Русский.
    pub fn set_ru_language(ui: &mut MainMenu) {
        ui.current_language = &RU_LANGUAGE; 
        ui.update_text_from_language();
    }

    /// Устанавливает язык на Английский.
    pub fn set_en_language(ui: &mut MainMenu) {
        ui.current_language = &EN_LANGUAGE;
        ui.update_text_from_language();
    }

    /// Открываем меню настроек.
    pub fn open_settings_menu(&mut self) {
        self.base_mut().show();
    }

    /// Закрывает меню настроек.
    pub fn close_settings_menu(&mut self) {
        self.base_mut().hide();
    }
}
