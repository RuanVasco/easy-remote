use std::thread;

use crate::ui::UserInterface;

mod config;
mod service;
mod ui;

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    thread::spawn(|| {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            println!("Tokio Runtime iniciado");

            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
            }
        })
    });

    let app = UserInterface::new();
    app.run();
}
