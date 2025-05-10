//! This module store a logic for UI (user interface).

use crate::main_scene::Improvements;
use godot::{
    classes::{Button, ColorRect, Control, IControl, Label},
    obj::WithBaseField,
    prelude::*,
};
use std::sync::LazyLock;

/// This enum store of all type of languages.
#[derive(Default)]
enum Languages {
    #[default]
    EN,
    RU,
}

/// This struct store a text from translation.
struct LanguageText {
    language: Languages,
    score: &'static str,
    max_score: &'static str,
    name_of_game: &'static str,
    start_button: &'static str,
    language_button: &'static str,
    improvement_slow_creeps: &'static str,
    improvement_shield: &'static str,
}

/// This variable store translation to English.
static EN_LANGUAGE: LazyLock<LanguageText> = LazyLock::new(|| LanguageText {
    language: Languages::EN,
    score: "Score",
    max_score: "Maximum score",
    name_of_game: "Squash the creeps!",
    start_button: "Play",
    language_button: "Select language",
    improvement_shield: "Shield is active!",
    improvement_slow_creeps: "Creeps is slow!",
});

/// This variable store translation to Russian.
static RU_LANGUAGE: LazyLock<LanguageText> = LazyLock::new(|| LanguageText {
    language: Languages::RU,
    score: "Счет",
    max_score: "Максимальный счет",
    name_of_game: "Раздави жуть!",
    start_button: "Играть",
    language_button: "Выбрать язык",
    improvement_shield: "Щит активен!",
    improvement_slow_creeps: "Жуть замедленна!",
});

/// This class store a UI data.
#[derive(GodotClass)]
#[class(base = Control)]
pub struct UserInterface {
    /// The current language.
    current_language: &'static LanguageText,

    /// Maximum score in current game.
    max_score: i64,

    /// Current score while player alive.
    score: i64,

    base: Base<Control>,
}

#[godot_api]
impl IControl for UserInterface {
    fn init(base: Base<Control>) -> Self {
        Self {
            current_language: &RU_LANGUAGE,
            max_score: 0,
            score: 0,
            base
        }
    }

    fn ready(&mut self) {
        // update UI to default language
        self.update_text_from_language();

        // connect 'pressed' signal for language button
        self.base()
            .get_node_as::<Button>("MainHUD/LanguageButton")
            .signals()
            .pressed()
            .connect_obj(self, Self::on_language_button_pressed);
    }
}

#[godot_api]
impl UserInterface {
    /// Setup interface to new language.
    fn update_text_from_language(&mut self) {
        let language = self.current_language;

        // update max score
        self.base()
            .get_node_as::<Label>("MaxScoreLabel")
            .set_text(&format!("{}: {}", language.max_score, self.max_score));

        // update score
        self.base()
            .get_node_as::<Label>("ScoreLabel")
            .set_text(&format!("{}: {}", language.score, self.score));

        // update language button
        self.base()
            .get_node_as::<Button>("MainHUD/LanguageButton")
            .set_text(&format!("{} (ru/en)", language.language_button));

        // update start button
        self.base()
            .get_node_as::<Button>("MainHUD/StartButton")
            .set_text(language.start_button);

        // update text logo
        self.base()
            .get_node_as::<Label>("MainHUD/NameOfGame")
            .set_text(language.name_of_game);

        // update positions for text
        match self.current_language.language {
            Languages::EN => self
                .base()
                .get_node_as::<Label>("MainHUD/NameOfGame")
                .set_position(Vector2::new(197.0, 190.0)),
            Languages::RU => self
                .base()
                .get_node_as::<Label>("MainHUD/NameOfGame")
                .set_position(Vector2::new(236.0, 192.0)),
        }
    }

    /// Setup UI to startup new game.
    pub fn start_new_game(&mut self) {
        // set score to zero
        self.base()
            .get_node_as::<Label>("ScoreLabel")
            .set_text(&format!("{}: 0", self.current_language.score));
        self.score = 0;

        // hide main hud
        self.base().get_node_as::<ColorRect>("MainHUD").hide();

        // deactivate improvements in UI
        self.set_improvement(Improvements::None);
    }

    /// Setup improvement text.
    pub fn set_improvement(&mut self, improvement: Improvements) {
        // update UI
        let mut improvement_label = self.base().get_node_as::<Label>("Improvement");
        improvement_label.show();

        match improvement {
            Improvements::SlowCreeps => {
                improvement_label.set_text(self.current_language.improvement_slow_creeps);
            }
            Improvements::PlayerShield => {
                improvement_label.set_text(self.current_language.improvement_shield);
            }
            Improvements::None => improvement_label.hide(),
        }
    }

    /// Update score on mob squashed.
    pub fn on_mob_squashed(&mut self) {
        // update score
        self.score += 1;
        self.base()
            .get_node_as::<Label>("ScoreLabel")
            .set_text(&format!("{}: {}", self.current_language.score, self.score));

        // update max score
        if self.score > self.max_score {
            self.max_score = self.score;
            self.base()
                .get_node_as::<Label>("MaxScoreLabel")
                .set_text(&format!(
                    "{}: {}",
                    self.current_language.max_score, self.max_score
                ));
        }
    }

    /// Set new language if button pressed.
    fn on_language_button_pressed(&mut self) {
        // set new language
        match self.current_language.language {
            Languages::RU => self.current_language = &EN_LANGUAGE,
            Languages::EN => self.current_language = &RU_LANGUAGE,
        }

        // update UI to new language
        self.update_text_from_language();
    }
}
