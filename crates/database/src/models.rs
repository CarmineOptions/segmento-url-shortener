use chrono::{DateTime, Utc};
use common::{Link, NewLink};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::links;

#[derive(Debug, Clone, Queryable, Selectable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbLink {
    pub id: i64,
    pub short_code: String,
    pub target_url: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Link> for DbLink {
    fn from(item: Link) -> Self {
        DbLink {
            id: item.id,
            short_code: item.short_code,
            target_url: item.target_url,
            is_active: item.is_active,
            created_at: item.created_at,
        }
    }
}

impl From<DbLink> for Link {
    fn from(item: DbLink) -> Self {
        Link {
            id: item.id,
            short_code: item.short_code,
            target_url: item.target_url,
            is_active: item.is_active,
            created_at: item.created_at,
        }
    }
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = links)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct DbNewLink<'a> {
    pub target_url: &'a str,
}

impl<'a> From<NewLink<'a>> for DbNewLink<'a> {
    fn from(item: NewLink<'a>) -> Self {
        DbNewLink {
            target_url: item.target_url,
        }
    }
}
