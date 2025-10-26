use crate::schema::tasks;
use crate::schema::tasks::dsl::*;
use diesel::prelude::*;
use serde_json::json;
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tasks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub completed: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Task {
    pub fn get_all(conn: &mut SqliteConnection) -> Vec<Task> {
        tasks.load::<Task>(conn).expect("Error loading tasks")
    }

    pub fn get_by_id(conn: &mut SqliteConnection, task_id: i32) -> Option<Task> {
        tasks
            .filter(id.eq(task_id))
            .first::<Task>(conn)
            .optional()
            .expect("Error loading task by id")
    }

    pub fn get_by_first_id(conn: &mut SqliteConnection) -> Option<Task> {
        tasks::table
            .order(tasks::id.desc())
            .select(Task::as_select())
            .first::<Task>(conn)
            .optional()
            .expect("Error loading newly created task")
    }

    pub fn update_by_id(
        conn: &mut SqliteConnection,
        task_id: i32,
        new_title: &str,
        new_description: &str,
        new_status: &str,
        new_completed: bool,
    ) -> usize {
        use crate::schema::tasks::dsl::*;
        diesel::update(tasks.filter(id.eq(task_id)))
            .set((
                title.eq(new_title),
                description.eq(new_description),
                status.eq(new_status),
                completed.eq(new_completed),
            ))
            .execute(conn)
            .expect("Error updating task")
    }

    pub fn delete_by_id(conn: &mut SqliteConnection, task_id: i32) -> usize {
        use crate::schema::tasks::dsl::*;
        diesel::delete(tasks.filter(id.eq(task_id)))
            .execute(conn)
            .expect("Error deleting task")
    }
}

#[derive(Insertable)]
#[diesel(table_name=tasks)]
pub struct NewTask<'a> {
    pub user_id: i32,
    pub title: &'a str,
    pub description: Option<String>,
    pub status: String,
    pub completed: Option<bool>,
}

impl NewTask<'_> {
    pub fn create(
        conn: &mut SqliteConnection,
        new_user_id: i32,
        new_title: &str,
        new_description: Option<String>,
        new_status: String,
        new_completed: Option<bool>,
    ) -> serde_json::Value {
        let new_task = NewTask {
            user_id: new_user_id,
            title: new_title,
            description: new_description,
            status: new_status,
            completed: new_completed,
        };

        diesel::insert_into(tasks::table)
            .values(&new_task)
            .execute(conn)
            .expect("Error saving new task");

        json!({"message": "Task created successfully"})
    }
}
