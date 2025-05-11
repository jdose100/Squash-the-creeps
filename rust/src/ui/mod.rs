//! This module store a logic for UI (User Interface).

use std::sync::LazyLock;
use godot::{
    classes::{Button, ColorRect, Control, IControl, Label},
    obj::WithBaseField,
    prelude::*,
};

mod play_menu;
mod settings;

/// This enum store of all type of languages.
#[derive(Default)]
enum Languages {
    #[default]
    EN,
    RU,
}

/// This struct store a text from translation.
#[allow(dead_code)]
struct LanguageText {
    language: Languages,
    // * MainHUD
    name_of_game: &'static str,
    start_button: &'static str,
    settings: &'static str,

    // * SettingsHUD/Language
    language_button: &'static str,
    set_current_language: &'static str,
    current_language: &'static str,

    // * SettingsHUD/Sound
    sound_button: &'static str,
    music_volume: &'static str,
    sound_effect_volume: &'static str,
}

/// This variable store translation to English.
static EN_LANGUAGE: LazyLock<LanguageText> = LazyLock::new(|| LanguageText {
    language: Languages::EN,
    // * MainHUD
    name_of_game: "Squash the creeps!",
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
});

/// This variable store translation to Russian.
static RU_LANGUAGE: LazyLock<LanguageText> = LazyLock::new(|| LanguageText {
    language: Languages::RU,
    // * MainHUD
    name_of_game: "Раздави жуть!",
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
});

/// This class store a UI data.
#[derive(GodotClass)]
#[class(base = Control)]
pub struct UserInterface {
    /// The current language.
    current_language: &'static LanguageText,

    /// The 
    settings: Gd<settings::SettingsUI>,

    base: Base<Control>,
}

#[godot_api]
impl IControl for UserInterface {
    fn init(base: Base<Control>) -> Self { 
        Self {
            current_language: &RU_LANGUAGE,
            settings: settings::SettingsUI::new_alloc(),
            base,
        }
    }

    fn ready(&mut self) {
        // init self.settings
        self.settings.bind_mut().base = Some(self.base().clone());

        // setup UI to start language
        // self.update_text_from_language();

        // * Settings Menu setup.

        // * Connect change language signals to SettingsHUD/LanguageSettings buttons.
        // connect RU language
        self
            .base()
            .get_node_as::<Button>("SettingsHUD/LanguageSettings/SetLanguageRU")
            .signals()
            .pressed()
            .connect_obj(self, settings::SettingsUI::on_ru_language_button_pressed);

        // connect EN language
        self
            .base()
            .get_node_as::<Button>("SettingsHUD/LanguageSettings/SetLanguageEN")
            .signals()
            .pressed()
            .connect_obj(self, settings::SettingsUI::on_en_language_button_pressed);

        // connect signal for open language menu
        self
            .base()
            .get_node_as::<Button>("SettingsHUD/LanguageSettingsButton")
            .signals()
            .pressed()
            .connect_obj(&self.settings, settings::SettingsUI::on_language_settings_button_pressed);

        // * Connect signals for SettingsHUD/SoundSettings.
        self
            .base()
            .get_node_as::<Button>("SettingsHUD/SoundSettingsButton")
            .signals()
            .pressed()
            .connect_obj(&self.settings, settings::SettingsUI::on_sound_settings_button_pressed);
        
        // set signal for close settings menu
        self
            .base()
            .get_node_as::<Button>("SettingsHUD/ExitButton")
            .signals()
            .pressed()
            .connect_obj(self, Self::close_settings_menu);

        // set signal for open settings menu
        self
            .base()
            .get_node_as::<Button>("MainHUD/SettingsButton")
            .signals()
            .pressed()
            .connect_obj(self, Self::open_settings_menu);
    }
}

#[godot_api]
impl UserInterface {
    /// Setup interface to new language.
    fn update_text_from_language(&mut self) {
        let language = self.current_language;

        // * Update MainHUD.

        // update start button
        self.base()
            .get_node_as::<Button>("MainHUD/StartButton")
            .set_text(language.start_button);

        // update text logo
        self.base()
            .get_node_as::<Label>("MainHUD/NameOfGame")
            .set_text(language.name_of_game);

        // update settings button
        self.base()
            .get_node_as::<Button>("MainHUD/LanguageButton")
            .set_text(&format!("{}", language.language_button));

        // update positions for text
        match self.current_language.language {
            Languages::EN => self
                .base()
                .get_node_as::<Label>("MainHUD/NameOfGame")
                .set_position(Vector2::new(197.0, 190.0)),
            Languages::RU => self
                .base()
                .get_node_as::<Label>("MainHUD/NameOfGame")
                .set_position(Vector2::new(236.0, 192.0)),
        }
    }

    /// Setup UI to startup new game.
    pub fn start_new_game(&mut self) {
        // hide main hud
        self.base().get_node_as::<ColorRect>("MainHUD").hide();
    } 

    /// Close settings menu and open main menu.
    fn close_settings_menu(&mut self) {
        // close settings menu
        self
            .base()
            .get_node_as::<ColorRect>("SettingsHUD")
            .hide();

        // open main menu
        self
            .base()
            .get_node_as::<ColorRect>("MainHUD")
            .show();
    }

    /// Open settings menu and close main menu.
    fn open_settings_menu(&mut self) {
        // close main menu
        self
            .base()
            .get_node_as::<ColorRect>("MainHUD")
            .hide();

        // open settings menu
        self
            .base()
            .get_node_as::<ColorRect>("SettingsHUD")
            .show();
    }
}
