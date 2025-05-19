//! В данном файле находится логика работы UI с настройками игры. 
//! Данный UI может менять настройки игры, но ответственность за
//! сохранение настроек несет MainScene.

use godot::{classes::{AudioServer, Button, ColorRect, HSlider, IColorRect}, obj::WithBaseField, prelude::*};
use crate::config::CONFIG;

use super::translation::*;

/// Реализация интерфейса настроек.
#[derive(GodotClass)]
#[class(init, base = ColorRect)]
pub(crate) struct SettingsHUD { 
    /// Указывает были ли изменены настройки.
    settings_changed: bool,

    base: Base<ColorRect>,
}

/// Данный impl блок содержит реализацию виртуальных
/// методов базового класса.
#[godot_api]
impl IColorRect for SettingsHUD {
    fn ready(&mut self) {
        // * Обновляем положения HSlider.
        self
            .base()
            .get_node_as::<HSlider>("SoundSettings/MusicVolumeHSlider")
            .set_value(CONFIG.lock().unwrap().settings.music_volume);

        self
            .base()
            .get_node_as::<HSlider>("SoundSettings/SoundEffectsVolumeHSlider")
            .set_value(CONFIG.lock().unwrap().settings.sound_effects_volume);

        // * Подключаем кнопки для переключения языка.
        self
            .base()
            .get_node_as::<Button>("LanguageSettings/SetLanguageRU")
            .signals()
            .pressed()
            .connect_obj(self, |this: &mut Self| { 
                CONFIG.lock().unwrap().set_language(Languages::RU);
                this.settings_changed = true;
            });

        self
            .base()
            .get_node_as::<Button>("LanguageSettings/SetLanguageEN")
            .signals()
            .pressed()
            .connect_obj(self, |this: &mut Self| {
                CONFIG.lock().unwrap().set_language(Languages::EN);
                this.settings_changed = true;
            });

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
            .connect_obj(self, Self::close_settings);

        // * Сигналы для настройки звука.
        self
            .base()
            .get_node_as::<HSlider>("SoundSettings/MusicVolumeHSlider")
            .signals()
            .value_changed()
            .connect_obj(self, Self::on_music_volume_slider_changed);
        
        self
            .base()
            .get_node_as::<HSlider>("SoundSettings/SoundEffectsVolumeHSlider")
            .signals()
            .value_changed()
            .connect_obj(self, Self::on_sound_effects_volume_slider_changed);
    } 
}

/// Данный impl блок содержит описание сигналов.
#[godot_api]
impl SettingsHUD {
    #[signal]
    /// Сигнал указывающий что меню настроек было закрыто.
    pub fn settings_closed();
}

/// Данный impl блок содержит реализацию методов класса SettingsHUD.
impl SettingsHUD { 
    /// Закрывает все открытые меню настроек.
    pub fn close_all_settings_menus(&self) {
        self.base().get_node_as::<Node2D>("LanguageSettings").hide();
        self.base().get_node_as::<Node2D>("SoundSettings").hide();
    }

    /// Данная функция закрывает меню настроек.
    pub fn close_settings(&mut self) {
        if self.settings_changed {
            CONFIG.lock().unwrap().write();
            self.settings_changed = false;
        }

        self.base_mut().hide();
        self.signals().settings_closed().emit();
    }

    // * Логика для настроек звука.

    /// Открываем меню настроек звука.
    pub fn open_sound_setting_menu(&mut self) {
        self.close_all_settings_menus();
        self.base().get_node_as::<Node2D>("SoundSettings").show();
    }

    /// Изменяет громкость музыки, данные получаются от HSlider.
    fn on_music_volume_slider_changed(&mut self, volume_db: f64) {
        let mut audio_server = AudioServer::singleton();
        let bus_idx = audio_server.get_bus_index("Music");
        audio_server.set_bus_volume_db(bus_idx, volume_db as f32);

        CONFIG.lock().unwrap().settings.music_volume = volume_db;
        self.settings_changed = true;
    }

    /// Изменяет громкость звуковых эффектов, данные получаются от HSlider.
    fn on_sound_effects_volume_slider_changed(&mut self, volume_db: f64) {
        let mut audio_server = AudioServer::singleton();
        let bus_idx = audio_server.get_bus_index("SoundEffects");
        audio_server.set_bus_volume_db(bus_idx, volume_db as f32);

        CONFIG.lock().unwrap().settings.sound_effects_volume = volume_db;
        self.settings_changed = true;
    }

    // * Логика для настроек языка.

    /// Открывает меню настроек языка.
    pub fn open_language_setting_menu(&mut self) {
        self.close_all_settings_menus();
        self.base().get_node_as::<Node2D>("LanguageSettings").show();
    }

    /// Открываем меню настроек.
    pub fn open_settings_menu(&mut self) {
        self.base_mut().show();
        self.close_all_settings_menus();
    }
}
