pub struct EntryPriority {
    pub entry_id: u32,
    pub priority: u32,
}

pub struct UserIdentity {
    pub username: String,
    pub id: u32,
}

pub struct ListEntry {
    pub id: u32,
    pub priority: u32,
    pub value: String,
}

pub struct List {
    pub version: u32,
    pub entiries_count: u32,
    pub entries: Vec<ListEntry>,
}
