use diesel::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, AsExpression, FromSqlRow, PartialEq)]
#[diesel(sql_type=Text)]
pub enum TaskStatus {
    #[serde(rename = "In progress")]
    NotStarted,
    #[serde(rename = "Completed")]
    Completed,
    #[serde(rename = "Not started")]
    InProgress,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub completed: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
