use std::time::SystemTime;

use rustfft::{FftPlanner, num_complex::Complex32};

use crate::{config::ComputationConfig, package::Package};

pub struct Processing {
    package: Package,
    computation_config: ComputationConfig,
}

impl Processing {
    pub fn new(package: Package, computation_config: ComputationConfig) -> Self {
        Self {
            package,
            computation_config,
        }
    }

    pub fn compute(&self) {
        loop {
            let prestart = SystemTime::now();
            let mut packets = self.package.compute_trigger();
            let start = SystemTime::now();
            let len = packets.a_a.len();

            if self.computation_config.use_cuda {
                let _ = crate::cuda::run(packets.a_a.make_contiguous());
                let _ = crate::cuda::run(packets.b_a.make_contiguous());
                let _ = crate::cuda::run(packets.v.make_contiguous());
            } else {
                // TODO: configuration
                let mut current_a: Vec<Complex32> = packets
                    .a_a
                    .into_iter()
                    // more copies!
                    .map(|v| Complex32::new(v, 0.0))
                    .collect();
                let mut current_b: Vec<Complex32> = packets
                    .b_a
                    .into_iter()
                    .map(|v| Complex32::new(v, 0.0))
                    .collect();
                let mut voltage: Vec<Complex32> = packets
                    .v
                    .into_iter()
                    .map(|v| Complex32::new(v, 0.0))
                    .collect();

                let mut planner = FftPlanner::new();
                let fft = planner.plan_fft_forward(len);
                fft.process(current_a.as_mut_slice());
                fft.process(current_b.as_mut_slice());
                fft.process(voltage.as_mut_slice());
            }

            let end = SystemTime::now();
            println!(
                "Waited {:?} Compute took {:?} {} samples",
                end.duration_since(prestart).unwrap(),
                end.duration_since(start).unwrap(),
                len
            );
        }
    }
}
