use chrono::{Local, Duration, Date};

// Could replace with string
#[derive(Debug, PartialEq, Clone)]
pub enum PoolType {
    SCY,
    SCM,
    LCM,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Practice {
    name: String,
    date: Date<Local>,
    pool_type: PoolType,
    sets: Vec<Set>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetData {
    label: Option<String>,
    reps: u32,
    attrs: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Set {
    Multiline { subsets: Vec<Set>, data: SetData },
    Single { data: SetData, time: Duration },
    Text(String),
}