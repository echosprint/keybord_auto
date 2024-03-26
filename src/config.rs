use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{read_to_string, write};
use std::path::PathBuf;

const TOML_FILE: &str = "pos_config.toml";

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub mail_num: i32,
    pub wait_edit: u64,
    pub sent_mail: Pos,
    pub add_attach: Attachment,
    pub mail: Vec<MailPos>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Attachment {
    pub button: Pos,
    pub file_list: Pos,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MailPos {
    pub start_pos: Pos,
    pub edit_menu_pos: Pos,
    pub title_date_pos: Pos,
    pub content_date_pos: Pos,
    pub content_attach_pos: Pos,
    pub title_attach_pos: Pos,
    pub final_pos: Pos,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

pub fn config() -> Config {
    let exe_path: PathBuf = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let toml_str = read_to_string(exe_dir.join(TOML_FILE)).expect("pos_config.toml not found");
    let config: Config = toml::from_str(&toml_str).expect("cannot parse the toml file");

    config
}

pub fn write_back_config(new_config: &Config) {
    let exe_path: PathBuf = env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let toml = toml::to_string(new_config).unwrap();
    write(&exe_dir.join(TOML_FILE), toml).unwrap();
}
