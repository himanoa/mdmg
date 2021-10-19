#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Scaffold {
    Complete {
        file_name: String,
        file_body: String,
    },
    Pending {
        file_name: String,
    },
}
