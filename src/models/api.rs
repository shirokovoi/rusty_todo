use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    #[test]
    fn list_ids_parse() {
        unimplemented!();
    }

    #[test]
    fn priorities_parse() {
        unimplemented!();
    }

    #[test]
    fn list_order_modify_parse() {
        unimplemented!();
    }

    #[test]
    fn todo_entry_parse() {
        unimplemented!();
    }

    #[test]
    fn list_response_parse() {
        unimplemented!();
    }

    #[test]
    fn create_entry_parse() {
        unimplemented!();
    }

    #[test]
    fn user_info_parse() {
        unimplemented!();
    }

    #[test]
    fn list_id_parse() {
        unimplemented!();
    }
}

#[derive(Serialize)]
pub struct ListIds {
    pub list_ids: Vec<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct Priorities {
    pub entry_id: u32,
    pub priority: u32,
}

#[derive(Deserialize, Serialize)]
pub struct ListOrderModify {
    pub version: u32,
    pub priorities: Vec<Priorities>,
}

#[derive(Deserialize)]
pub struct PagingParameters {
    pub count: u32,
    pub offset: u32,
}

#[derive(Serialize)]
pub struct TodoEntry {
    pub id: u32,
    pub priority: u32,
    pub description: String,
}

#[derive(Serialize)]
pub struct ListResponse {
    pub version: u32,
    pub total_entries: u32,
    pub entries: Vec<TodoEntry>,
}

#[derive(Deserialize, Serialize)]
pub struct EntryCreate {
    pub value: String,
    pub version: u32,
}

#[derive(Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ListId {
    pub list_id: u32,
}

#[derive(Deserialize)]
pub struct EntryLocation {
    pub list_id: u32,
    pub entry_id: u32,
    pub version: u32,
}
