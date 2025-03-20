use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InputFile {
    pub file_name: String,
    pub file_size: u64,
    pub file_path: String,
    pub file_directory: String,
    pub file_updated_at_timestamp: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProcessProperties {
    pub tool_name: String,
    pub tool_pid: String,
    pub tool_parent_pid: String,
    pub tool_binary_path: String,
    pub tool_cmd: String,
    pub start_timestamp: String,
    pub process_cpu_utilization: f32,
    pub process_memory_usage: u64,
    pub process_memory_virtual: u64,
    pub process_run_time: u64,
    pub process_disk_usage_read_last_interval: u64,
    pub process_disk_usage_write_last_interval: u64,
    pub process_disk_usage_read_total: u64,
    pub process_disk_usage_write_total: u64,
    pub process_status: String,
    pub input_files: Option<Vec<InputFile>>,
    pub container_id: Option<String>,
    pub aws_batch_job_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedProcess {
    pub tool_name: String,
    pub tool_pid: String,
    pub duration_sec: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSetsProcessed {
    pub datasets: String,
    pub total: u64,
}
