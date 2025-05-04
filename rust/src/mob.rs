//! This file contain the Mob class for Godot.
//! Mob is a enemy for player and mob can kill the player.

use std::f64::consts::PI;
use godot::{
    classes::{
        AnimationPlayer, AudioStreamPlayer, CharacterBody3D, 
        CollisionShape3D, GpuParticles3D, ICharacterBody3D, 
        VisibleOnScreenNotifier3D
    }, 
    global::{randf_range, randi_range}, 
    prelude::*
};

/// This class is a enemy for player.
#[derive(GodotClass)]
#[class(init, base = CharacterBody3D)]
pub struct Mob {
    #[init(val = 1.0)] /// improvement for player
    pub slowdown: f64,

    #[init(val = 10)] /// minimum speed of the mob in m/s
    #[export] pub min_speed: i64,

    #[init(val = 18)] /// maximum speed of the mob in m/s
    #[export] pub max_speed: i64,

    #[init(val = 0.84)] /// minimum scale of the mob
    #[export] pub min_scale: f64,

    #[init(val = 1.09)] /// maximum scale of the mob
    #[export] pub max_scale: f64,
    
    base: Base<CharacterBody3D>
} 

#[godot_api] impl ICharacterBody3D for Mob {
    fn ready(&mut self) {
        // connect method for screen_exited signal
        self.base().get_node_as
            ::<VisibleOnScreenNotifier3D>("VisibleNotifier")
            .signals().screen_exited()
            .connect_obj(
                self, 
                Self::on_visible_on_screen_notifier_3d_screen_exited
            );
        
        // connect method for DeadSound.finished signal
        self.base().get_node_as::<GpuParticles3D>("DeadEffect")
            .signals().finished()
            .connect_obj(self, Self::on_dead_effect_finished);
    }

    fn physics_process(&mut self, _delta: f64) {
        self.base_mut().move_and_slide();
    }
}

#[godot_api] impl Mob {
    #[signal] pub fn squashed();

    /// This function will be called from Main scene and need for init mob.
    pub fn initialize(&mut self, start_pos: Vector3, player_pos: Vector3) {
        self.base_mut()
            .look_at_from_position(start_pos, player_pos);

        // set scale of the mob
        let scale_factor = randf_range(
            self.min_scale, self.max_scale
        ) as f32;

        self.base_mut().set_scale(Vector3::new(
            scale_factor, scale_factor, scale_factor
        ));

        // rotate this mob randomly within range of -45 and +45 degrees,
        // so that it doesn't move directly towards the player
        self.base_mut()
            .rotate_y(randf_range(-PI / 4.0, PI / 4.0) as f32);
        
        // we calculate a random speed (in integer)
        let random_speed = randi_range(
            self.min_speed, self.max_speed
        ) as f32 / self.slowdown as f32;

        // get velocity
        #[allow(unused_assignments)]
        let mut velocity = self.base().get_velocity();

        // we calculate a forward velocity that represents the speed
        velocity = Vector3::FORWARD * random_speed;

        // we calculate a forward velocity vector based on the mob's
        // Y rotation in order to move in the direction the mob is looking
        velocity = velocity.rotated(
            Vector3::UP, self.base().get_rotation().y
        );

        // set new velocity
        self.base_mut().set_velocity(velocity);

        // set animation speed scale
        let mut animation = self.base()
            .get_node_as::<AnimationPlayer>("AnimationPlayer");
        animation.set_speed_scale(random_speed / self.min_speed as f32);
    }

    /// Kill the mob.
    pub fn squash(&mut self) {
        // start dead effect
        self.base().get_node_as::<GpuParticles3D>("DeadEffect")
            .set_emitting(true);

        // disable collision shape
        self.base().get_node_as::<CollisionShape3D>("CollisionShape3D")
            .set_disabled(true);

        // start dead sound
        self.base().get_node_as::<AudioStreamPlayer>("DeadSound").play();

        // hide pivot
        self.base().get_node_as::<Node3D>("Pivot").hide();

        // set velocity to zero
        self.base_mut().set_velocity(Vector3::ZERO);

        // emit signal
        self.signals().squashed().emit();
    }

    /// Delete the mob if he leave from screen.
    fn on_visible_on_screen_notifier_3d_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }

    /// Delete the mob if dead effect finalize.
    fn on_dead_effect_finished(&mut self) {
        self.base_mut().queue_free();
    }
}
