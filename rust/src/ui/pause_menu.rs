//! В данном файле содержится логика для работы Меню Паузы.

use godot::{classes::{Button, ColorRect, Label}, prelude::*};

use super::translation::LanguageText;

/// Этот класс содержит реализацию меню паузы.
#[derive(GodotClass)]
#[class(init, base = ColorRect)]
pub struct PauseMenu {
    base: Base<ColorRect>
}

impl PauseMenu {
    /// Переводит меню на новый язык.
    pub fn translate(&mut self, language: &LanguageText) {
        self
            .base()
            .get_node_as::<Label>("NameOfMenu")
            .set_text(language.pause_menu);

        self
            .base()
            .get_node_as::<Button>("ContinueButton")
            .set_text(language.continue_button);

        self
            .base()
            .get_node_as::<Button>("RestartButton")
            .set_text(language.restart);

        self
            .base()
            .get_node_as::<Button>("SettingsButton")
            .set_text(language.settings);

        self
            .base()
            .get_node_as::<Button>("ExitButton")
            .set_text(language.exit);
    }

    pub fn hide(&self) {
        self.base().get_node_as::<Label>("NameOfMenu").hide();

        self.base().get_node_as::<Button>("ContinueButton").hide();
        self.base().get_node_as::<Button>("RestartButton").hide();
        self.base().get_node_as::<Button>("SettingsButton").hide();
        self.base().get_node_as::<Button>("ExitButton").hide();
    }
    
    pub fn show(&self) {
        self.base().get_node_as::<Label>("NameOfMenu").show();

        self.base().get_node_as::<Button>("ContinueButton").show();
        self.base().get_node_as::<Button>("RestartButton").show();
        self.base().get_node_as::<Button>("SettingsButton").show();
        self.base().get_node_as::<Button>("ExitButton").show();
    }
}
