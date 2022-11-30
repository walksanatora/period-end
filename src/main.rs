use std::collections::HashMap;
use chrono::prelude::*;
use clap::Parser;

mod structs;
use structs::*;

mod update_counter;
use update_counter::get_upgradable_packages;

#[derive(Parser, Debug)]
#[command(author = "walksanator", version = "v0.0.1", about = "A simple program that prints when my class period ends")]
struct Args {
    #[arg(short,long, help = "disables printing the time")]
    notime: bool,
    #[arg(short,long,help = "path to the json file", default_value = "sched.json")]
    sched_path: String
}

fn load_scheduel(args: &Args) -> Option<HashMap<Weekday,Schedule>> {
    let string = std::fs::read_to_string(args.sched_path.clone());
    if let Ok(cstring) = string {
        let content = cstring.into_boxed_str();
        let content = serde_json::from_str::<Week>(&content);
        if let Ok(week) = content {
            #[cfg(debug_assertions)]
            println!("Unpacking week");
            Some(week.into())
        } else {
            None
        }
    } else {
        None
    }
}

fn main() {
    let args = Args::parse();
    let datetime = Local::now();
    let tod = datetime.time();
    let sc = load_scheduel(&args);
    if let Some(sched) = sc {
        #[cfg(debug_assertions)]
        println!("{}",datetime.format("%a %H:%M"));
        let day_sched = sched.get(&datetime.weekday());
        let time_disabled = std::env::var("NOTIME").is_ok() || args.notime;
        let formatted_date = if time_disabled {
            "".to_string()
        } else {
            tod.format("%I:%M %P").to_string()
        };
        match day_sched {
            Some(sched) => {
                let blk = sched.get_block(&tod);
                if let Some(ublk) = blk {
                        let mut line = format!("{}{} {}-{}",
                        if time_disabled {"".to_string()} else {format!("({}) ",formatted_date)},
                        ublk.name,
                        ublk.starts.format("%I:%M"),
                        ublk.ends.format("%I:%M %P"));
                        let pkgs_count = get_upgradable_packages();
                        if pkgs_count > 0 {let pkgs = format!(" ({})",pkgs_count).into_boxed_str();
                        line += &pkgs};
                        println!("{}",line)
                } else if (tod > sched.end_of_day) || (tod < sched.start_of_day) {//2pm
                    let mut line = format!("{}",formatted_date);
                    let pkgs_count = get_upgradable_packages();
                    #[cfg(debug_assertions)]
                    println!("{}",pkgs_count);
                    if pkgs_count > 0 {
                        let pkgs = format!(" ({})",pkgs_count).into_boxed_str();
                        line += &pkgs
                    };
                    println!("{}",line);
                } else {
                    let mut line = format!("{} passing",formatted_date);
                    let pkgs_count = get_upgradable_packages();
                    #[cfg(debug_assertions)]
                    println!("Packages {}",pkgs_count);
                    if pkgs_count > 0 {
                        let pkgs = format!(" ({})",pkgs_count).into_boxed_str();
                        line += &pkgs
                    };
                    println!("{}",line); 
                }
            },
            None => {
                let mut line = format!("({})",formatted_date);
                let pkgs_count = get_upgradable_packages();
                #[cfg(debug_assertions)]
                println!("{}",pkgs_count);
                if pkgs_count > 0 {
                    let pkgs = format!(" ({})",pkgs_count).into_boxed_str();
                    line += &pkgs
                };
                println!("{}",line);
            }
        }
    } else {
        println!("!sched.json")
    }
}
