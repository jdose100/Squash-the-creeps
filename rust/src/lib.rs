use godot::prelude::{ExtensionLibrary, gdextension};

mod levels;
mod main_scene;
mod mob;
mod player;
mod ui;

struct SquashTheCreeps;
#[gdextension]
unsafe impl ExtensionLibrary for SquashTheCreeps {}
