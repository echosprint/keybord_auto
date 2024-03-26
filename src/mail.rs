use crate::config::*;
use crate::util::{sleep, ClickModify};
use enigo::*;

pub fn edit_mail1(mail_pos: &MailPos, date_str: &str, wait_edit: u64, attach: &Attachment) {
    edit_mail_with_attachment(mail_pos, date_str, wait_edit);
    add_attachment(attach, 2);
}

pub fn edit_mail2(mail_pos: &MailPos, date_str: &str, wait_edit: u64, attach: &Attachment) {
    edit_mail_with_attachment(mail_pos, date_str, wait_edit);
    add_attachment(attach, 1);
}

pub fn edit_mail3(mail_pos: &MailPos, date_str: &str, wait_edit: u64, attach: &Attachment) {
    edit_mail_with_attachment(mail_pos, date_str, wait_edit);
    add_attachment(attach, 0);
}

pub fn edit_mail4(mail_pos: &MailPos, date_str: &str, wait_edit: u64) {
    edit_mail_without_attachment(mail_pos, date_str, wait_edit);
}

pub fn edit_mail5(mail_pos: &MailPos, date_str: &str, wait_edit: u64) {
    edit_mail_without_attachment(mail_pos, date_str, wait_edit);
}

fn edit_mail_with_attachment(mail_pos: &MailPos, date_str: &str, wait_edit: u64) {
    let mut enigo = Enigo::new();
    // start postion for edit
    enigo.move_and_click(&mail_pos.start_pos, 100);
    // right-click on the mail
    enigo.mouse_click(MouseButton::Right);
    sleep(300);
    // click the edit menu
    enigo.move_and_click(&mail_pos.edit_menu_pos, wait_edit);
    // edit the date in title
    enigo.move_and_change_date(&mail_pos.title_date_pos, date_str, 500);
    // edit the date in content
    enigo.move_and_change_date(&mail_pos.content_date_pos, date_str, 100);
    // remove the attachment in content
    enigo.move_and_click(&mail_pos.content_attach_pos, 10);
    enigo.key_click(Key::Backspace);
    sleep(100);
    // remove the attachment in title
    enigo.move_and_click(&mail_pos.title_attach_pos, 10);
    enigo.key_click(Key::Delete);
    sleep(500);
    // move cursor to final position
    enigo.move_and_click(&mail_pos.final_pos, 100);
}

fn edit_mail_without_attachment(mail_pos: &MailPos, date_str: &str, wait_edit: u64) {
    let mut enigo = Enigo::new();
    // start postion for edit
    enigo.move_and_click(&mail_pos.start_pos, 100);
    // right-click on the mail
    enigo.mouse_click(MouseButton::Right);
    sleep(300);
    // click the edit menu
    enigo.move_and_click(&mail_pos.edit_menu_pos, wait_edit);
    // edit the date in title
    enigo.move_and_change_date(&mail_pos.title_date_pos, date_str, 500);
    // edit the date in content
    enigo.move_and_change_date(&mail_pos.content_date_pos, date_str, 100);
    // move cursor to final position
    enigo.move_and_click(&mail_pos.final_pos, 100);
}

fn add_attachment(attach: &Attachment, file_num: i32) {
    let mut enigo = Enigo::new();
    // click on the add attachment button
    enigo.move_and_click(&attach.button, 500);
    // select the file
    enigo.move_and_click(
        &Pos {
            x: attach.file_list.x,
            y: attach.file_list.y + file_num * 20,
        },
        500,
    );
    // double click the file to select
    enigo.mouse_click(MouseButton::Left);
    enigo.mouse_click(MouseButton::Left);
    sleep(300);
}

pub fn select_the_sent_mail(sent_pos: &Pos, sent_num: i32) {
    let mut enigo = Enigo::new();
    let mut y_offset = if sent_num == 0 { 0 } else { 40 };
    y_offset += sent_num * 70 + 4 * 70;
    enigo.move_and_click(
        &Pos {
            x: sent_pos.x,
            y: sent_pos.y + y_offset,
        },
        500,
    );
}

pub fn move_to_sent_button() {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(40, 40);
}

pub fn click_to_sent_button() {
    move_to_sent_button();
    sleep(10_000);
    let mut enigo = Enigo::new();
    enigo.mouse_click(MouseButton::Left);
    sleep(6000)
}
