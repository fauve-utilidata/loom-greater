//! This module represents various ways of packaging various samples into buckets in an efficient way.
//! So we're going to have some buffer for accessing every channel
//! What data structure do we want?
//! it needs to be as big as MAX_WINDOW-SIZE * SAMPLES
//! We need to be able to access randomly access slices of the data.
//! We need to be a
//! And in other windows
//! So maybe a circular buffer is correct if it's indexable, slicable?
//! (spoiler, I can't find any that are slicable)
//! we just need to do janitoring around whether how we
//!
//! what if we keep the packets in a fixed length queue (can we slice it?)
//! with metadata on when the sample occured
//! and then copy it into whatever buffer comes in.

use std::{collections::VecDeque, sync::Arc};

use crossbeam_channel::{Receiver, Sender, bounded};
use parking_lot::Mutex;

use crate::{
    buffer_channels::{
        ADE9000_CURRENT_PHASEA_32KSPS_F32, ADE9000_CURRENT_PHASEB_32KSPS_F32,
        ADE9000_VOLTAGE_PRIMARY_32KSPS_F32,
    },
    config::ComputationConfig,
};

#[derive(Clone)]
pub struct Package {
    // TODO: parse this out into channels
    packets: Arc<Mutex<Option<Packets>>>,
    compute_config: ComputationConfig,

    full_send: Sender<Packets>,
    full_recv: Receiver<Packets>,
}

/// This is an incomplete list
#[derive(Default)]
pub struct Packets {
    pub v: VecDeque<f32>,
    pub a_a: VecDeque<f32>,
    pub b_a: VecDeque<f32>,
}

impl Packets {
    fn new(capacity: usize) -> Self {
        Self {
            v: VecDeque::with_capacity(capacity),
            a_a: VecDeque::with_capacity(capacity),
            b_a: VecDeque::with_capacity(capacity),
        }
    }
}

impl Package {
    // IDK if that is the correct size of things.
    pub fn new(compute_config: ComputationConfig) -> Self {
        let (sender, recevier) = bounded(1);
        Self {
            // mapping to window size isn't quite correct
            packets: Arc::new(Mutex::new(Some(Packets::new(
                compute_config.num_samples_per_compute,
            )))),
            full_send: sender,
            full_recv: recevier,
            compute_config,
        }
    }

    pub fn push(&self, data: Box<[u8]>, num_samples: usize) {
        let mut wrapped_packets = self.packets.lock();
        let mut packets = wrapped_packets.take().unwrap();
        for i in 1..=num_samples {
            // there is probably a zerocopy way of doing this.
            // but from_le_bytes takes a sized array.
            // we're only occupying 4+(sizeof(float)*3) bytes of stack.
            let mut tmp: [u8; 4] = [0; 4];
            tmp[0] = data[ADE9000_VOLTAGE_PRIMARY_32KSPS_F32 * i];
            tmp[1] = data[(ADE9000_VOLTAGE_PRIMARY_32KSPS_F32 * i) + 1];
            tmp[2] = data[(ADE9000_VOLTAGE_PRIMARY_32KSPS_F32 * i) + 2];
            tmp[3] = data[(ADE9000_VOLTAGE_PRIMARY_32KSPS_F32 * i) + 3];
            let voltage = f32::from_le_bytes(tmp);
            tmp[0] = data[ADE9000_CURRENT_PHASEA_32KSPS_F32 * i];
            tmp[1] = data[(ADE9000_CURRENT_PHASEA_32KSPS_F32 * i) + 1];
            tmp[2] = data[(ADE9000_CURRENT_PHASEA_32KSPS_F32 * i) + 2];
            tmp[3] = data[(ADE9000_CURRENT_PHASEA_32KSPS_F32 * i) + 3];
            let current_a = f32::from_le_bytes(tmp);
            tmp[0] = data[ADE9000_CURRENT_PHASEB_32KSPS_F32 * i];
            tmp[1] = data[(ADE9000_CURRENT_PHASEB_32KSPS_F32 * i) + 1];
            tmp[2] = data[(ADE9000_CURRENT_PHASEB_32KSPS_F32 * i) + 2];
            tmp[3] = data[(ADE9000_CURRENT_PHASEB_32KSPS_F32 * i) + 3];
            let current_b = f32::from_le_bytes(tmp);
            packets.v.push_back(voltage);
            packets.a_a.push_back(current_a);
            packets.b_a.push_back(current_b);
        }
        // all packet sizes are the same.
        if packets.a_a.len() >= self.compute_config.num_samples_per_compute {
            let _ = self.full_send.send(packets);
            packets = Packets::default();
        }
        *wrapped_packets = Some(packets);
    }

    // TODO: This will have to be adjusted when we introduce the different channels
    pub fn get_past_samples_voltage(&self, num_samples: usize) -> Vec<f32> {
        // this copies, probably try not to do that,
        // but that data structure will require some thinking and custom impl.
        self.packets
            .lock()
            .as_ref()
            .unwrap()
            .v
            .iter()
            .rev()
            .take(num_samples)
            .cloned()
            .collect()
    }

    pub fn get_past_samples_current_a(&self, num_samples: usize) -> Vec<f32> {
        // this copies, probably try not to do that,
        // but that data structure will require some thinking and custom impl.
        let packets = self.packets.lock();
        packets
            .as_ref()
            .unwrap()
            .a_a
            .iter()
            .rev()
            .take(num_samples)
            .cloned()
            .collect()
    }

    pub fn get_past_samples_current_b(&self, num_samples: usize) -> Vec<f32> {
        // this copies, probably try not to do that,
        // but that data structure will require some thinking and custom impl.
        let packets = self.packets.lock();
        packets
            .as_ref()
            .unwrap()
            .b_a
            .iter()
            .rev()
            .take(num_samples)
            .cloned()
            .collect()
    }

    pub fn compute_trigger(&self) -> Packets {
        self.full_recv.recv().unwrap()
    }
}
