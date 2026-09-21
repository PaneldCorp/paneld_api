use uuid::Uuid;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Author
{
    pub id: Uuid,
    pub japanese_name: String,
    pub name: String,
    pub birthday: NaiveDate,
    pub website: Option<String>,
    pub description: Option<String>,
    pub pub_date: NaiveDate,
    pub update_date: NaiveDate
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewAuthor
{
    pub japanese_name: String,
    pub name: String,
    pub birthday: NaiveDate,
    pub website: Option<String>,
    pub description: Option<String>,
}

impl Author
{
    pub fn from_new_author(new_author: NewAuthor, uuid: Uuid, now: NaiveDate) -> Self
    {
        let NewAuthor {
            japanese_name,
            name,
            birthday,
            website,
            description
        } = new_author;

        Author {
            id: uuid,
            japanese_name,
            name,
            birthday,
            website,
            description,
            pub_date: now,
            update_date: now
        }
    }
}
