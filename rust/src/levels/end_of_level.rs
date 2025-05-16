//! Данный модуль содержит структуры данных выхода из уровня. Они нужны для того,
//! чтобы главная сцена знала как игрок вышел из уровня и что собрал на нем. Это
//! дает возможность главной сцене открывать новые уровни, сохранять достигнутый
//! прогресс и т.д.

use std::sync::{LazyLock, RwLock};

/// Данный enum содержит информацию о том, как игрок вышел с уровня.
#[derive(Default, Debug)]
#[allow(dead_code)]
pub enum HowPlayerExitedFromLevel {
    ExitFromMenu,
    EndOfLevel,

    #[default]
    UnknownExited
}

/// Глобальный класс сохраняющий в глобальном доступе данные
/// переданные после прохождения уровня. Данный класс содержит данные о конце уровня. 
/// То есть о том, как игрок его закончил (дошел до выхода или вышел из него через меню).
/// А также дополнительные данные (на данный момент они отсутствуют, но класс предполагает 
/// их возможное наличие).
#[derive(Default, Debug)]
pub struct ExitFromLevelData {
    pub exited: HowPlayerExitedFromLevel
}

/// Переменная singleton, которая используется для доступа
/// к данным после прохождения уровня.
pub static EXIT_FROM_LEVEL_DATA: LazyLock<RwLock<ExitFromLevelData>> = LazyLock::new( ||
    RwLock::new(ExitFromLevelData::default())
);
