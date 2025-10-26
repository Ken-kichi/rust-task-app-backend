use backend::models::task::{NewTask, Task};
use backend::models::user::{NewUser, User};

fn main() {
    use backend::establish_connection;

    let mut conn = establish_connection();

    NewUser::create(
        &mut conn,
        "John Doe",
        "john+task45@example.com",
        "password12345",
    );

    let user = User::get_by_first_id(&mut conn).unwrap();

    println!("Created user: {:?}", user.id);

    NewTask::create(
        &mut conn,
        user.id,
        "Sample Task",
        Some("This is a sample task description.".to_string()),
        "pending".to_string(),
        Some(false),
    );

    let task = Task::get_by_first_id(&mut conn).unwrap();

    println!("Created task for user id {}: {}", task.user_id, task.title);
}
