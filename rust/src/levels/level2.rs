use godot::{
    classes::{Marker3D, Path3D, PathFollow3D},
    global::randf_range,
    prelude::*,
};

use super::{BaseLevel, get_text_mob_follow_path, get_text_mob_name, get_text_mob_path};
use crate::{mob::Mob, player::Player, ui::UserInterface};

#[derive(GodotClass)]
#[class(base = Node)]
struct Level2 {
    /// How much mobs on level.
    #[export]
    all_mobs_on_level: i64,

    /// Real base class for logic.
    base_level: Gd<BaseLevel>,

    /// Base class for godot functions.
    base: Base<Node>,
}

#[godot_api]
impl INode for Level2 {
    fn init(base: Base<Node>) -> Self {
        Self {
            all_mobs_on_level: 0,
            base_level: BaseLevel::new_alloc(),
            base,
        }
    }

    fn ready(&mut self) {
        // init logic from base
        self.base_level.bind_mut().use_child_mob_init_logic = true;
        self.base_level.bind_mut().ready();

        // TODO
        // get player and ui
        let player = self.base().try_get_node_as::<Player>("Player");
        let ui = self.base().try_get_node_as::<UserInterface>("UserInterface");

        // setup player if is exists
        if let Some(mut player) = player {
            // alive player
            player.bind_mut().alive();
            
            self.base()
                .get_node_as::<Player>("Player")
                .signals()
                .hit()
                .connect_obj(self, Self::on_player_hit);
        }

        // setup ui if is exists
        if let Some(mut ui) = ui {
            ui.bind_mut().start_new_game();
        }


        // init all mobs
        for i in 0..self.all_mobs_on_level {
            // get mob from scene
            let mob = self.base().try_get_node_as::<Mob>(&get_text_mob_name(i));

            // check mob is exits or not
            if let Some(mut mob) = mob {
                // if mob exits then connect signal
                mob.signals()
                    .squashed()
                    .connect_obj(self, Self::on_mob_squashed);

                //* The following code is needed to be able to move along a given path!

                // get follow path
                let follow_path = self
                    .base()
                    .try_get_node_as::<PathFollow3D>(&get_text_mob_follow_path(i));

                // get path
                let path = 
                    self.base().try_get_node_as::<Path3D>(&get_text_mob_path(i));

                // get speed
                let follow_speed = randf_range(0.1, 0.34);

                // * debug info
                godot_print!("{}: {:?}, {:?}", get_text_mob_name(i), follow_path, path);

                // init mob
                mob.bind_mut().initialize(follow_path, path, follow_speed);
            }
        }
    }

    fn process(&mut self, _delta: f64) {
        self.base()
            .get_node_as::<Marker3D>("CameraPivot")
            .set_position(self.base().get_node_as::<Player>("Player").get_position());
    }
}

#[godot_api]
impl Level2 {
    fn on_player_hit(&mut self) {
        self.base().get_node_as::<Player>("Player").bind_mut().alive();
    }

    fn on_mob_squashed(&mut self) {
        // get array of mobs with group 'ArenaMobs'
        let mobs = self
            .base()
            .get_tree()
            .unwrap()
            .get_nodes_in_group("ArenaMobs");

        // count of dead mobs
        let mut mob_dead_count: usize = 0;

        // get count of dead mobs 
        for mob in mobs.iter_shared() {
            let mob = mob.cast::<Mob>();

            if mob.bind().is_die {
                mob_dead_count += 1;
            }
        }

        // if all mobs die, then open the door
        if mob_dead_count == mobs.len() {
            // get door
            let door = self.base().get_tree().unwrap().get_nodes_in_group("door").at(0);
            let mut door = door.cast::<Node3D>();

            // update door position
            let position = door.get_position() + Vector3::new(0.0, 7.0, 0.0);
            door.set_position(
                position
            );
        }
    }
}
