pub const APPLICATION_ID: &str = "io.kerynth.krl";

pub fn application_name() -> &'static str {
    "Kerynth Rust Launcher"
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: android_activity::AndroidApp) {
    use android_activity::{MainEvent, PollEvent};
    use std::time::Duration;

    loop {
        let mut destroyed = false;

        let _ = app.poll_events(Some(Duration::from_millis(500)), |event| {
            if matches!(event, PollEvent::Main(MainEvent::Destroy)) {
                destroyed = true;
            }
        });

        if destroyed {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{APPLICATION_ID, application_name};

    #[test]
    fn exposes_android_application_identity() {
        assert_eq!(APPLICATION_ID, "io.kerynth.krl");
        assert_eq!(application_name(), "Kerynth Rust Launcher");
    }
}
