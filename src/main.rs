use cxx_qt_lib::{QQmlApplicationEngine, QQuickStyle, QUrl};
use cxx_qt_lib_extras::QApplication;
use std::process::ExitCode;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

fn main() -> ExitCode {
    let mut app = QApplication::new();
    app.pin_mut().set_application_name(&"timerec".into());
    app.pin_mut()
        .set_application_display_name(&"TimeRec".into());
    app.pin_mut()
        .set_application_version(&env!("CARGO_PKG_VERSION").into());

    if std::env::var_os("QT_QUICK_CONTROLS_STYLE").is_none() {
        // Keep KDE integration on Linux; other platforms can use Qt's bundled
        // Fusion style without installing qqc2-desktop-style.
        let style = if cfg!(target_os = "linux") {
            "org.kde.desktop"
        } else {
            "Fusion"
        };
        QQuickStyle::set_style(&style.into());
    }

    let mut engine = QQmlApplicationEngine::new();
    let loaded = Arc::new(AtomicBool::new(false));
    let loaded_signal = Arc::clone(&loaded);
    let _connection = engine.pin_mut().on_object_created(move |_, object, _| {
        loaded_signal.store(!object.is_null(), Ordering::Relaxed);
    });

    // Embedded local QML loads synchronously, so a failed load must not enter
    // an event loop with no window.
    engine
        .pin_mut()
        .load(&QUrl::from("qrc:/qt/qml/local/timerec/src/qml/Main.qml"));
    if !loaded.load(Ordering::Relaxed) {
        eprintln!("TimeRec: failed to load the main window. See the Qt messages above.");
        return ExitCode::FAILURE;
    }

    if app.pin_mut().exec() == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
