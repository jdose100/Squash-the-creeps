use godot::{classes::{Path3D, PathFollow3D}, global::randf_range, prelude::*};

use crate::mob::Mob;
use super::{BaseLevel, get_text_mob_name, get_text_mob_follow_path, get_text_mob_path};

#[derive(GodotClass)]
#[class(init, base = Node)]
struct Level2 { 
    #[init(val = 0)] /// How much mobs on level.
    #[export] all_mobs_on_level: i64,

    #[hint(base)] /// Base class for godot functions.
    real_base: Base<Node>,

    #[init(val = BaseLevel::new_alloc())] /// Real base class for logic.
    base: Gd<BaseLevel>
}

#[godot_api] impl INode for Level2 {
    fn ready(&mut self) {
        // init logic from base
        self.base.bind_mut().use_child_mob_init_logic = false;
        self.base.bind_mut().ready();

        // init all mobs
        for i in 0..self.all_mobs_on_level {
                // get mob from scene
                let mob = self.base().try_get_node_as::<Mob>(&get_text_mob_name(i)); 

                // check mob is exits or not
                if let Some(mut mob) = mob {
                    // if mob exits then connect signal
                    mob.signals().squashed().connect_obj(self, Self::on_mob_squashed);

                    //* The following code is needed to be able to move along a given path!

                    // get follow path
                    let follow_path = self.base().try_get_node_as::<PathFollow3D>(
                        &get_text_mob_follow_path(i)
                    );

                    // get path
                    let path = self.base().try_get_node_as::<Path3D>(
                        &get_text_mob_path(i)
                    );

                    // get speed
                    let follow_speed = randf_range(0.1, 0.34);

                    // init mob
                    mob.bind_mut().initialize(follow_path, path, follow_speed);
                }
            }
    }
}

#[godot_api] impl Level2 {
    fn on_mob_squashed(&mut self) {
        let mobs = self.base().get_tree().unwrap().get_nodes_in_group("ArenaMobs");

        for mob in mobs.iter_shared() {
            let mob = mob.cast::<Mob>();
        }
    }
}
