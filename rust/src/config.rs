//! Данный файл содержит реализацию конфига. Он представлен в
//! виде singleton. Его задачи это синхронизация данных в UI
//! (язык, плашки настроек, например HSlider), хранение настроек
//! и прогресса в файле.

use crate::ui::translation::{LanguageText, Languages, EN_LANGUAGE, RU_LANGUAGE};

use std::fs::File;
use std::io::Read;
use std::env;
use std::os::unix::fs::FileExt;
use std::sync::{LazyLock, Mutex};

use godot::classes::AudioServer;
use godot::global::godot_error;
use serde_derive::{Serialize, Deserialize};
use serde_json::{to_string, from_slice};

/// Данная структура содержит данные о настройка игры.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Settings {
    pub language: Languages,
    pub sound_effects_volume: f64,
    pub music_volume: f64, 
}

/// Данная структуры содержит данные о прогрессе игрока.
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct GameProgress {
    pub max_level: i64
}

/// Данная структура содержит реализацию конфига.
pub struct ConfigData {
    pub settings: Settings,
    pub progress: GameProgress,
    file: File,
}

impl ConfigData {
    /// Возвращает ошибку или конфиг.
    #[allow(dead_code)]
    fn new() -> Result<Self, std::io::Error> {
        let file_path: String;
        
        if cfg!(debug_assertions) && cfg!(target_os = "linux") {
            file_path = "/home/user/code/godot/squash_the_creeps_rust/config.json".to_string();
        } else {
            file_path = 
                env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string() + "/config.json";
        }

        // Структура для получения данных из файла.
        #[derive(Deserialize, Debug)]
        struct Deserialize {
            pub settings: Settings,
            pub progress: GameProgress,
        }

        // Открываем файл и получаем данные в виде строки.
        let mut file = File::options()
            .append(false)
            .write(true)
            .read(true)
            .open(file_path)?;

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        // Получаем десериализованные данные.
        let data: Deserialize = from_slice(buf.as_bytes()).expect("from slice expect");

        // Настраиваем звук.
        let mut audio_server = AudioServer::singleton();

        // Громкость звуковых эффектов.
        let sound_effects_volume_idx = audio_server.get_bus_index("SoundEffects");
        audio_server.set_bus_volume_db(
            sound_effects_volume_idx,
            data.settings.sound_effects_volume as f32
        );

        // Громкость музыки.
        let music_volume_idx = audio_server.get_bus_index("Music");
        audio_server.set_bus_volume_db(music_volume_idx, data.settings.music_volume as f32);

        // Возвращаем структуру.
        Ok(Self {
            settings: data.settings,
            progress: data.progress,
            file,
        })
    }

    /// Данный метод записывает данные в файл конфига.
    pub fn write(&mut self) {
        /// Структура для сериализации.
        #[derive(Serialize)]
        struct Serialize {
            pub settings: Settings,
            pub progress: GameProgress,
        }

        // Получаем данные для сериализации.
        let serialize = Serialize {
            settings: self.settings,
            progress: self.progress
        };

        // Получаем результат сериализации.
        let write_data = match to_string(&serialize) {
            Ok(string) => string,
            Err(error) => { 
                godot_error!("Error serialize data: {error:?}"); 
                return; 
            }
        };

        // Записываем в файл данные.
        if let Err(error) = self.file.set_len(0) {
            godot_error!("Can't set file data to zero! Error: {error:?}");
        } else if let Err(error) = self.file.write_all_at(write_data.as_bytes(), 0) {
            godot_error!(
                "Error write data to {}: {error:?}", 
                env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string() + "/config"
            );
        }
    }

    /// Получаем текущий язык из конфига.
    pub fn get_language(&self) -> &'static LanguageText {
        match self.settings.language {
            Languages::RU => &RU_LANGUAGE,
            Languages::EN => &EN_LANGUAGE,
        }
    }
}

pub static CONFIG: LazyLock<Mutex<ConfigData>> = LazyLock::new(|| Mutex::new(ConfigData::new().unwrap()));
