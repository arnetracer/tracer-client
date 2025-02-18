use process::{CompletedProcess, DataSetsProcessed, ProcessProperties};
use syslog::SyslogProperties;
use system_metrics::{SystemMetric, SystemProperties};

pub mod process;
pub mod syslog;
pub mod system_metrics;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum EventAttributes {
    Process(ProcessProperties),
    CompletedProcess(CompletedProcess),
    SystemMetric(SystemMetric),
    Syslog(SyslogProperties),
    SystemProperties(SystemProperties),
    ProcessDatasetStats(DataSetsProcessed),
    // TODO: take out when done with demo
    Other(serde_json::Value),
}
