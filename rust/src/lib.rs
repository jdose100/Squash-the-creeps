//! Данная библиотека содержит реализацию игры `Deeps-Creeps`. 
//! Подробности архитектуры игры можно найти в файле `architecture`,
//! который открывается сервисом Draw.IO
use godot::prelude::{ExtensionLibrary, gdextension};

mod config;
mod levels;
mod main_scene;
mod mob;
mod player;
mod ui;

struct SquashTheCreeps;
#[gdextension]
unsafe impl ExtensionLibrary for SquashTheCreeps {}
