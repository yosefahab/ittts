use chrono::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct TicketType {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TicketType {
    pub fn new(id: usize) -> Self {
        TicketType {
            id,
            title: "Network Connectivity Issue".to_string(),
            description: "Some long long long long long long long description about network"
                .to_string(),
            status: "Closed".to_string(),
            priority: "Low".to_string(),
            category: "Software".to_string(),
            ..Default::default()
        }
    }

    pub fn created_at_str(&self) -> String {
        self.created_at.format("%d/%m/%y %H:%M:%S").to_string()
    }

    pub fn updated_at_str(&self) -> String {
        self.updated_at.format("%d/%m/%y %H:%M:%S").to_string()
    }
}
