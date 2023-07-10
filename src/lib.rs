use log::Log;

pub struct MockLogger {

}

impl Log for MockLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn flush(&self) {
        
    }

    fn log(&self, _record: &log::Record) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
