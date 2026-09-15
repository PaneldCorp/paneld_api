use uuid::Uuid;
use chrono::NaiveDate;

#[derive(sqlx::FromRow, Debug)]
pub struct Author
{
    pub id: Uuid,
    pub japanese_name: String,
    pub name: String,
    pub birthday: NaiveDate,
    pub website: Option<String>,
    pub description: Option<String>
}
