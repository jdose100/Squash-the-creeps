//! В данном файле содержится логика для работы Меню Паузы.

use godot::{classes::{Button, ColorRect, Label}, prelude::*};

/// Этот класс содержит реализацию меню паузы.
#[derive(GodotClass)]
#[class(init, base = ColorRect)]
pub struct PauseMenu {
    base: Base<ColorRect>
}

impl PauseMenu {
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

