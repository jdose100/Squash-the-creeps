//! Данный файл содержит данные для перевода UI.

use serde_derive::{Deserialize, Serialize};
use std::sync::LazyLock;

/// В данном перечислении находятся все языки,
/// на которые переведена игра.
#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub enum Languages {
    EN,
    RU,
}

impl ToString for Languages {
    fn to_string(&self) -> String {
        match self {
            Languages::EN => return "EN".to_string(),
            Languages::RU => return "RU".to_string(),
        }
    }
}

/// В данной структуре находятся все поля, которые должны
/// быть представлены на языке.
#[allow(dead_code)]
pub struct LanguageText {
    pub language: Languages,
    pub exit: &'static str,
    pub levels: &'static str,

    // * MainHUD
    pub start_button: &'static str,
    pub settings: &'static str,

    // * SettingsHUD/Language
    pub language_button: &'static str,
    pub set_current_language: &'static str,
    pub current_language: &'static str,

    // * SettingsHUD/Sound
    pub sound_button: &'static str,
    pub music_volume: &'static str,
    pub sound_effect_volume: &'static str,

    // * PauseMenu
    pub pause_menu: &'static str,
    pub continue_button: &'static str,
    pub restart: &'static str,
}

/// В данной переменной хранится перевод текста на английский язык.
pub static EN_LANGUAGE: LazyLock<LanguageText> = LazyLock::new(|| LanguageText {
    language: Languages::EN,
    exit: "Exit",
    levels: "Levels",

    // * MainHUD
    start_button: "Play",
    settings: "Settings",

    // * SettingsHUD/Language
    language_button: "Language",
    set_current_language: "Set new language",
    current_language: "Current language",

    // * SettingsHUD/Sound
    sound_button: "Sound",
    music_volume: "Music volume",
    sound_effect_volume: "Sound effects volume",

    // * PauseMenu
    pause_menu: "Pause Menu",
    continue_button: "Continue",
    restart: "Restart"
});

/// В данной переменной хранится перевод на русский язык.
pub static RU_LANGUAGE: LazyLock<LanguageText> = LazyLock::new(|| LanguageText {
    language: Languages::RU,
    exit: "Выход",
    levels: "Уровни",

    // * MainHUD
    start_button: "Играть",
    settings: "Настройки",

    // * SettingsHUD/Language
    language_button: "Язык",
    set_current_language: "Выбрать другой язык",
    current_language: "Установленный язык",

    // * SettingsHUD/Sound
    sound_button: "Звук",
    music_volume: "Громкость музыки",
    sound_effect_volume: "Громкость звуковых эффектов",

    // * PauseMenu
    pause_menu: "Меню паузы",
    continue_button: "Продолжить",
    restart: "Заново"
});
