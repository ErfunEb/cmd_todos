use sqlite::Connection;
use std::io;

pub struct App {
  connection: Connection,
}

impl App {
  pub fn new(connection: Connection) -> Self {
    App { connection }
  }

  fn menu(&self) {
    println!();
    println!("Choose an option: ");
    println!("1. List of todos");
    println!("2. Add todo");
    println!("3. Mark todo as done");
    println!("4. Delete todo");
    println!("5. Exit");
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
    self
      .connection
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
    let statement = format!(
      "INSERT INTO todos(name, done) VALUES ('{}', {})",
      name, false
    );
    self
      .connection
      .execute(statement)
      .expect("Failed to add todo to file");
  }

  fn mark_as_done(&self) {
    println!("Please enter todo id that you want to mark as done: ");
    let id_string = self.input();
    let id: i32 = id_string.parse().unwrap();
    let statement = format!("UPDATE todos SET done={} WHERE id={}", true, id);
    self
      .connection
      .execute(statement)
      .expect("Failed to mark todo as done!");
  }

  fn delete_todo(&self) {
    println!("Please enter todo id that you want to delete: ");
    let id_string = self.input();
    let id: i32 = id_string.parse().unwrap();
    let statement = format!("DELETE FROM todos WHERE id={}", id);
    self
      .connection
      .execute(statement)
      .expect("Failed to delete todo!");
  }

  fn migrate(&self) {
    self
      .connection
      .execute(
        "create table if not exists todos (
         id integer primary key,
         name text not null unique,
         done boolean default(false)
     )",
      )
      .expect("Error creating table");
  }

  pub fn run(&self) {
    self.migrate();
    loop {
      self.menu();
      let input = self.input();
      match input.as_str() {
        "1" => self.list_todos(),
        "2" => self.add_todo(),
        "3" => self.mark_as_done(),
        "4" => self.delete_todo(),
        "5" => break,
        _ => println!("Wrong command!"),
      }
    }
  }
}
