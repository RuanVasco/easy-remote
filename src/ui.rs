use gtk4::glib::BoxedAnyObject;
use gtk4::{ListItem, ListView, PolicyType, SignalListItemFactory, SingleSelection, gio, prelude::*};
use gtk4::{Application, ApplicationWindow, Button, ScrolledWindow};

use crate::config::Entries;
use crate::model::VncConnection;

pub fn build(app: &Application) {
    let store = gio::ListStore::new::<BoxedAnyObject>();
    let entries = Entries::load();

    for conexao in entries {
        let obj = BoxedAnyObject::new(conexao);
        store.append(&obj);
    }

    let selection_model = SingleSelection::new(Some(store));

    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_factory, item| {
        let item = item.downcast_ref::<ListItem>().unwrap();

        let button = Button::builder()
            .margin_top(5)
            .margin_bottom(5)
            .margin_start(10)
            .margin_end(10)
            .build();

        item.set_child(Some(&button));
    });

    factory.connect_bind(move |_factory, item| {
        let item = item.downcast_ref::<ListItem>().unwrap();

        let button = item.child().and_downcast::<Button>().unwrap();

        let entry = item.item().and_downcast::<BoxedAnyObject>().unwrap();

        let vnc_conn = entry.borrow::<VncConnection>().clone();

        button.set_label(&format!("{}", vnc_conn.label));

        button.connect_clicked(move |_| {
            vnc_conn.connect();
        });
    });

    let list_view = ListView::new(Some(selection_model), Some(factory));

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .min_content_height(400)
        .child(&list_view)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Lista de Técnicos (GtkListView)")
        .default_width(350)
        .default_height(500)
        .child(&scrolled_window)
        .build();

    window.present();
}
