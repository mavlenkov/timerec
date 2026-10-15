use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(QmlModule::new("local.timerec").qml_file("src/qml/Main.qml"))
        // Qt QML needs explicit Network linkage on macOS.
        .qt_module("Network")
        .qrc_resources(["assets/timerec.svg"])
        .build();
}
