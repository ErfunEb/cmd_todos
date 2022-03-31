use sqlite::Connection;
use std::io;

struct App {
    connection: Connection,
}

impl App {
    fn new(connection: Connection) -> Self {
        App { connection }
    }

    fn menu(&self) {
        println!();
        println!("Choose an option: ");
        println!("1. List of todos");
        println!("2. Add todo");
        println!("3. Mark todo as done");
        println!("4. Delete todo");
        println!();
    }

    fn input(&self) -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read a sentence");
        input.pop();
        input
    }

    fn list_todos(&self) {
        println!("List of todos: ");
        self.connection
            .iterate("SELECT * FROM todos", |pairs| {
                let mut iteration = pairs.iter();
                let (_, result) = iteration.next().unwrap();
                let id = result.unwrap();
                let (_, result) = iteration.next().unwrap();
                let name = result.unwrap();
                let (_, result) = iteration.next().unwrap();
                let done = if result.unwrap() == "1" {
                    "true"
                } else {
                    "false"
                };
                println!("id: {} - name: {} - done: {}", id, name, done);
                true
            })
            .unwrap();
    }

    fn add_todo(&self) {
        println!("Enter new todo name: ");
        let name = self.input();
        self.connection
            .execute(format!(
                "INSERT INTO todos(name, done) VALUES ('{}', {})",
                name, false
            ))
            .expect("Failed to add todo to file");
    }

    fn mark_as_done(&self) {
        println!("Please enter todo id that you want to mark as done: ");
        let id_string = self.input();
        let id: i32 = id_string.parse().unwrap();
        self.connection
            .execute(format!("UPDATE todos SET done={} WHERE id={}", true, id))
            .expect("Failed to mark todo as done!");
    }

    fn delete_todo(&self) {
        println!("Please enter todo id that you want to delete: ");
        let id_string = self.input();
        let id: i32 = id_string.parse().unwrap();
        self.connection
            .execute(format!("DELETE FROM todos WHERE id={}", id))
            .expect("Failed to delete todo!");
    }
}

fn main() {
    let connection: Connection = sqlite::open("todos.db").unwrap();
    connection
        .execute(
            "create table if not exists todos (
             id integer primary key,
             name text not null unique,
             done boolean default(false)
         )",
        )
        .expect("Error creating table");
    let app = App::new(connection);

    loop {
        app.menu();
        let input = app.input();
        match input.as_str() {
            "1" => {
                app.list_todos();
            }
            "2" => {
                app.add_todo();
            }
            "3" => {
                app.mark_as_done();
            }
            "4" => {
                app.delete_todo();
            }
            _ => {
                println!("Wrong command!")
            }
        }
    }
}
