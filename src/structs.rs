use std::collections::HashMap;

use schemars::JsonSchema;

use chrono::prelude::*;
use serde::{Serialize,Deserialize};

// actual real usable rust structs
#[derive(Clone)]
pub struct Block {
    pub starts: NaiveTime,
    pub ends: NaiveTime,
    pub name: String,
}

#[derive(Clone)]
pub struct Schedule {
    pub blocks: Vec<Block>,
    pub start_of_day: NaiveTime,
    pub end_of_day: NaiveTime
}

impl Schedule {
    //clippy was being drunk and saying this was dead code, it is not it is used
    #[allow(dead_code)]
    pub fn get_block(&self, dt: &NaiveTime) -> Option<Block> {
        for block in &self.blocks {
            if (dt < &block.ends) && (dt > &block.starts) {
                return Some(block.clone())
            }
        };
        None
    }
}

//all the structs that are converted from json
#[derive(Serialize,Deserialize,JsonSchema,Debug)]
pub struct Time {
    h: u8,
    m: u8
}

#[derive(Serialize,Deserialize,JsonSchema,Debug)]
pub struct JsonBlock {
    starts: Time,
    ends: Time,
    name: String
}

#[derive(Serialize,Deserialize,JsonSchema,Debug)]
pub struct Day {
    blocks: Vec<JsonBlock>,
    start_of_day: Time,
    end_of_day: Time,
}

#[derive(Serialize,Deserialize,JsonSchema,Debug)]
pub struct Week {
    mon: Day,
    tues: Day,
    wens: Day,
    thurs: Day,
    fri: Day
}

//convert from the json types to rust types

impl Into<NaiveTime> for Time {
    fn into(self) -> NaiveTime {
        NaiveTime::from_hms_opt(self.h.into(), self.m.into(),0).unwrap_or_default()
    }
}
impl Into<Block> for JsonBlock {
    fn into(self) -> Block {
        Block {
            starts: self.starts.into(),
            ends: self.ends.into(),
            name: self.name.clone()
        }
    }
}
impl Into<Schedule> for Day {
    fn into(self) -> Schedule {
        let mut blocks = vec![];
        for k in self.blocks {
            blocks.push(k.into())
        }
        Schedule {
            blocks,
            start_of_day: self.start_of_day.into(),
            end_of_day: self.end_of_day.into()
        }
    }
}

impl Into<HashMap<Weekday,Schedule>> for Week {
    fn into(self) -> HashMap<Weekday,Schedule> {
        let mut map = HashMap::new();
        map.insert(Weekday::Mon, self.mon.into());
        map.insert(Weekday::Tue, self.tues.into());
        map.insert(Weekday::Wed, self.wens.into());
        map.insert(Weekday::Thu, self.thurs.into());
        map.insert(Weekday::Fri, self.fri.into());
        map
    }
}