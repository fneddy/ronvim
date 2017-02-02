extern crate ronvim;
#[macro_use]
extern crate qml;

use qml::{QmlEngine, QObjectMacro, register_qml_type};
use ronvim::{QFrontend,Frontend,QBackend};



fn main() {
    let mut engine = QmlEngine::new();
    Q_REGISTER_QML!(QBackend);

    let frontend = QFrontend::create();
    engine.add_import_path("./src/");
    engine.set_and_store_property("frontend", frontend.get_qobj());
    engine.load_data(include_str!("RonVim.qml"));

    engine.exec();
    engine.quit();
}
