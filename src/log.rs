use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::Arc;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::{Layer as _, filter};

struct LogFileWriter(File);

impl<'a> MakeWriter<'a> for LogFileWriter {
    type Writer = Box<dyn std::io::Write + 'a>;

    fn make_writer(&'a self) -> Self::Writer {
        Box::new(&self.0)
    }
}

impl TryFrom<&Path> for LogFileWriter {
    type Error = anyhow::Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self(file))
    }
}

/// Initialize the log configuration based on the CLI arguments
pub fn init_log(log_level: LevelFilter, log_filter: Option<String>, log_file: Option<&Path>) {
    let log_filter = Arc::new(log_filter);

    let stdout_logger = tracing_subscriber::fmt::layer()
        .compact()
        .with_ansi(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_line_number(true)
        .with_writer(std::io::stdout);

    let log_filter_t = log_filter.clone();
    let registry =
        tracing_subscriber::registry().with(stdout_logger.with_filter(log_level).with_filter(
            filter::filter_fn(move |metadata| {
                filter_log_matching_target(metadata, log_filter_t.clone())
            }),
        ));

    if let Some(log_file) = log_file {
        let file_logger = tracing_subscriber::fmt::layer()
            .compact()
            .with_ansi(false)
            .with_span_events(FmtSpan::CLOSE)
            .with_line_number(true)
            .with_writer(LogFileWriter::try_from(log_file).expect("failed to init log writer"));

        let registry = registry.with(file_logger.with_filter(log_level).with_filter(
            filter::filter_fn(move |metadata| {
                filter_log_matching_target(metadata, log_filter.clone())
            }),
        ));
        tracing::subscriber::set_global_default(registry).expect("failed to set global default");
    } else {
        tracing::subscriber::set_global_default(registry).expect("failed to set global default");
    }
}

/// Log filter to only log messages with target matching the given target
#[inline]
fn filter_log_matching_target(metadata: &tracing::Metadata, target: Arc<Option<String>>) -> bool {
    let Some(matching_target) = target.as_deref() else {
        return true;
    };

    let target = metadata.target();
    target.starts_with(matching_target)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_should_init_log() {
        init_log(LevelFilter::INFO, None, None);
        tracing::info!("This is an info log");
        tracing::debug!("This is a debug log");
    }
}
