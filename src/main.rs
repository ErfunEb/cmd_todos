mod app;

use app::App;
use sqlite::Connection;

fn main() {
    let connection: Connection = sqlite::open("todos.db").unwrap();
    let app = App::new(connection);
    app.run();
}
