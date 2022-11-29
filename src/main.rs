use std::collections::HashMap;
use chrono::prelude::*;
use std::process::Command;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "walksanator", version = "v0.0.1", about = "A simple program that prints when my class period ends")]
struct Args {
    #[arg(short,long, help = "disables printing the time")]
    notime: bool
}

#[derive(Clone)]
struct Block {
    starts: NaiveTime,
    ends: NaiveTime,
    name: String,
}

#[derive(Clone)]
struct Schedule {
    blocks: Vec<Block>,
    start_of_day: NaiveTime,
    end_of_day: NaiveTime
}
impl Schedule {
    fn get_block(&self, dt: &NaiveTime) -> Option<Block> {
        for block in &self.blocks {
            if (dt < &block.ends) && (dt > &block.starts) {
                return Some(block.clone())
            }
        };
        None
    }
}

fn load_scheduel() -> HashMap<Weekday,Schedule> {
    let mut outputs = HashMap::new();
    //create scheduels for the days of the week (todo: read from file)
    let mon = Schedule {
        blocks: vec![
            Block { 
                starts: NaiveTime::from_hms(8,30, 0),
                ends: NaiveTime::from_hms(9,10,0),
                name: "Per. 1".to_string()
            },
            Block { 
                starts: NaiveTime::from_hms(9,16, 0),
                ends: NaiveTime::from_hms(9,58,0),
                name: "Per. 2".to_string()
            },
            Block { 
                starts: NaiveTime::from_hms(10,4, 0),
                ends: NaiveTime::from_hms(10,44,0),
                name: "Per. 3".to_string()
            },
            Block { 
                starts: NaiveTime::from_hms(10,50, 0),
                ends: NaiveTime::from_hms(11,30,0),
                name: "Per. 4".to_string()
            },
            Block { 
                starts: NaiveTime::from_hms(11,36, 0),
                ends: NaiveTime::from_hms(12,16,0),
                name: "Per. 5".to_string()
            },
            Block { 
                starts: NaiveTime::from_hms(12,52, 0),
                ends: NaiveTime::from_hms(13,32,0),
                name: "Per. 6".to_string()
            }
        ],
        start_of_day: NaiveTime::from_hms(9,16,0),
        end_of_day: NaiveTime::from_hms(13,32,0)
    };
    let wed_fri = Schedule { 
        blocks:  vec![
            Block {
                starts: NaiveTime::from_hms(8, 30, 0),
                ends: NaiveTime::from_hms(10,2,0),
                name: "Per. 2".to_string()
            },
            Block {
                starts: NaiveTime::from_hms(10,10, 0),
                ends: NaiveTime::from_hms(11,50,0),
                name: "Per. 4".to_string()
            },
            Block {
                starts: NaiveTime::from_hms(13, 5, 0),
                ends: NaiveTime::from_hms(14,37,0),
                name: "Per. 6".to_string()
            }
        ],
        start_of_day: NaiveTime::from_hms(9,16,0),
        end_of_day: NaiveTime::from_hms(14,37,0)
    };
    let tue_thur = Schedule { 
        blocks:  vec![
            Block {
                starts: NaiveTime::from_hms(8, 30, 0),
                ends: NaiveTime::from_hms(10,2,0),
                name: "Per. 1".to_string()
            },
            Block {
                starts: NaiveTime::from_hms(10,10, 0),
                ends: NaiveTime::from_hms(11,50,0),
                name: "Per. 3".to_string()
            },
            Block {
                starts: NaiveTime::from_hms(13, 5, 0),
                ends: NaiveTime::from_hms(14,37,0),
                name: "Per. 5".to_string()
            }
        ],
        start_of_day: NaiveTime::from_hms(9,16,0),
        end_of_day: NaiveTime::from_hms(14,37,0)
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

fn get_upgradable_packages() -> usize {
    if let Ok(updates) = Command::new("apt")
        .args(["list","--upgradable"])
        .output() {
            if updates.status.code().unwrap_or(1) == 0 {
                let tmp = &updates.stdout.into_boxed_slice();
                let o = String::from_utf8_lossy(tmp);
                o.lines().count() - 1
            } else {
                eprintln!("! apt list failed");
                0
            }
        } else {
            0
        }
}

fn main() {
    let args = Args::parse();
    let datetime = Local::now();
    let tod = datetime.time();
    let sc = load_scheduel();
    //println!("{}",datetime.format("%a %H:%M"));
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
                println!("{}",pkgs_count);
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
