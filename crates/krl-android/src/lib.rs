pub const APPLICATION_ID: &str = "io.kerynth.krl";
pub const APPLICATION_NAME: &str = "Kerynth Rust Launcher";

pub fn application_name() -> &'static str {
    APPLICATION_NAME
}

#[cfg(target_os = "android")]
mod ui;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: android_activity::AndroidApp) {
    let options = eframe::NativeOptions {
        android_app: Some(app),
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    let result = eframe::run_native(
        APPLICATION_NAME,
        options,
        Box::new(|context| Ok(Box::new(ui::KrlApp::new(context)))),
    );

    if let Err(error) = result {
        panic!("KRL Android UI failed to start: {error}");
    }
}

#[cfg(test)]
mod tests {
    use super::{APPLICATION_ID, APPLICATION_NAME, application_name};

    #[test]
    fn exposes_android_application_identity() {
        assert_eq!(APPLICATION_ID, "io.kerynth.krl");
        assert_eq!(APPLICATION_NAME, "Kerynth Rust Launcher");
        assert_eq!(application_name(), APPLICATION_NAME);
    }
}
