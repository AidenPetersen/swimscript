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
    date: chrono::Date<Tz>,
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
    Multiline { subsets: Vec<Set>, data: EntryData },
    Single { data: EntryData, time: chrono::Duration },
    Text(String),
}