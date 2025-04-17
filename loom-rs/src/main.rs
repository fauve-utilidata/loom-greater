use std::time::Duration;

use config::{dummy_ingestion_config, jetson_config};
use ingestion::Ingestion;
use package::Package;
use processing::Processing;

mod buffer_channels;
mod config;
mod cuda;
mod ingestion;
mod package;
mod processing;

pub struct DataProducerPipeline {}

fn main() {
    env_logger::init();

    // let ingestion_config = dummy_ingestion_config();
    let config = jetson_config();

    let package = Package::new(config.computation_config.clone());

    let ingestion = Ingestion::new(config.ingestion_config.clone(), package.clone());

    let ingestion_thread = std::thread::spawn(move || {
        ingestion.start();
    });

    let processor = Processing::new(package, config.computation_config);
    let data_thread = std::thread::spawn(move || {
        processor.compute();
    });

    let _ = ingestion_thread.join();
    let _ = data_thread.join();
}
