//! In this file stored all logic, classes, struct and enums for Main scene.

use crate::{player::Player, ui::UserInterface};
use godot::{
    classes::{AudioServer, AudioStream, AudioStreamPlayer, HSlider, Marker3D},
    obj::WithBaseField,
    prelude::*,
};

/// This enum store all improvements for player.
/// He need for update UI.
pub enum Improvements {
    None,
}

/// This class store logic for Main scene.
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
        // connect signal for setup music volume
        self.base()
            .get_node_as::<UserInterface>("UserInterface")
            .get_node_as::<HSlider>("SettingsHUD/SoundSettings/MusicVolumeHSlider")
            .signals()
            .value_changed()
            .connect_obj(self, Self::on_music_volume_slider_changed);

        // connect signal for setup sound effects volume
        self.base()
            .get_node_as::<UserInterface>("UserInterface")
            .get_node_as::<HSlider>("SettingsHUD/SoundSettings/SoundEffectsVolumeHSlider")
            .signals()
            .value_changed()
            .connect_obj(self, Self::on_sound_effects_volume_slider_changed);

        // start music
        self.base()
            .get_node_as::<AudioStreamPlayer>("BackgroundMusic")
            .play();
    }

    fn process(&mut self, _delta: f64) {
    }
}

#[godot_api]
impl MainScene {
    /// Change music volume from HSlider.
    fn on_music_volume_slider_changed(&mut self, volume_db: f64) {
        // get audio server
        let mut audio_server = AudioServer::singleton();

        // set new sound volume
        let bus_idx = audio_server.get_bus_index("Music");
        audio_server.set_bus_volume_db(bus_idx, volume_db as f32);
    }

    /// Change sound effects volume from HSlider
    fn on_sound_effects_volume_slider_changed(&mut self, volume_db: f64) {
        // get audio server
        let mut audio_server = AudioServer::singleton();

        // set new sound volume
        let bus_idx = audio_server.get_bus_index("SoundEffects");
        audio_server.set_bus_volume_db(bus_idx, volume_db as f32);
    }
}
