//! This file contain the Player class and logic for Godot.
//! Player is a entity with control by gamer. It is needed so that
//! the gamer can connect with the gaming world.

use godot::{
    classes::{
        AnimationPlayer, Area3D, AudioStreamPlayer, CharacterBody3D, GpuParticles3D,
        ICharacterBody3D, Input,
    },
    obj::WithBaseField,
    prelude::*,
};
use std::f32::consts::PI;

/// Player class store a logic for control player and other.
#[derive(GodotClass)]
#[class(base = CharacterBody3D)]
pub struct Player {
    /// Velocity for player movement.
    target_velocity: Vector3,

    /// Shield around player, if he active player can't die.
    pub shield_active: bool,

    /// Indicates whether the player is dead or not.
    is_die: bool,

    /// Gravity for player.
    #[export]
    pub fall_acceleration: f64,

    /// Vertical impulse applied to the character upon bouncing a mob, in m/s.
    #[export]
    pub bounce_impulse: f64,

    /// Vertical impulse applied to the character upon jumping, in m/s.
    #[export]
    pub jump_impulse: f64,

    /// How fast the player moves, in m/s.
    #[export]
    speed: f64,

    /// Position for spawn if player alive.
    #[export]
    spawn_coords: Vector3,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            target_velocity: Vector3::ZERO,
            shield_active: false,
            is_die: true,
            fall_acceleration: 75.0,
            bounce_impulse: 16.0,
            jump_impulse: 20.0,
            speed: 14.0,
            spawn_coords: Vector3::ZERO,
            base,
        }
    }

    fn ready(&mut self) {
        // hide Pivot
        self.base().get_node_as::<Node3D>("Pivot").hide();

        // connect 'body_entered' signal
        self.base()
            .get_node_as::<Area3D>("MobDetector")
            .signals()
            .body_entered()
            .connect_obj(self, Self::on_mob_detector_body_entered);
    }

    fn physics_process(&mut self, delta: f64) {
        // if player is die, then exit
        if self.is_die {
            return;
        }

        // get input singleton
        let input = Input::singleton();

        // variable to store the input direction
        let mut direction = Vector3::ZERO;

        // we check for each move input and update the direction accordingly
        if input.is_action_pressed("move_right") {
            direction.x += 1.0;
        }
        if input.is_action_pressed("move_left") {
            direction.x -= 1.0;
        }
        if input.is_action_pressed("move_back") {
            direction.z += 1.0;
        }
        if input.is_action_pressed("move_forward") {
            direction.z -= 1.0;
        }

        // update look and normalize direction if need
        // and set animation speed
        if direction != Vector3::ZERO {
            // normalize direction
            direction = direction.normalized();

            // set look at for Pivot
            let point = self.base().get_position() + direction;
            self.base().get_node_as::<Node3D>("Pivot").look_at(point);

            // set animation speed
            self.base()
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .set_speed_scale(3.0);
        } else {
            // set animation speed
            self.base()
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .set_speed_scale(1.0);
        }

        // update ground velocity
        self.target_velocity.x = direction.x * self.speed as f32;
        self.target_velocity.z = direction.z * self.speed as f32;

        // update vertical velocity
        if !self.base().is_on_floor() {
            self.target_velocity.y =
                self.target_velocity.y - (self.fall_acceleration * delta) as f32;
        }

        // jumping
        if self.base().is_on_floor() && input.is_action_pressed("jump") {
            self.target_velocity.y = self.jump_impulse as f32;
        }

        // iterate through all collisions that occurred this frame
        for index in 0..self.base().get_slide_collision_count() {
            // we get one of the collisions with the player
            let collision = 
                self.base_mut().get_slide_collision(index).unwrap();

            // if there are duplicate with a mob in a single frame
            // the mob will be deleted after the first collision, and
            // a second call to get_collider will return null, leading
            // to a null pointer when calling
            // collision.get_collider().is_in_group("mob")
            // this block of code prevents processing duplicate collisions
            if let None = collision.get_collider() {
                continue;
            }

            // if the collider is with a mob
            if collision
                .get_collider()
                .unwrap()
                .cast::<Node3D>()
                .is_in_group("mob")
            {
                let collider = collision.get_collider().unwrap();
                let mut mob = collider.cast::<crate::mob::Mob>();

                // we check that we are hitting it from above
                if Vector3::UP.dot(collision.get_normal()) > 0.1 {
                    // if so, we squash it and bounce
                    self.target_velocity.y = self.bounce_impulse as f32;
                    mob.bind_mut().squash();

                    // prevent further duplicate calls
                    break;
                }
            }
        }

        // moving
        let target_velocity = self.target_velocity;
        self.base_mut().set_velocity(target_velocity);
        self.base_mut().move_and_slide();

        // get rotation
        let velocity = self.base().get_velocity();
        let mut rotation = self.base().get_node_as::<Node3D>("Pivot").get_rotation();
        rotation.x = PI / 6.0 * velocity.y / self.jump_impulse as f32;

        // rotate
        self.base_mut()
            .get_node_as::<Node3D>("Pivot")
            .set_rotation(rotation);

        // ! NEXT CODE ONLY FOR DEVELOP!
        // if player.y position < -10, then set player position to spawn_coords
        if self.base().get_position().y < -10.0 {
            self.kill();
        }
    }
}

#[godot_api]
impl Player {
    /// Signal emit if player hit.
    #[signal]
    pub fn hit();

    /// Makes the player alive.
    #[func]
    pub fn alive(&mut self) {
        // alive player
        self.is_die = false;

        // show Pivot
        self.base().get_node_as::<Node3D>("Pivot").show();

        // set position to spawn coordinates
        let spawn_coords = self.spawn_coords;
        self.base_mut().set_position(spawn_coords);
    }

    /// Kill the player (from signal).
    fn on_mob_detector_body_entered(&mut self, _body: Gd<Node3D>) {
        self.kill();
    }

    /// Kill the player.
    fn kill(&mut self) {
        if !self.is_die && !self.shield_active && false {
            self.is_die = true;

            // hide Pivot
            self.base().get_node_as::<Node3D>("Pivot").hide();

            // play DeadSound
            self.base()
                .get_node_as::<AudioStreamPlayer>("DeathSound")
                .play();

            // emit DeadEffect
            self.base()
                .get_node_as::<GpuParticles3D>("DeathEffect")
                .set_emitting(true);

            // emit signal
            self.signals().hit().emit();
        }
    }
}
