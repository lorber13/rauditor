use crate::app::App;

mod app;

fn main() -> eframe::Result {
    eframe::run_native(
        "rauditor",
        Default::default(),
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
