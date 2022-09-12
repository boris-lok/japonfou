#[derive(Debug, Default)]
pub struct IdGeneratorConfig {
    pub worker_id: u8,
    pub data_center_id: u8,
    pub timestamp_offset: u128,
}

impl IdGeneratorConfig {
    pub fn new(worker_id: u8, data_center_id: u8, timestamp_offset: u128) -> Self {
        Self {
            worker_id,
            data_center_id,
            timestamp_offset,
        }
    }
}
