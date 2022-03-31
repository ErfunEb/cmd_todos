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
        println!("1. list of todos");
        println!();
    }
    fn input(&self) -> String {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read a sentence");
        input
    }

    fn list_todos(&self) {
        self.connection
            .iterate("SELECT * FROM todos", |pairs| {
                let mut iteration = pairs.iter();
                let (_, result) = iteration.next().unwrap();
                let id = result.unwrap();
                let (_, result) = iteration.next().unwrap();
                let name = result.unwrap();
                let (_, result) = iteration.next().unwrap();
                let done = result.unwrap();
                println!("id: {} - name: {} - done: {}", id, name, done);
                true
            })
            .unwrap();
    }
}

fn main() {
    let connection: Connection = sqlite::open("test.db").unwrap();
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
        let mut input = app.input();
        input.pop();
        match input.as_str() {
            "1" => {
                app.list_todos();
            }
            _ => {
                println!("Wrong command!")
            }
        }
    }
}
