mod config;
mod mail;
mod util;

use chrono::Datelike;
use chrono::Local;
use clap::Parser;
use mail::*;

use crate::config::{config, write_back_config};

/// Automatically keyboard manipulation
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// to process the specified mail
    #[arg(short, long, default_value_t = 0,value_parser=clap::value_parser!(i32).range(0..=5))]
    mail: i32,
    /// other mails had been sent in order to calculate the offset
    #[arg(short, long, default_value_t = -1)]
    sent: i32,
    /// automatically send all the mails
    #[arg(short, long, action)]
    all: bool,
}

fn main() {
    let now = Local::now(); // Get the current date and time
    let formatted_date = format!("{:04}{:02}{:02}", now.year(), now.month(), now.day());
    println!("Formatted date: {}", formatted_date);

    let args = Args::parse();
    // println!("{:?}", args);

    let mut pos_config = config();
    // println!("{:#?}", pos_config);

    let mut mail_num = if args.all {
        1
    } else {
        match args.mail {
            0 => {
                let mail_num = pos_config.mail_num;
                pos_config.mail_num = mail_num % 5 + 1;
                write_back_config(&pos_config);
                mail_num
            }
            _ => args.mail,
        }
    };

    loop {
        if mail_num > 5 {
            break;
        }

        if args.sent >= 0 {
            select_the_sent_mail(&pos_config.sent_mail, args.sent);
        }

        println!("process the mail {}", mail_num);

        match mail_num {
            1 => edit_mail1(
                &pos_config.mail[0],
                &formatted_date,
                pos_config.wait_edit,
                &pos_config.add_attach,
            ),
            2 => edit_mail2(
                &pos_config.mail[1],
                &formatted_date,
                pos_config.wait_edit,
                &pos_config.add_attach,
            ),
            3 => edit_mail3(
                &pos_config.mail[2],
                &formatted_date,
                pos_config.wait_edit,
                &pos_config.add_attach,
            ),
            4 => edit_mail4(&pos_config.mail[3], &formatted_date, pos_config.wait_edit),
            5 => edit_mail5(&pos_config.mail[4], &formatted_date, pos_config.wait_edit),
            _ => (),
        }

        if !args.all {
            move_to_sent_button();
            break;
        } else {
            mail_num += 1;
            click_to_sent_button();
        }
    }
}
