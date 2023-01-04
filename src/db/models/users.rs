use diesel::prelude::*;
use super::super::schema::{userfollows, users};

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: String,
    pub username: String,
}

#[juniper::graphql_object]
impl User {
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn username(&self) -> &str {
        self.username.as_str()
    }
}

#[derive(Queryable, Insertable, Debug, Clone, PartialEq, Eq)]
#[diesel(table_name = userfollows)]
pub struct UserFollow {
    pub id: i32,
    pub follower: String,
    pub following: String,
}
