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
    notime: bool
}

fn fallback_sched() -> HashMap<Weekday, Schedule> {
    #[cfg(debug_assertions)]
    println!("Defaulting schedele");
    let mut outputs = HashMap::new();
    //create scheduels for the days of the week (todo: read from file)
    let mon = Schedule {
        blocks: vec![
            Block { 
                starts: NaiveTime::from_hms_opt(8,30, 0).unwrap(),
                ends: NaiveTime::from_hms_opt(9,10,0).unwrap(),
                name: "Per. 1".to_string()
            },
            Block { 
                starts: NaiveTime::from_hms_opt(9,16, 0).unwrap(),
                ends: NaiveTime::from_hms_opt(9,58,0).unwrap(),
                name: "Per. 2".to_string()
            },
            Block { 
                starts: NaiveTime::from_hms_opt(10,4, 0).unwrap(),
                ends: NaiveTime::from_hms_opt(10,44,0).unwrap(),
                name: "Per. 3".to_string()
            },
            Block { 
                starts: NaiveTime::from_hms_opt(10,50, 0).unwrap(),
                ends: NaiveTime::from_hms_opt(11,30,0).unwrap(),
                name: "Per. 4".to_string()
            },
            Block { 
                starts: NaiveTime::from_hms_opt(11,36, 0).unwrap(),
                ends: NaiveTime::from_hms_opt(12,16,0).unwrap(),
                name: "Per. 5".to_string()
            },
            Block { 
                starts: NaiveTime::from_hms_opt(12,52, 0).unwrap(),
                ends: NaiveTime::from_hms_opt(13,32,0).unwrap(),
                name: "Per. 6".to_string()
            }
        ],
        start_of_day: NaiveTime::from_hms_opt(9,16,0).unwrap(),
        end_of_day: NaiveTime::from_hms_opt(13,32,0).unwrap()
    };
    let wed_fri = Schedule { 
        blocks:  vec![
            Block {
                starts: NaiveTime::from_hms_opt(8, 30, 0).unwrap(),
                ends: NaiveTime::from_hms_opt(10,2,0).unwrap(),
                name: "Per. 2".to_string()
            },
            Block {
                starts: NaiveTime::from_hms_opt(10,10, 0).unwrap(),
                ends: NaiveTime::from_hms_opt(11,50,0).unwrap(),
                name: "Per. 4".to_string()
            },
            Block {
                starts: NaiveTime::from_hms_opt(13, 5, 0).unwrap(),
                ends: NaiveTime::from_hms_opt(14,37,0).unwrap(),
                name: "Per. 6".to_string()
            }
        ],
        start_of_day: NaiveTime::from_hms_opt(9,16,0).unwrap(),
        end_of_day: NaiveTime::from_hms_opt(14,37,0).unwrap()
    };
    let tue_thur = Schedule { 
        blocks:  vec![
            Block {
                starts: NaiveTime::from_hms_opt(8, 30, 0).unwrap(),
                ends: NaiveTime::from_hms_opt(10,2,0).unwrap(),
                name: "Per. 1".to_string()
            },
            Block {
                starts: NaiveTime::from_hms_opt(10,10, 0).unwrap(),
                ends: NaiveTime::from_hms_opt(11,50,0).unwrap(),
                name: "Per. 3".to_string()
            },
            Block {
                starts: NaiveTime::from_hms_opt(13, 5, 0).unwrap(),
                ends: NaiveTime::from_hms_opt(14,37,0).unwrap(),
                name: "Per. 5".to_string()
            }
        ],
        start_of_day: NaiveTime::from_hms_opt(9,16,0).unwrap(),
        end_of_day: NaiveTime::from_hms_opt(14,37,0).unwrap()
    };
    //adding days to the output
    outputs.insert(
        Weekday::Mon,
        mon
    );
    outputs.insert(
        Weekday::Tue,
        tue_thur.clone()    
    );
    outputs.insert(
        Weekday::Wed,
        wed_fri.clone()    
    );
    outputs.insert(
        Weekday::Thu,
        tue_thur    
    );
    outputs.insert(
        Weekday::Fri,
        wed_fri    
    );
    outputs
}

fn load_scheduel() -> HashMap<Weekday,Schedule> {
    let string = std::fs::read_to_string("sched.json");
    if let Ok(cstring) = string {
        let content = cstring.into_boxed_str();
        let content = serde_json::from_str::<Week>(&content);
        if let Ok(week) = content {
            #[cfg(debug_assertions)]
            println!("Unpacking week");
            week.into()
        } else {
            fallback_sched()
        }
    } else {
        fallback_sched()
    }
}

fn main() {
    let args = Args::parse();
    let datetime = Local::now();
    let tod = datetime.time();
    let sc = load_scheduel();
    #[cfg(debug_assertions)]
    println!("{}",datetime.format("%a %H:%M"));
    let day_sched = sc.get(&datetime.weekday());
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
}
