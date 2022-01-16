use std::time::Instant;

#[derive(Default)]
pub struct TimeDeltaLogger {
    prev: std::sync::Mutex<Option<Instant>>,
}

impl log::Log for TimeDeltaLogger {
    fn enabled(&self, _: &log::Metadata) -> bool { true }

    fn log(&self, record: &log::Record) {
        let mut g = self.prev.try_lock().unwrap();
        let t = std::time::Instant::now();
        let d = if let Some(prev_t) = *g {
            t - prev_t
        } else {
            std::time::Duration::new(0, 0)
        };
        *g = Some(t);

        eprintln!("+{:.4}  {}", d.as_secs_f64(), record.args());
    }

    fn flush(&self) {}
}
