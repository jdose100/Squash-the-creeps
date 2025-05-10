//! This file contain the Mob class for Godot.
//! Mob is a enemy for player and mob can kill the player.

use godot::{
    classes::{
        AnimationPlayer, AudioStreamPlayer, CharacterBody3D, CollisionShape3D, GpuParticles3D,
        ICharacterBody3D, Path3D, PathFollow3D, VisibleOnScreenNotifier3D,
    },
    global::{randf_range, randi_range},
    obj::WithBaseField,
    prelude::*,
};

/// This class is a enemy for player.
#[derive(GodotClass)]
#[class(base = CharacterBody3D)]
pub struct Mob {
    /// Improvement for player.
    pub slowdown: f64,

    /// Set 'self.position' to spawn_coords if mob alive.
    pub spawn_coords: Vector3,

    /// Set 'self.velocity' to spawn_velocity if mob alive.
    pub spawn_velocity: Vector3,

    /// Minimum speed of the mob, in m/s.
    #[export]
    pub min_speed: i64,

    /// Maximum speed of the mob, in m/s.
    #[export]
    pub max_speed: i64,

    /// Minimum scale of the mob.
    #[export]
    pub min_scale: f64,

    /// Maximum scale of the mob.
    #[export]
    pub max_scale: f64,

    /// needs for move mob along the trajectory, depends on path!
    follow_path: Option<Gd<PathFollow3D>>,

    /// Speed for 'follow_path' and 'path'.
    follow_speed: f64,

    /// Needs for move mob along the trajectory, depends on 'follow_path'!
    path: Option<Gd<Path3D>>,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Mob {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            slowdown: 1.0,
            spawn_coords: Vector3::ZERO,
            spawn_velocity: Vector3::ONE,
            min_speed: 10,
            max_speed: 18,
            min_scale: 0.84,
            max_scale: 1.09,
            follow_path: None,
            follow_speed: 0.1,
            path: None,
            base
        }
    }

    fn ready(&mut self) {
        // connect method for screen_exited signal
        self.base()
            .get_node_as::<VisibleOnScreenNotifier3D>("VisibleNotifier")
            .signals()
            .screen_exited()
            .connect_obj(self, Self::on_visible_on_screen_notifier_3d_screen_exited);

        // connect method for DeadSound.finished signal
        self.base()
            .get_node_as::<GpuParticles3D>("DeadEffect")
            .signals()
            .finished()
            .connect_obj(self, Self::on_dead_effect_finished);
    }

    fn physics_process(&mut self, _delta: f64) {
        // moves the mob along the trajectory if possible
        if let Some(mut follow_path) = self.follow_path.clone() {
            // get path
            let path = self.path.clone().unwrap();

            // get oldest position
            let old_position = self.base().get_position();

            // calculate new position
            let new_position = follow_path.get_position() + path.get_position();

            // calculate rotation
            self.base_mut()
                .look_at_from_position(old_position, new_position);

            // set new position
            // self.base_mut().set_position(new_position);
            self.base_mut().set_position(new_position);

            // update ratio
            let ratio = follow_path.get_progress() + self.follow_speed as f32;
            follow_path.set_progress(ratio);
        }

        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl Mob {
    #[signal]
    pub fn squashed();

    /// This function will be called from BaseLevel and need for init mob.
    pub fn initialize(
        &mut self,
        follow_path: Option<Gd<PathFollow3D>>,
        path: Option<Gd<Path3D>>,
        follow_speed: f64,
    ) {
        // set self variables
        self.follow_speed = follow_speed;
        self.follow_path = follow_path;
        self.path = path;

        // set spawn coords
        self.spawn_coords = self.base().get_position();

        // set scale of the mob
        let scale_factor = randf_range(self.min_scale, self.max_scale) as f32;

        self.base_mut()
            .set_scale(Vector3::new(scale_factor, scale_factor, scale_factor));

        // we calculate a random speed
        let random_speed =
            randi_range(self.min_speed, self.max_speed) as f32 / self.slowdown as f32;

        // set animation speed scale
        let mut animation = self
            .base()
            .get_node_as::<AnimationPlayer>("AnimationPlayer");
        animation.set_speed_scale(random_speed / self.min_speed as f32);
    }

    /// Kill the mob.
    pub fn squash(&mut self) {
        // start dead effect
        self.base()
            .get_node_as::<GpuParticles3D>("DeadEffect")
            .set_emitting(true);

        // disable collision shape
        self.base()
            .get_node_as::<CollisionShape3D>("CollisionShape3D")
            .set_disabled(true);

        // start dead sound
        self.base()
            .get_node_as::<AudioStreamPlayer>("DeadSound")
            .play();

        // hide pivot
        self.base().get_node_as::<Node3D>("Pivot").hide();

        // set velocity to zero
        self.base_mut().set_velocity(Vector3::ZERO);

        // emit signal
        self.signals().squashed().emit();
    }

    /// Delete the mob if he leave from screen.
    fn on_visible_on_screen_notifier_3d_screen_exited(&mut self) {
        // ! self.base_mut().queue_free();
    }

    /// Delete the mob if dead effect finalize.
    fn on_dead_effect_finished(&mut self) {
        // ! self.base_mut().queue_free();
    }

    /// Alive mob.
    pub fn alive(&mut self) {
        // set velocity
        let spawn_velocity = self.spawn_velocity;
        self.base_mut().set_velocity(spawn_velocity);

        // set position
        let spawn_coords = self.spawn_coords;
        self.base_mut().set_position(spawn_coords);

        // enable collision shape
        self.base()
            .get_node_as::<CollisionShape3D>("CollisionShape3D")
            .set_deferred("disabled", &Variant::from(false));

        // show pivot
        self.base().get_node_as::<Node3D>("Pivot").show();
    }
}
