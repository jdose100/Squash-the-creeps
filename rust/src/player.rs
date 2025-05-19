//! В данном модуле содержится реализация поведения игрока.
//! Игрок позволяет пользователю контактировать с игровым миров.

use godot::{
    classes::{
        AnimationPlayer, Area3D, AudioStreamPlayer, CharacterBody3D, GpuParticles3D,
        ICharacterBody3D, Input,
    },
    obj::WithBaseField,
    prelude::*,
};
use std::f32::consts::PI;

/// Данный класс содержит логику контроля и поведения игрока.
#[derive(GodotClass)]
#[class(base = CharacterBody3D)]
pub struct Player {
    /// Скорость движения игрока.
    target_velocity: Vector3,

    /// Показывает умер ли игрок, или нет.
    is_die: bool,

    /// Гравитация игрока.
    #[export]
    pub fall_acceleration: f64,

    /// Вертикальный импульс, применяемый к персонажу при отскоке от моба.
    /// Измеряется в метрах в секунду.
    #[export]
    pub bounce_impulse: f64,

    /// Вертикальный импульс, применяемый к персонажу при прыжке. 
    /// Измеряется в метрах в секунду.
    #[export]
    pub jump_impulse: f64,

    /// Скорость игрока, в метрах в секунду.
    #[export]
    speed: f64,

    /// Позиция для появления игрока при оживлении.
    #[export]
    spawn_coords: Vector3,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            target_velocity: Vector3::ZERO,
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
        // Скрываем меш.
        self.base().get_node_as::<Node3D>("Pivot").hide();

        // Подключаем сигнал 'body_entered'.
        self
            .base()
            .get_node_as::<Area3D>("MobDetector")
            .signals()
            .body_entered()
            .connect_obj(self, Self::on_mob_detector_body_entered);
        
        self
            .base()
            .get_node_as::<Area3D>("ExitDetector")
            .signals()
            .body_entered()
            .connect_obj(self, |_this: &mut Self, _body| {
                godot_print!("exit")
            });
    }

    fn physics_process(&mut self, delta: f64) {
        let input = Input::singleton();

        // Переменная для получения направления.
        let mut direction = Vector3::ZERO;

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

        // Обновляем внешний вид и нормализуем направление, если необходимо, 
        // а также задаем скорость анимации.
        if direction != Vector3::ZERO {
            direction = direction.normalized();

            // Устанавливаем взгляд на опорную точку.
            let point = self.base().get_position() + direction;
            self.base().get_node_as::<Node3D>("Pivot").look_at(point);

            // Устанавливаем скорость анимации.
            self.base()
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .set_speed_scale(3.0);
        } else {
            // Устанавливаем скорость анимации.
            self.base()
                .get_node_as::<AnimationPlayer>("AnimationPlayer")
                .set_speed_scale(1.0);
        }

        // Обновляем скорость относительно земли.
        self.target_velocity.x = direction.x * self.speed as f32;
        self.target_velocity.z = direction.z * self.speed as f32;

        // Обновляем вертикальную скорость.
        if !self.base().is_on_floor() {
            self.target_velocity.y =
                self.target_velocity.y - (self.fall_acceleration * delta) as f32;
        }

        // Прыгаем, если надо.
        if self.base().is_on_floor() && input.is_action_pressed("jump") {
            self.target_velocity.y = self.jump_impulse as f32;
        }

        // Перебираем все столкновения, произошедшие в этом кадре.
        for index in 0..self.base().get_slide_collision_count() {
            let collision = self.base_mut().get_slide_collision(index).unwrap();

            // Если в одном кадре есть дубликаты с мобом, то моб будет удален после первого столкновения, 
            // а второй вызов `get_collider` вернет null, что приведет к нулевому указателю при вызове 
            // `collision.get_collider().is_in_group("mob")`.
            // Этот блок кода предотвращает обработку дубликатов столкновений.
            if let None = collision.get_collider() {
                continue;
            }

            // Если коллайдер это моб.
            if collision
                .get_collider()
                .unwrap()
                .cast::<Node3D>()
                .is_in_group("mob")
            {
                let collider = collision.get_collider().unwrap();
                let mut mob = collider.cast::<crate::mob::Mob>();

                // Мы проверяем, что мы ударяем сверху.
                if Vector3::UP.dot(collision.get_normal()) > 0.1 {
                    self.target_velocity.y = self.bounce_impulse as f32;
                    mob.bind_mut().squash();
                    break;
                }
            }
        }

        // Двигаем персонажа.
        let target_velocity = self.target_velocity;
        self.base_mut().set_velocity(target_velocity);
        self.base_mut().move_and_slide();

        // Вращаем персонажа.
        let velocity = self.base().get_velocity();
        let mut rotation = self.base().get_node_as::<Node3D>("Pivot").get_rotation();
        rotation.x = PI / 6.0 * velocity.y / self.jump_impulse as f32;

        self
            .base()
            .get_node_as::<Node3D>("Pivot")
            .set_rotation(rotation);

        // ! Данный код нужен только для разработки.
        if self.base().get_position().y < -10.0 {
            self.kill();
        }
    }
}

#[godot_api]
impl Player {
    /// Данный сигнал испускается, если игрок умер.
    #[signal]
    pub fn hit();

    /// Оживляет игрока.
    #[func]
    pub fn alive(&mut self) {
        self.base().get_node_as::<Node3D>("Pivot").show();
        self.is_die = false;

        // Меняем координаты.
        let spawn_coords = self.spawn_coords;
        self.base_mut().set_position(spawn_coords);

        // Разрешаем physic_process.
        self.base_mut().set_physics_process(true);
    }

    /// Убивает игрока (после вызова сигнала).
    fn on_mob_detector_body_entered(&mut self, _body: Gd<Node3D>) {
        self.kill();
    }

    /// Убивает игрока.
    fn kill(&mut self) {
        if !self.is_die {
            self.base().get_node_as::<Node3D>("Pivot").hide();
            self.is_die = true;

            self.base()
                .get_node_as::<AudioStreamPlayer>("DeathSound")
                .play();

            self.base()
                .get_node_as::<GpuParticles3D>("DeathEffect")
                .set_emitting(true);

            self.signals().hit().emit();
        }
    }
}

