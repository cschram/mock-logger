# Mock Logger for Rust

This is a testing utility providing a mock logging implementation that can be used to verify correctness in your own logging.

## Usage

```rust
use log::info;

fn log_something() {
    info!("something");
}
#[cfg(test)]
mod test {
    use mock_logger::MockLogger;
    use super::*;
    
    #[test]
    fn test_logging() {
        mock_logger::init();
        log_something();
        MockLogger.entries(|entries| {
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].level, log::Level::Info);
            assert_eq!(entries[0].body, "something");
        });
    }
}
```

