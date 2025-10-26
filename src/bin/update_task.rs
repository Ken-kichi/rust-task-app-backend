use backend::establish_connection;
use backend::models::task::Task;

fn main() {
    let mut conn = establish_connection();

    let resutls = Task::get_all(&mut conn);

    for task in resutls {
        println!("Task ID: {}, Title: {}", task.id, task.title);
    }

    Task::update_by_id(
        &mut conn,
        1,
        "Updated Task Title",
        "Updated description for the task.",
        "in_progress",
        true,
    );

    let updated_task = Task::get_by_id(&mut conn, 1).unwrap();

    println!(
        "Updated Task ID: {}, Title: {}, Status: {}, Completed: {}",
        updated_task.id,
        updated_task.title,
        updated_task.status,
        updated_task.completed.unwrap()
    );
}
