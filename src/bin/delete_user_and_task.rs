use backend::establish_connection;
use backend::models::{Task, User};

fn main() {
    let mut conn = establish_connection();

    Task::delete_by_id(&mut conn, 1);
    User::delete_by_id(&mut conn, 1);

    let user_results = User::get_all(&mut conn);

    for user in user_results {
        println!("Deleting User ID: {}, Name: {}", user.id, user.name);
        User::delete_by_id(&mut conn, user.id);
    }

    let task_results = Task::get_all(&mut conn);

    for task in task_results {
        println!("Deleting Task ID: {}, Title: {}", task.id, task.title);
        Task::delete_by_id(&mut conn, task.id);
    }
}
