use serde::Deserialize;
#[derive(Clone, Debug, Deserialize)]
pub struct IngestionConfig {
    pub binary_path: String,
    pub data_path: String,
    pub num_samples_per_read: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ComputationConfig {
    pub num_samples_per_compute: usize,
    pub use_cuda: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub computation_config: ComputationConfig,
    pub ingestion_config: IngestionConfig,
}

pub fn jetson_config() -> Config {
    Config {
        ingestion_config: IngestionConfig {
            binary_path: "/opt/metrorec_and_friends/read_metrobuf".to_string(),
            data_path: "/opt/metrorec_buffer".to_string(),
            num_samples_per_read: 1024,
        },
        computation_config: ComputationConfig {
            num_samples_per_compute: 32000,
            use_cuda: false,
        },
    }
}

pub fn dummy_ingestion_config() -> Config {
    Config {
        ingestion_config: IngestionConfig {
            binary_path: "/home/fauve/janitoring/my_bins//metrofake".to_string(),
            data_path: "dummy/file".to_string(),
            num_samples_per_read: 1024,
        },
        computation_config: ComputationConfig {
            num_samples_per_compute: 32000,
            use_cuda: false,
        },
    }
}
