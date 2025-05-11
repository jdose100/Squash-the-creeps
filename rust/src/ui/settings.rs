//! Store a logic for settings UI menu.
use godot::{classes::Control, prelude::*};

/// Implementation for settings UI menu.
#[derive(GodotClass)]
#[class(init, internal, base = Node)]
pub(super) struct SettingsUI {
    /// Need for GodotClass, is ignored!
    _base: Base<Node>,

    /// Real base for this struct.
    pub base: Option<Gd<Control>>,
}

impl SettingsUI {
    /// Return base for this struct.
    fn base(&self) -> &Gd<Control> {
        self.base.as_ref().unwrap()
    }

    /// Close all settings menu.
    pub fn close_settings_menus(&mut self) {
        // close language settings
        self.base().get_node_as::<Node2D>("SettingsHUD/LanguageSettings").hide();

        // close sound settings
        self.base().get_node_as::<Node2D>("SettingsHUD/SoundSettings").hide();
    }

    // * Logic for 'Sound Settings'.

    /// Open sound settings menu.
    pub fn on_sound_settings_button_pressed(&mut self) {
        // close other opened menus
        self.close_settings_menus();

        // open sound menu
        self.base().get_node_as::<Node2D>("SettingsHUD/SoundSettings").show();
    }

    // * Logic for 'Language Settings'.

    /// Open language settings menu.
    pub fn on_language_settings_button_pressed(&mut self) {
        // close other opened menus
        self.close_settings_menus();

        // open language menu
        self.base().get_node_as::<Node2D>("SettingsHUD/LanguageSettings").show();
    }

    /// Set RU language if button pressed.
    pub fn on_ru_language_button_pressed(ui: &mut super::UserInterface) {
        // set new language
        ui.current_language = &super::RU_LANGUAGE; 

        // update UI to new language
        ui.update_text_from_language();
    }

    /// Set EN language if button pressed
    pub fn on_en_language_button_pressed(ui: &mut super::UserInterface) {
        // set new language
        ui.current_language = &super::EN_LANGUAGE;

        // update UI to new language
        ui.update_text_from_language();
    }
}
