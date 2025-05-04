//! In this file stored all logic, classes, struct and enums for Main scene.

use crate::{mob::Mob, player::Player, ui::UserInterface};
use godot::{
    classes::{
        Marker3D, object::ConnectFlags, AudioStreamPlayer, Button, ColorRect, 
        MeshInstance3D, PathFollow3D, Timer
    }, 
    global::{randf, randf_range, randi}, 
    obj::{WithBaseField, WithUserSignals}, 
    prelude::*
};

/// This enum store all improvements for player.
/// He need for update UI.
pub enum Improvements {
    SlowCreeps,
    PlayerShield,
    None
}

/// This class store logic for Main scene.
#[derive(GodotClass)]
#[class(init, base = Node)]
struct MainScene {
    /// this field contain Mob scene for spawn creeps
    #[init(val = OnReady::from_loaded("res://scenes/mob.tscn"))]
    mob_scene: OnReady<Gd<PackedScene>>,

    #[init(val = 1.0)] /// deceleration factor for creeps
    slow_creeps: f64, 

    base: Base<Node>
}

#[godot_api] impl INode for MainScene {
    fn ready(&mut self) {
        // connect 'timeout' signal to spawn new mobs
        self.base().get_node_as::<Timer>("MobTimer").signals().timeout()
            .connect_obj(self, Self::on_mob_timer_timeout);

        // connect 'pressed' signal from StartButton to start new game
        self.base().get_node_as::<Button>(
            "UserInterface/MainHUD/StartButton"
        ).signals().pressed()
            .connect_obj(self, Self::on_start_button_pressed);

        // connect 'hit' signal from Player
        self.base().get_node_as::<Player>("Player").bind_mut()
            .signals().hit().connect_obj(self, Self::on_player_hit);

        // show hud
        self.base().get_node_as::<ColorRect>("UserInterface/MainHUD").show();
        
        // start music
        self.base().get_node_as::<AudioStreamPlayer>("BackgroundMusic").play();
    }

    fn process(&mut self, _delta: f64) {
        // get player position
        let mut player_position = 
            self.base().get_node_as::<Player>("Player")
            .get_position();
        player_position.y = 0.0;

        // move camera
        self.base().get_node_as::<Marker3D>("CameraPivot").set_position(
            player_position
        );
    }
}

#[godot_api] impl MainScene {
    /// Create mobs on timer timeout.
    fn on_mob_timer_timeout(&mut self) {
        // create a new mob
        let mut mob = self.mob_scene.instantiate_as::<Mob>();

        // set slowdown
        mob.bind_mut().slowdown = self.slow_creeps;

        // chose a random location on the SpawnLocation
        let mut mob_spawn_location = 
            self.base().get_node_as::<PathFollow3D>("SpawnPath/SpawnLocation");
        mob_spawn_location.set_progress_ratio(randf() as f32);

        // get player position
        let player_position = 
            self.base().get_node_as::<Player>("Player").get_position();
        
        // initialize mob
        mob.bind_mut().initialize(
            mob_spawn_location.get_position(), 
            player_position
        );

        // we connect the mob to the score label to update the score
        // upon squashing one
        mob.signals().squashed().connect_builder()
            .object(&self.to_gd())
            .method_mut(Self::on_mob_squashed)
            .flags(ConnectFlags::DEFERRED)
            .done();

        // spawn the mob by adding it to the Main scene
        self.base_mut().add_child(&mob);
    }

    /// Update score if mob squashed and activate improvements if need.
    fn on_mob_squashed(&mut self) {
        // update score
        self.base().get_node_as::<UserInterface>("UserInterface")
            .bind_mut().on_mob_squashed();

        // add improvement if need
        let shield_active = 
            self.base().get_node_as::<Player>("Player")
            .bind().shield_active;

        if self.slow_creeps == 1.0 && !shield_active {
            if randf_range(-7.0, 20.0) >= 13.4 {
                // active time of improvement is secs
                let active_time: f64;

                // 50% chance to get slowdown and 50% chance to get a shield
                if randi() % 2 == 0 {
                    // set improvement
                    self.slow_creeps = 1.4;
                    active_time = 20.0;

                    // update UI
                    self.base().get_node_as::<UserInterface>("UserInterface")
                        .bind_mut()
                        .set_improvement(Improvements::SlowCreeps);
                } else {
                    // set visual improvement
                    self.base().get_node_as::<MeshInstance3D>(
                        "Player/Pivot/Shield"
                    ).show();

                    // set improvement
                    self.base().get_node_as::<Player>("Player").bind_mut()
                        .shield_active = true;

                    // set active time
                    active_time = 10.0;

                    // update UI
                    self.base().get_node_as::<UserInterface>("UserInterface")
                        .bind_mut()
                        .set_improvement(Improvements::PlayerShield);
                }

                // create timer for disable improvements
                self.base().get_tree().unwrap()
                    .create_timer(active_time).unwrap()
                    .signals().timeout().connect_obj(
                        self, Self::on_improvement_timer_timeout
                    );
            }
        }
    }

    /// Disable all improvements on timeout.
    fn on_improvement_timer_timeout(&mut self) {
        // deactivate all improvements
        self.slow_creeps = 1.0; 
        self.base().get_node_as::<Player>("Player").bind_mut()
            .shield_active = false;

        // hide player shield
        self.base().get_node_as::<MeshInstance3D>("Player/Pivot/Shield").hide();

        // hide improvement on UI
        self.base().get_node_as::<UserInterface>("UserInterface")
            .bind_mut().set_improvement(Improvements::None);
    }

    /// Activate MainHUD if player die and deactivate all improvements.
    fn on_player_hit(&mut self) {
        self.base().get_node_as::<ColorRect>("UserInterface/MainHUD").show();
    }

    /// Start game if 'StartButton' pressed.
    fn on_start_button_pressed(&mut self) {
        self.base().get_node_as::<AudioStreamPlayer>(
            "UserInterface/ClickSound"
        ).play();

        self.new_game();
    }

    /// Start new game.
    fn new_game(&mut self) {
        // delete all mobs
        self.base().get_tree().unwrap().call_group(
            "mob", "queue_free", &[]
        );

        // disable all improvements
        self.on_improvement_timer_timeout();

        // alive player
        self.base().get_node_as::<Player>("Player").bind_mut().alive();

        // set UI to start new game
        self.base().get_node_as::<UserInterface>("UserInterface")
            .bind_mut().start_new_game();
    }
}
