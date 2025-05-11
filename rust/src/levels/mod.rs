//! This module store all logic for levels and 'BaseLevel' class.

// import decencies
use crate::{mob::Mob, player::Player, ui::UserInterface};
use godot::{
    classes::{Label, Marker3D, Path3D, PathFollow3D},
    global::randf_range,
    obj::{BaseRef, WithBaseField},
    prelude::*,
};

// modules for external level logic
mod level2;

/// Base logic for levels.
#[derive(GodotClass)]
#[class(base = Node)]
struct BaseLevel {
    /// How much mobs on level.
    #[export]
    all_mobs_on_level: i64,

    /// How much mobs squashed.
    squashed_mobs: i64,

    base: Base<Node>,
}

#[godot_api]
impl INode for BaseLevel {
    fn init(base: Base<Node>) -> Self {
        Self {
            all_mobs_on_level: 0,
            squashed_mobs: 0,
            base,
        }
    }

    fn ready(&mut self) {
        // init this node
        Self::init_node(&self.base());

        // init all mobs
        Self::default_mobs_init(
            &self.base(),
            self.all_mobs_on_level,
            |this, mob| {
                mob.signals()
                    .squashed()
                    .connect_obj(this, Self::on_mob_squashed);
            }
        );

        // connect 'hit' signal from player
        self.base()
            .get_node_as::<Player>("Player")
            .signals()
            .hit()
            .connect_obj(self, Self::on_player_hit);
    }

    fn process(&mut self, _delta: f64) {
        self.base()
            .get_node_as::<Marker3D>("CameraPivot")
            .set_position(self.base().get_node_as::<Player>("Player").get_position());
    }
}

#[godot_api]
impl BaseLevel {
    /// Setup player and IU with BaseLevel parameters, need for delete
    /// boilerplate code in child classes.
    pub fn init_node<T: GodotClass + INode>(node: &BaseRef<T>) {
        // setup player if is exists
        if let Some(mut player) = node.try_get_node_as::<Player>("Player") {
            // alive player
            player.bind_mut().alive();
        }

        // setup UI if is exists
        if let Some(mut ui) = node.try_get_node_as::<UserInterface>("UserInterface") {
            ui.bind_mut().start_new_game();
        }
    }

    /// Init and call 'method' for all mobs in scene specified by the counter.
    pub fn default_mobs_init<T, F>(node: &BaseRef<T>, mob_count: i64, method: F)
    where
        T: GodotClass + INode + Inherits<Node>,
        F: Fn(&Gd<T>, &mut Gd<Mob>),
    {
        // cycle of all mobs in mob_count
        for i in 0..mob_count {
            // get mob from scene
            let mob = node.try_get_node_as::<Mob>(&get_text_mob_name(i));

            // check mob is exits or not
            if let Some(mut mob) = mob {
                // if mob exits then connect signal
                method(&node.to_godot().cast(), &mut mob);

                //* The following code is needed to be able to move along a given path!

                // get follow path
                let follow_path =
                    node.try_get_node_as::<PathFollow3D>(&get_text_mob_follow_path(i));

                // get path
                let path = node.try_get_node_as::<Path3D>(&get_text_mob_path(i));

                // get speed
                let follow_speed = randf_range(0.1, 0.34);

                // init mob
                mob.bind_mut().initialize(follow_path, path, follow_speed);
            }
        }
    }

    /// Update 'squashed_mobs' on mob squashed and play sound if all mobs in level squashed
    fn on_mob_squashed(&mut self) {
        self.squashed_mobs += 1;

        if self.squashed_mobs == self.all_mobs_on_level {
            // TODO
            godot_print!("all mob squashed!");

            //* for visual output, only on develop!
            let mut label = self
                .base()
                .get_node_as::<UserInterface>("UserInterface")
                .get_node_as::<Label>("Improvement");

            label.show();
            label.set_text("End of level!");
        }
    }

    /// auto alive for player,
    // ! ONLY FOR DEVELOP!
    fn on_player_hit(&mut self) {
        self.base()
            .get_node_as::<Player>("Player")
            .bind_mut()
            .alive();

        // alive all mobs
        for i in 0..self.all_mobs_on_level {
            let mob = self.base().try_get_node_as::<Mob>(&format!("Mobs/Mob{i}"));

            if let Some(mut mob) = mob {
                mob.bind_mut().alive();
            }
        }

        // set squashed mobs to zero
        self.squashed_mobs = 0;

        // hide label
        self.base()
            .get_node_as::<UserInterface>("UserInterface")
            .get_node_as::<Label>("Improvement")
            .hide();
    }
}

// functions providing a string API for loading data from a scene

/// Get mob path in scene with respecting the hierarchy of names.
/// Return path to 'Mob' class in scene.
#[inline(always)]
fn get_text_mob_name(mob_num: i64) -> String {
    format!("Mobs/Mob{mob_num}")
}

/// Get mob path in scene with respecting the hierarchy of names.
/// Return path to 'PathFollow3D' class in scene.
#[inline(always)]
fn get_text_mob_follow_path(mob_num: i64) -> String {
    format!("Mobs/Mob{mob_num}Path/PathFollow3D")
}

/// Get mob path in scene with respecting the hierarchy of names.
/// Return path to 'Path3D' class in scene.
#[inline(always)]
fn get_text_mob_path(mob_num: i64) -> String {
    format!("Mobs/Mob{mob_num}Path")
}
