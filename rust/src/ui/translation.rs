//! Данный файл содержит данные для перевода UI.

use serde_derive::{Deserialize, Serialize};

/// В данном перечислении находятся все языки,
/// на которые переведена игра.
#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub enum Languages {
    EN,
    RU,
}

