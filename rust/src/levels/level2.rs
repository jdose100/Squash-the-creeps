use godot::{
    classes::Marker3D,
    prelude::*,
};

use super::BaseLevel;
use crate::{mob::Mob, player::Player};

/// Store a logic for level number 2. This class
/// open the door if all 'ArenaMobs' is die.
#[derive(GodotClass)]
#[class(base = Node)]
struct Level2 {
    /// How much mobs on level.
    #[export]
    all_mobs_on_level: i64,

    /// Base class for godot functions.
    base: Base<Node>,
}

#[godot_api]
impl INode for Level2 {
    fn init(base: Base<Node>) -> Self {
        Self {
            all_mobs_on_level: 0,
            base,
        }
    }

    fn ready(&mut self) {
        // init logic from base
        BaseLevel::init_node(&self.base());

        // init all mobs with base
        BaseLevel::default_mobs_init(
            &self.base(), 
            self.all_mobs_on_level, 
            |this, mob| {
                mob.signals()
                    .squashed()
                    .connect_obj(this, Self::on_mob_squashed);
            }
        );

        // connect method for 'hit' signal from player
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
impl Level2 {
    fn on_player_hit(&mut self) {
        self.base()
            .get_node_as::<Player>("Player")
            .bind_mut()
            .alive();
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
            let mut door = self
                .base()
                .get_tree()
                .unwrap()
                .get_nodes_in_group("door")
                .at(0)
                .cast::<Node3D>();

            // update door position
            let position = door.get_position() + Vector3::new(0.0, 7.0, 0.0);
            door.set_position(position);
        }
    }
}
