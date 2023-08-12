use google_cloud_logging::{GCLogSeverity, GCOperation, GCSourceLocation, GoogleCloudStructLog};
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError};

const GCP_ERROR_REPORT_TYPE: &str =
    "type.googleapis.com/google.devtools.clouderrorreporting.v1beta1.ReportedErrorEvent";

struct CloudLogger;

static LOGGER: CloudLogger = CloudLogger;

impl Log for CloudLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn flush(&self) {}

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level = record.level();

            let log_entry = GoogleCloudStructLog {
                severity: Some(match level {
                    Level::Error => GCLogSeverity::Error,
                    Level::Warn => GCLogSeverity::Warning,
                    Level::Info => GCLogSeverity::Info,
                    Level::Debug => GCLogSeverity::Debug,
                    Level::Trace => GCLogSeverity::Default,
                }),
                report_type: match level {
                    Level::Error => Some(GCP_ERROR_REPORT_TYPE.to_owned()),
                    _ => None,
                },
                message: Some(format!("{}", record.args())),
                operation: Some(GCOperation {
                    id: Some("bionic-reading-discord-bot"),
                    producer: Some("bionic-reading-discord-bot"),
                    ..Default::default()
                }),
                source_location: Some(GCSourceLocation {
                    file: record.file_static(),
                    line: record.line().map(|s| s.to_string()),
                    function: record.module_path_static(),
                }),
                ..Default::default()
            };

            println!(
                "{}",
                serde_json::to_string(&log_entry).expect("Error during logging")
            );
        }
    }
}

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
