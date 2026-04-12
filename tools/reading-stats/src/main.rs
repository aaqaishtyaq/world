mod app;
mod classify;
mod markdown;
mod model;
mod repo;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}
