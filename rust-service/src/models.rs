use serde::Deserialize;
use serde_json::Value;

/// Student detail payload from Node `GET /api/v1/students/:id`.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudentReportData {
    pub id: i64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub system_access: Option<bool>,
    pub phone: Option<String>,
    pub gender: Option<String>,
    pub dob: Option<Value>,
    pub class: Option<String>,
    pub section: Option<String>,
    /// Numeric in Postgres, so it arrives as a JSON number rather than a string.
    pub roll: Option<Value>,
    pub father_name: Option<String>,
    pub father_phone: Option<String>,
    pub mother_name: Option<String>,
    pub mother_phone: Option<String>,
    pub guardian_name: Option<String>,
    pub guardian_phone: Option<String>,
    pub relation_of_guardian: Option<String>,
    pub current_address: Option<String>,
    pub permanent_address: Option<String>,
    pub admission_date: Option<Value>,
    pub reporter_name: Option<String>,
}

impl StudentReportData {
    pub fn display(value: &Option<String>) -> String {
        value
            .as_ref()
            .map(|v| v.trim().to_string())
            .filter(|v| !v.is_empty())
            .unwrap_or_else(|| "-".to_string())
    }

    pub fn display_value(value: &Option<Value>) -> String {
        match value {
            Some(Value::String(s)) if !s.trim().is_empty() => s.clone(),
            Some(Value::Bool(b)) => b.to_string(),
            Some(Value::Number(n)) => n.to_string(),
            Some(other) => other.to_string().trim_matches('"').to_string(),
            None => "-".to_string(),
        }
    }
}
