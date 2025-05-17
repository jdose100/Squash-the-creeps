//! Данный модуль содержит логику уровней и все необходимые данные, которые касаются уровней.
//! BaseLevel - это класс, который реализует минимум функций для работы уровня,
//! является полностью самостоятельным. Подмодули же содержат доп. логику, а 
//! BaseLevel в этом случае выступает в качестве родителя.


use crate::{config::CONFIG, mob::Mob, player::Player, ui::{pause_menu::PauseMenu, settings::SettingsHUD}};
use end_of_level::HowPlayerExitedFromLevel;
use godot::{
    classes::{Area3D, Button, CollisionShape3D, ColorRect, InputEvent, Marker3D, Path3D, PathFollow3D}, 
    global::randf_range, 
    obj::{bounds::DeclUser, BaseRef, Bounds, WithBaseField}, 
    prelude::*
};

pub(crate) mod end_of_level;
mod level2;

/// Класс, реализующий минимальную логику
/// для самостоятельности уровня.
#[derive(GodotClass)]
#[class(base = Node)]
struct BaseLevel {
    /// Как много мобов на уровне.
    #[export]
    all_mobs_on_level: i64,

    /// Сколько мобов было убито на уровне.
    squashed_mobs: i64,

    /// Все мобы на уровне.
    mobs: Vec<Gd<Mob>>,

    base: Base<Node>,
}

#[godot_api]
impl INode for BaseLevel {
    fn init(base: Base<Node>) -> Self {
        Self {
            all_mobs_on_level: 0,
            squashed_mobs: 0,
            mobs: Vec::new(),
            base,
        }
    }

    fn ready(&mut self) {
        // Инициализируем всех мобов.
        self.mobs =
            Self::default_mobs_init(
                &self.base(),
                self.all_mobs_on_level,
                |this, mob| {
                    mob
                        .signals()
                        .squashed()
                        .connect_obj(this, Self::on_mob_squashed);
                }
            );

        // Инициализируем уровень.
        Self::init_node(&self.base(), self.mobs.clone(), self.base().get_scene_file_path(), 1);

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

    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("pause") {
            Self::pause(self.base().clone().cast::<Self>(), &mut self.mobs);
        }
    }
}

impl BaseLevel {
    /// Настраивает уровень с параметрами BaseLevel, 
    /// необходим для удаления шаблонного кода в дочерних классах. 
    /// 
    /// Mobs - массив мобов нужный для работы паузы.
    /// 
    /// FileName - строка в виде пути к сцене, нужна для работы кнопки `Restart`.
    /// 
    /// Пример использования:
    /// ```rust
    /// BaseLevel::init_node(&self.base(), self.mobs.clone());
    /// ```
    pub fn init_node<T>(
        node: &BaseRef<T>, 
        mut mobs: Vec<Gd<Mob>>,
        path: GString,
        num_of_level: u64,
    ) 
    where
        T: GodotClass + INode + Bounds<Declarer = DeclUser> + Inherits<Node> + WithBaseField
    {
        if let Some(mut player) = node.try_get_node_as::<Player>("Player") {
            player.bind_mut().alive();
            
            node.get_node_as::<CollisionShape3D>(
                "Objects/Exit/CharacterBody3D/CollisionShape3D"
            ).set_disabled(true);
            node.get_node_as::<Node3D>("Objects/Exit").hide();

            // Настройка выхода из уровня.
            node
                .get_node_as::<Area3D>("Player/ExitDetector")
                .signals()
                .body_entered()
                .connect_obj(
                    &mut node.to_godot().cast::<T>(), 
                    move |this: &mut T, _body: Gd<Node3D>| {
                        end_of_level::EXIT_FROM_LEVEL_DATA
                            .write().unwrap().num_of_level_exit = num_of_level;

                        end_of_level::EXIT_FROM_LEVEL_DATA
                            .write().unwrap().exited = HowPlayerExitedFromLevel::EndOfLevel;

                        this.base().get_tree().unwrap().change_scene_to_file("res://scenes/main.tscn")
                });
        }

        if let Some(mut pause_menu) = node.try_get_node_as::<PauseMenu>("PauseMenu") {
            // Сразу переводим меню на уже установленный язык.
            pause_menu.bind_mut().translate(CONFIG.lock().unwrap().get_language());

            // * Настройка кнопки продолжить.
            let continue_func = move |this: &mut T| { 
                this.base().get_node_as::<Player>("Player").set_physics_process(true);
                this.base().get_node_as::<ColorRect>("PauseMenu").hide();

                for mob in &mut mobs {
                    mob.set_physics_process(true);
                }
            };

            pause_menu
                .get_node_as::<Button>("ContinueButton")
                .signals()
                .pressed()
                .connect_obj(&mut node.to_godot().cast::<T>(), continue_func);

            // * Настройка кнопки рестарт.
            let restart_func = move |this: &mut T| {
                this.base().get_tree().unwrap().change_scene_to_file(&path);
            };

            pause_menu
                .get_node_as::<Button>("RestartButton")
                .signals()
                .pressed()
                .connect_obj(&mut node.to_godot().cast::<T>(), restart_func);

            // * Настройка кнопки выхода.
            let exit_func = |this: &mut T| {
                // Указываем что вышли из уровня через меню.
                end_of_level::EXIT_FROM_LEVEL_DATA.write().unwrap()
                    .exited = end_of_level::HowPlayerExitedFromLevel::ExitFromMenu;

                this.base().get_tree().unwrap().change_scene_to_file("res://scenes/main.tscn");
            };

            pause_menu
                .get_node_as::<Button>("ExitButton")
                .signals()
                .pressed()
                .connect_obj(&mut node.to_godot().cast::<T>(), exit_func);

            // * Настройка кнопки настройки.
            // Закрытие настроек.
            pause_menu
                .get_node_as::<Button>("SettingsHUD/ExitButton")
                .signals()
                .pressed()
                .connect_obj(&pause_menu, |this: &mut PauseMenu| {
                    this.base().get_node_as::<SettingsHUD>("SettingsHUD").hide();
                    this.show();
                });
            
            // Открытие настроек
            pause_menu
                .get_node_as::<Button>("SettingsButton")
                .signals()
                .pressed()
                .connect_obj(&pause_menu, |this: &mut PauseMenu| {
                    this.base().get_node_as::<SettingsHUD>("SettingsHUD").show();
                    this.hide();
                });
            
            // Изменение настроек.
            pause_menu
                .get_node_as::<SettingsHUD>("SettingsHUD")
                .signals()
                .language_changed()
                .connect_obj(&pause_menu, |this: &mut PauseMenu| {
                    this.translate(CONFIG.lock().unwrap().get_language());
                });
        }
    }

