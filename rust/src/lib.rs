use godot::prelude::*;

mod main_scene;
mod player;
mod levels;
mod mob;
mod ui;

struct SquashTheCreeps;
#[gdextension] unsafe impl ExtensionLibrary for SquashTheCreeps {}
