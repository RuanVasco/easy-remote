use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Label};

fn main() {
    let app = Application::builder()
        .application_id("com.github.RuanVasco.easy-tech")
        .build();

    app.connect_activate(|app| {
        let label = Label::new(Some("Painel do Técnico\n(Em Construção)"));

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Easy Remote - Técnico")
            .default_width(600)
            .default_height(400)
            .child(&label)
            .build();

        window.present();
    });

    app.run();
}