    /// Инициализировать и вызвать «метод» для всех мобов в сцене, указанных счетчиком. 
    pub fn default_mobs_init<T, F>(node: &BaseRef<T>, mob_count: i64, method: F) -> Vec<Gd<Mob>>
    where
        T: GodotClass + INode + Inherits<Node>,
        F: Fn(&Gd<T>, &mut Gd<Mob>),
    {
        let mut mobs: Vec<Gd<Mob>> = Vec::new();

        for i in 0..mob_count {
            let mob = node.try_get_node_as::<Mob>(&get_text_mob_name(i));

            if let Some(mut mob) = mob {
                method(&node.to_godot().cast(), &mut mob);

                let follow_path =
                    node.try_get_node_as::<PathFollow3D>(&get_text_mob_follow_path(i));

                let path = node.try_get_node_as::<Path3D>(&get_text_mob_path(i));
                let follow_speed = randf_range(0.1, 0.34);

                mob.bind_mut().initialize(follow_path, path, follow_speed);
                
                mobs.push(mob);
            }
        }

        mobs
    }

    /// Ставит уровень на паузу.
    pub fn pause<T>(node: Gd<T>, mobs: &mut Vec<Gd<Mob>>)
    where
        T: GodotClass + INode + Inherits<Node> + WithBaseField
    {
        node.get_node_as::<Player>("Player").set_physics_process(false);
        node.get_node_as::<ColorRect>("PauseMenu").show();

        for mob in mobs {
            mob.set_physics_process(false);
        }
    }

    /// Обновляет «squashed_mobs» для раздавленных мобов, если все мобы на уровне раздавлены. 
    fn on_mob_squashed(&mut self) {
        self.squashed_mobs += 1;

        if self.squashed_mobs == self.all_mobs_on_level {
            self.base().get_node_as::<Node3D>("Objects/Exit").show();
        
            self
                .base()
                .get_node_as::<CollisionShape3D>("Objects/Exit/CharacterBody3D/CollisionShape3D")
                .set_disabled(false);
        }
    }

    /// Авто оживление игрока.
    // ! ТОЛЬКО ДЛЯ РАЗРАБОТКИ!
    fn on_player_hit(&mut self) {
        self.base().get_node_as::<Node3D>("Objects/Exit").hide();
        
        self
            .base()
            .get_node_as::<CollisionShape3D>("Objects/Exit/CharacterBody3D/CollisionShape3D")
            .set_disabled(false);

        self.base()
            .get_node_as::<Player>("Player")
            .call_deferred("alive", &[]);

        // Оживляет всех мобов.
        for i in 0..self.all_mobs_on_level {
            let mob = self.base().try_get_node_as::<Mob>(&format!("Mobs/Mob{i}"));

            if let Some(mut mob) = mob {
                mob.bind_mut().alive();
            }
        }

        self.squashed_mobs = 0;
    }
}

// * Функции, предоставляющие строковый API для загрузки данных из сцены.

/// Получает путь моба в сцене с учетом иерархии имен. 
/// Возвращает путь к классу 'Mob' в сцене. 
#[inline(always)]
fn get_text_mob_name(mob_num: i64) -> String {
    format!("Mobs/Mob{mob_num}")
}

/// Получает путь моба в сцене с учетом иерархии имен. 
/// Возвращает путь к классу 'PathFollow3D' в сцене.
#[inline(always)]
fn get_text_mob_follow_path(mob_num: i64) -> String {
    format!("Mobs/Mob{mob_num}Path/PathFollow3D")
}
/// Получает путь моба в сцене с учетом иерархии имен. 
/// Возвращает путь к классу 'Path3D' в сцене.
#[inline(always)]
fn get_text_mob_path(mob_num: i64) -> String {
    format!("Mobs/Mob{mob_num}Path")
}
