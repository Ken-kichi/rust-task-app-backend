use backend::establish_connection;
use backend::models::{Task, User};

fn main() {
    let connection = &mut establish_connection();

    let task_results = Task::get_all(connection);

    println!("Displaying {} tasks", task_results.len());
    for task in task_results {
        println!("{}", task.title);
        println!("----------\n");
    }

    let user_results = User::get_all(connection);

    println!("Displaying {} users", user_results.len());
    for user in user_results {
        println!(
            "Name: {}, Email: {},Password: {}",
            user.name, user.email, user.password_hash
        );
        println!("----------\n");
    }
}
