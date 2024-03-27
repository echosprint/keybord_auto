use crate::config::*;
use enigo::*;
use std::thread;
use std::time::Duration;

pub trait ClickModify {
    fn move_and_click(&mut self, pos: &Pos, duration: u64);
    fn move_and_change_date(&mut self, pos: &Pos, date_str: &str, duration: u64);
    fn move_and_change_str(&mut self, pos: &Pos, del_num: i32, change_str: &str, duration: u64);
}

impl ClickModify for Enigo {
    fn move_and_click(&mut self, pos: &Pos, duration: u64) {
        self.mouse_move_to(pos.x, pos.y);
        self.mouse_click(MouseButton::Left);
        thread::sleep(Duration::from_millis(duration));
    }

    fn move_and_change_date(&mut self, pos: &Pos, date_str: &str, duration: u64) {
        let del_num: i32 = date_str.len().try_into().unwrap();
        self.move_and_change_str(pos, del_num, date_str, duration)
    }

    fn move_and_change_str(&mut self, pos: &Pos, del_num: i32, change_str: &str, duration: u64) {
        self.move_and_click(pos, 10);
        (0..del_num).for_each(|_| self.key_click(Key::Backspace));
        self.key_sequence(&change_str);
        sleep(duration);
    }
}

pub fn sleep(duration: u64) {
    thread::sleep(Duration::from_millis(duration));
}
