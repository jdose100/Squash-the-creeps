//! Данный файл содержит реализацию моба.
//! Моб - враг игрока, его задача стоит в том, что бы
//! охранять территорию и убивать игрока.

use godot::{
    classes::{
        AnimationPlayer, AudioStreamPlayer, CharacterBody3D, CollisionShape3D, GpuParticles3D,
        ICharacterBody3D, Path3D, PathFollow3D, VisibleOnScreenNotifier3D,
    },
    global::{randf_range, randi_range},
    obj::WithBaseField,
    prelude::*,
};

/// Класс с реализацией Моба.
#[derive(GodotClass)]
#[class(base = CharacterBody3D)]
pub struct Mob {
    /// Минимальная скорость в метрах в секунду.
    #[export]
    pub min_speed: i64,

    /// Максимальная скорость в метрах в секунду.
    #[export]
    pub max_speed: i64,

    /// Минимальный коэффициент размера.
    #[export]
    pub min_scale: f64,

    /// Максимальный коэффициент размера.
    #[export]
    pub max_scale: f64,

    /// Устанавливаем данную позицию при возрождении.
    pub spawn_coords: Vector3,

    /// Устанавливает данную скорость при возрождении.
    pub spawn_velocity: Vector3,

    /// Показывает умер ли моб, или нет. Нужно для логики уровней.
    pub is_die: bool,

    /// Нужна для траектории движения моба, зависит от `path`!
    follow_path: Option<Gd<PathFollow3D>>,

    /// Скорость для 'follow_path' и 'path'.
    follow_speed: f64,
    
    /// Нужна для траектории движения моба, зависит от `follow_path`!
    path: Option<Gd<Path3D>>,

    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Mob {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            spawn_coords: Vector3::ZERO,
            spawn_velocity: Vector3::ONE,
            min_speed: 10,
            max_speed: 18,
            min_scale: 0.84,
            max_scale: 1.09,
            is_die: false,
            follow_path: None,
            follow_speed: 0.1,
            path: None,
            base,
        }
    }

    fn ready(&mut self) {
        self
            .base()
            .get_node_as::<VisibleOnScreenNotifier3D>("VisibleNotifier")
            .signals()
            .screen_exited()
            .connect_obj(self, Self::on_visible_on_screen_notifier_3d_screen_exited);
    }

    fn physics_process(&mut self, _delta: f64) {
        // Двигает моба по траектории, если это возможно.
        if let Some(mut follow_path) = self.follow_path.clone() {
            let path = self.path.clone().unwrap();

            // Получаем старую позицию.
            let old_position = self.base().get_position();

            // Считаем новую позицию.
            let new_position = follow_path.get_position() + path.get_position();

            // Считаем вращение.
            self.base_mut().look_at_from_position(old_position, new_position);

            // Устанавливаем позицию.
            self.base_mut().set_position(new_position);

            // Обновляем коэффициент.
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

    /// Инициализирует моба дополнительными данными, неизвестными при вызове `init`.
    pub fn initialize(
        &mut self,
        follow_path: Option<Gd<PathFollow3D>>,
        path: Option<Gd<Path3D>>,
        follow_speed: f64,
    ) {
        self.spawn_coords = self.base().get_position();
        self.follow_speed = follow_speed;
        self.follow_path = follow_path;
        self.path = path;

        let scale_factor = randf_range(self.min_scale, self.max_scale) as f32;
        self.base_mut().set_scale(Vector3::new(scale_factor, scale_factor, scale_factor));

        let random_speed = randi_range(self.min_speed, self.max_speed) as f32;
        let mut animation = self.base().get_node_as::<AnimationPlayer>("AnimationPlayer");
        animation.set_speed_scale(random_speed / self.min_speed as f32);
    }

    /// Убивает моба.
    pub fn squash(&mut self) {
        self.is_die = true;
 
        self.base().get_node_as::<CollisionShape3D>("CollisionShape3D").set_disabled(true);
        self.base().get_node_as::<GpuParticles3D>("DeadEffect").set_emitting(true);
        self.base().get_node_as::<AudioStreamPlayer>("DeadSound").play();
        self.base().get_node_as::<Node3D>("Pivot").hide();
        self.base_mut().set_velocity(Vector3::ZERO);
        self.signals().squashed().emit();
    }

    fn on_visible_on_screen_notifier_3d_screen_exited(&mut self) {
        // ! self.base_mut().queue_free();
    }

    /// Оживляет моба.
    pub fn alive(&mut self) {
        self.is_die = false;

        let spawn_velocity = self.spawn_velocity;
        self.base_mut().set_velocity(spawn_velocity);

        let spawn_coords = self.spawn_coords;
        self.base_mut().set_position(spawn_coords);

        self.base().get_node_as::<Node3D>("Pivot").show();

        self
            .base()
            .get_node_as::<CollisionShape3D>("CollisionShape3D")
            .set_disabled(false);
    }
}
