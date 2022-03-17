use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Tag {
    pub id: String,
    pub summary: String,
    pub description: Option<String>,
    pub label: String,
}
