use bevy::prelude::*;
use directories::ProjectDirs;
use std::{fs::{File, read_to_string}, io::{Write, Seek, SeekFrom}};

#[derive(Component)]
pub struct Head {
    pub body: Vec<Entity>
}

#[derive(Component)]
pub struct Velocity { 
    pub vector: Vec3
}

#[derive(Component)]
pub struct MoveTimer {
    pub timer: Timer
}

#[derive(Component)]
pub struct Tail;

#[derive(Component)]
pub struct Apple;

#[derive(Component)]
pub struct Menu;
#[derive(Component)]
pub struct PlayButton;
#[derive(Component)]
pub struct QuitButton;

pub struct AppleEaten;
pub struct SpawnTail(pub Entity);

#[derive(Resource)]
pub struct ScoreManager {
    pub score: u32,
    pub high_score: u32,
    file: File
}

impl Default for ScoreManager {
    fn default() -> Self {
        let proj_dirs = ProjectDirs::from("com", "rainbowasteroids", "bevy-snake").unwrap();
        let save_file_path = {
            let mut path = proj_dirs.data_dir().to_path_buf();
            path.push("save.dat");
            path.into_boxed_path()
        };

        let high_score = if save_file_path.is_file() {
            read_to_string(&save_file_path).expect("Failed to read save file").parse::<u32>().unwrap_or_else(|e| {
                println!("Failed to read save file: {}", e.to_string());
                0
            })
        } else {
            0
        };

        std::fs::create_dir_all(proj_dirs.data_dir()).expect("Could not create directory for save file");
        let mut result = ScoreManager { 
            score: 0, 
            high_score, 
            file: File::create(save_file_path).expect("Unable to open save file for writing")
        };

        result.sync();

        result
    }
}

impl ScoreManager {
    pub fn sync(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
        }

        self.file.seek(SeekFrom::Start(0)).expect("Could not seek");
        write!(self.file, "{}", self.high_score).expect("Could not write to save file");
        self.file.flush().expect("Could not flush to save file");

        self.score = 0;
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    StartScreen,
    LoseScreen,
    WinScreen,
    Playing,
}

impl AppState {
    pub fn title(&self) -> &str {
        match self {
            AppState::StartScreen => "Snake",
            AppState::LoseScreen => "Game Over",
            AppState::WinScreen => "You Win",
            AppState::Playing => panic!("AppState::Playing.title() is undefined")
        }
    }

    pub fn play_button_title(&self) -> &str {
        if let AppState::Playing = self {
            panic!("AppState::Playing.play_button_title() is undefined");
        } else if let AppState::StartScreen = self {
            "Play"
        } else {
            "Play again"
        }
    }
}
