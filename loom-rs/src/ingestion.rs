// alright, I'm not bloody sure how metrorec gets polled.
// I'm going to cross that bridge later.
// nah, fuck it we ball

use std::{
    io::Read,
    process::{Command, Stdio},
};

use crate::{buffer_channels::*, config::IngestionConfig, package::Package};

pub struct Ingestion {
    config: IngestionConfig,
    package: Package,
}

impl Ingestion {
    pub fn new(config: IngestionConfig, package: Package) -> Self {
        Self { config, package }
    }

    pub fn start(&self) {
        let mut cmd = Command::new(self.config.binary_path.clone());
        // buffer path
        /*
        cmd.arg(format!("{}", 32000))
            .arg(format!("{}", 1024))
            .stdout(Stdio::piped());
        */

        cmd.arg(self.config.data_path.clone())
            // mode
            .arg(format!("{}", 0))
            // I want every channel.
            .arg(format!("{}", ADE9000_GENERIC_SEQNR_32KSPS_I64))
            .arg(format!("{}", ADE9000_GENERIC_MONOTIME_32KSPS_I64))
            .arg(format!("{}", ADE9000_GENERIC_UTCTIME_32KSPS_I64))
            .arg(format!("{}", ADE9000_GENERIC_UTCTIME_BACKCALC_32KSPS_I64))
            .arg(format!("{}", ADE9000_VOLTAGE_PRIMARY_32KSPS_F32))
            .arg(format!("{}", ADE9000_VOLTAGE_SECONDARY_32KSPS_F32))
            .arg(format!("{}", ADE9000_CURRENT_PHASEA_32KSPS_F32))
            .arg(format!("{}", ADE9000_CURRENT_PHASEB_32KSPS_F32))
            .arg(format!("{}", ADE9000_METROREC_RECV_MONOTIME_32KSPS_I64))
            .arg(format!("{}", ADE9000_RAW_MONOTIME_32KSPS_I64))
            .arg(format!("{}", ADE9000_BUFFER_SEQNR_32KSPS_I32))
            .arg(format!("{}", ADE9000_UDP_SEQNR_32KSPS_I32))
            .arg(format!("{}", ADE9000_CUR_WFB_INDEX_32KSPS_I32))
            .arg(format!("{}", ADE9000_LOC_WFB_INDEX_32KSPS_I32))
            .arg(format!("{}", ADE9000_DISCOVERY_TIME_32KSPS_I32))
            .arg(format!("{}", ADE9000_DISCOVERY_TSHI_32KSPS_I32))
            .arg(format!("{}", ADE9000_BUFFER_MONOTIME_32KSPS_I32))
            .arg(format!("{}", ADE9000_SAMPLE_MONOTIME_32KSPS_I32))
            .arg(format!("{}", ADE9000_SAMPLE_MONOTSHI_32KSPS_I32))
            .arg(format!("{}", ADE9000_VOLTRAW_PRIMARY_32KSPS_I32))
            .arg(format!("{}", ADE9000_VOLTRAW_SECONDARY_32KSPS_I32))
            .arg(format!("{}", ADE9000_VOLTRAW_TERTIARY_32KSPS_I32))
            .arg(format!("{}", ADE9000_CURRAW_PHASEA_32KSPS_I32))
            .arg(format!("{}", ADE9000_CURRAW_PHASEB_32KSPS_I32))
            .arg(format!("{}", ADE9000_CURRAW_PHASEC_32KSPS_I32))
            .arg(format!("{}", ADE9000_CURRAW_PHASEN_32KSPS_I32))
            .arg(format!("{}", ADE9000_VOLTAGE_PRIMARY_CLEAN_32KSPS_F32))
            .arg(format!("{}", ADE9000_VOLTAGE_SECONDARY_CLEAN_32KSPS_F32))
            .stdout(Stdio::piped());

        let num_i64_channels = 6;
        let num_i32_channels = 16;
        let num_f32_channels = 6;

        // the size of the channel should be constant between reads.

        let buffer_size = (size_of::<i64>() * num_i64_channels
            + size_of::<f32>() * num_f32_channels
            + size_of::<i32>() * num_i32_channels)
            * self.config.num_samples_per_read;

        let mut producer_process = cmd
            .spawn()
            .expect(&format!("Could not start {}", self.config.binary_path));

        let mut producer_stdout = producer_process
            .stdout
            .take()
            .expect("Could not get stdout");
        let mut samples = 0;
        loop {
            // how to solve a problem like maria?
            // we need to read a slice, and transport it across thread lines
            // additionally, the read size is dynamic, depending on the number of samples read.
            // maybe? we could solve this by reading in constants from a compile time config
            // but we still have the problem of passing stack memory across threads.
            // so each "packet" has to go onto the heap so we can pass it and minimize copies.
            // TODO: maybe pool these buffers?
            // does a sendable memory pool exist?
            // duh, just pop them into a channel. conveyor belt sucka.

            let mut buf = vec![0; buffer_size].into_boxed_slice();
            if let Err(err) = producer_stdout.read_exact(&mut buf) {
                log::error!(
                    "Could not read producer process: {err:#?}. TODO: restart producer process"
                );
            }
            self.package.push(buf, self.config.num_samples_per_read);
            samples += self.config.num_samples_per_read;
            if samples > 32000 {
                println!("we did the thing");
                samples = 0;
            }
        }
    }
}
