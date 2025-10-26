use crate::schema::users;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use serde_json::json;

#[derive(Queryable, Selectable)]
#[diesel(table_name=crate::schema::users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<chrono::NaiveDateTime>,
}

impl User {
    pub fn get_all(conn: &mut SqliteConnection) -> Vec<User> {
        users.load::<User>(conn).expect("Error loading users")
    }

    pub fn get_by_id(conn: &mut SqliteConnection, user_id: i32) -> Option<User> {
        users
            .filter(id.eq(user_id))
            .first::<User>(conn)
            .optional()
            .expect("Error loading user by id")
    }

    pub fn get_by_first_id(conn: &mut SqliteConnection) -> Option<User> {
        users::table
            .order(users::id.desc())
            .select(User::as_select())
            .first::<User>(conn)
            .optional()
            .expect("Error loading newly created user")
    }

    pub fn delete_by_id(conn: &mut SqliteConnection, user_id: i32) -> usize {
        use crate::schema::users::dsl::*;
        diesel::delete(users.filter(id.eq(user_id)))
            .execute(conn)
            .expect("Error deleting user")
    }
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

impl<'a> NewUser<'a> {
    pub fn create(
        conn: &mut SqliteConnection,
        new_name: &'a str,
        new_email: &'a str,
        new_password_hash: &'a str,
    ) -> serde_json::Value {
        let new_user = NewUser {
            name: new_name,
            email: new_email,
            password_hash: new_password_hash,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)
            .expect("Error saving new user");

        json!({"message": "User created successfully"})
    }
}
