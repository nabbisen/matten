mod app;
mod cli;
mod output;
mod render;
mod report;
mod request;

fn main() {
    if let Err(err) = app::run() {
        eprintln!("matten-report error: {err}");
        std::process::exit(1);
    }
}
