mod app;

use app::App;

fn main() {
    let app = App::new();
    println!("Score: {}", app.score);
}
