use std::path::Path;
use std::fs::OpenOptions;
use std::sync::Arc;
use tracing_appender::non_blocking::WorkerGuard;

#[derive(Debug, Clone)]
pub struct Logger {
    _guard: Arc<WorkerGuard>,
}

impl Logger {
    pub fn new(filename: &Path) -> Self {
        let f = OpenOptions::new()
            //.write(true)
            .append(true)
            .open(filename)
            .expect("Can open logfile");
        // :TODO: handle new file creation

        let (non_blocking, guard) = tracing_appender::non_blocking(f);
        let subscriber = tracing_subscriber::fmt().with_writer(non_blocking);
        subscriber.init();
        let _guard = Arc::new(guard);
        Self { _guard }
    }
}
