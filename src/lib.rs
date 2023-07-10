use log::{Level, LevelFilter, Log};
use std::cell::{Ref, RefCell};
use std::fmt;
use std::sync::Once;

/// Captured log entry.
pub struct LogEntry {
    pub level: Level,
    pub body: String,
}

impl<'a> From<&'a log::Record<'a>> for LogEntry {
    fn from(record: &'a log::Record) -> Self {
        Self {
            level: record.level(),
            body: fmt::format(*record.args()),
        }
    }
}

thread_local! {
    static LOG_ENTRIES: RefCell<Vec<LogEntry>> = RefCell::new(Vec::<LogEntry>::new());
}

/// Mock logging implementation
pub struct MockLogger;

impl MockLogger {
    /// Access all captured log entries.
    pub fn entries<F>(f: F)
    where
        F: FnOnce(Ref<'_, Vec<LogEntry>>),
    {
        LOG_ENTRIES.with(|entries| {
            f(entries.borrow());
        });
    }

    /// Map over all captured log entries.
    pub fn map<F>(f: F)
    where
        F: Fn(usize, &LogEntry),
    {
        LOG_ENTRIES.with(|entries| {
            for (i, entry) in entries.borrow().iter().enumerate() {
                f(i, entry);
            }
        });
    }

    /// Clear captured log entries.
    pub fn empty() {
        LOG_ENTRIES.with(|entries| {
            entries.borrow_mut().truncate(0);
        });
    }
}

impl Log for MockLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn flush(&self) {}

    fn log(&self, record: &log::Record) {
        LOG_ENTRIES.with(|entries| {
            entries.borrow_mut().push(LogEntry::from(record));
        });
    }
}

static INITIALIZED: Once = Once::new();
static INSTANCE: MockLogger = MockLogger {};

/// Initialize the MockLogger.
/// If called after the logger has already been initialized this will clear any previously captured log entries.
pub fn init() {
    INITIALIZED.call_once(|| {
        log::set_logger(&INSTANCE).unwrap();
        log::set_max_level(LevelFilter::Debug);
    });
    LOG_ENTRIES.with(|entries| {
        entries.borrow_mut().truncate(0);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{debug, error, info, warn};

    #[test]
    fn test_entries() {
        init();
        debug!("one");
        info!("two");
        warn!("three");
        error!("four");
        MockLogger::entries(|entries| {
            assert_eq!(entries.len(), 4);
            assert_eq!(entries[0].level, Level::Debug);
            assert_eq!(entries[0].body, "one".to_owned());
            assert_eq!(entries[1].level, Level::Info);
            assert_eq!(entries[1].body, "two".to_owned());
            assert_eq!(entries[2].level, Level::Warn);
            assert_eq!(entries[2].body, "three".to_owned());
            assert_eq!(entries[3].level, Level::Error);
            assert_eq!(entries[3].body, "four".to_owned());
        });
    }

    #[test]
    fn test_map() {
        init();
        debug!("entry {}", 1);
        debug!("entry {}", 2);
        MockLogger::map(|i, entry| match i {
            0 => {
                assert_eq!(entry.level, Level::Debug);
                assert_eq!(entry.body, "entry 1".to_owned());
            }
            1 => {
                assert_eq!(entry.level, Level::Debug);
                assert_eq!(entry.body, "entry 2".to_owned());
            }
            _ => {
                panic!("MockLogger has too many entries!");
            }
        });
    }
}
