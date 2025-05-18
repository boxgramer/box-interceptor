use log::Log;

pub struct InfoLogger {
    pub sender: std::sync::mpsc::Sender<String>,
}

impl Log for InfoLogger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &log::Record) {
        let msg = format!("[{}], {}", record.level(), record.args());
        let _ = self.sender.send(msg);
    }

    fn flush(&self) {}
}
