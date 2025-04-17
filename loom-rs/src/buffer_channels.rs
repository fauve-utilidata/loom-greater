// 0 2 3 6 11 12 14 15 4 5 90 80 81 82 83 84 92 93 94 21 22 23 25 26 27 28 41 42
pub const ADE9000_GENERIC_SEQNR_32KSPS_I64: usize = 0; // Generic Sequence Number of the sample (currently as received by microcontroller)
pub const ADE9000_GENERIC_MONOTIME_32KSPS_I64: usize = 2; // Generic Monotonic receive time (currently metrology microcontroller uptime), in us
pub const ADE9000_GENERIC_UTCTIME_32KSPS_I64: usize = 3; // Generic utc receive time (currently the minimum delay wall time)
pub const ADE9000_GENERIC_UTCTIME_BACKCALC_32KSPS_I64: usize = 6; // EXPERIMENTAL: Generic utc receive time with back calculation for samples before ntp sync
pub const ADE9000_VOLTAGE_PRIMARY_32KSPS_F32: usize = 11; // Primary   voltage (usually 240 V for split-phase), calibrated
pub const ADE9000_VOLTAGE_SECONDARY_32KSPS_F32: usize = 12; // Secondary voltage (usually 120 V for split-phase), calibrated
pub const ADE9000_CURRENT_PHASEA_32KSPS_F32: usize = 14; // Current on leg "A", calibrated - not phase-shift compensated to voltage!
pub const ADE9000_CURRENT_PHASEB_32KSPS_F32: usize = 15; // Current on leg "B", calibrated - not phase-shift compensated to voltage!

// Channels which are usually only used by developers as well as calibration & verification tools, they could change frequently:
pub const ADE9000_METROREC_RECV_MONOTIME_32KSPS_I64: usize = 4; // EXPERIMENTAL: Monotonic receive time at metrorec (usual jetson uptime) in us
pub const ADE9000_RAW_MONOTIME_32KSPS_I64: usize = 5; // EXPERIMENTAL: Discovery time monotime counter of metrolology microcontroller

pub const ADE9000_BUFFER_SEQNR_32KSPS_I32: usize = 90; // Sequence number of the sample, as received by metrorec

pub const ADE9000_UDP_SEQNR_32KSPS_I32: usize = 80; // 
pub const ADE9000_CUR_WFB_INDEX_32KSPS_I32: usize = 81; // 
pub const ADE9000_LOC_WFB_INDEX_32KSPS_I32: usize = 82; // 
pub const ADE9000_DISCOVERY_TIME_32KSPS_I32: usize = 83; // lower 32 bit of the 64-bit discovery time counter
pub const ADE9000_DISCOVERY_TSHI_32KSPS_I32: usize = 84; // upper 32 bit of the 64-bit discovery time counter

pub const ADE9000_BUFFER_MONOTIME_32KSPS_I32: usize = 92; // Monotonic receive time at metrorec (usually jetson uptime), IN NANOSECONDS (int32 wraps about every second)
pub const ADE9000_SAMPLE_MONOTIME_32KSPS_I32: usize = 93; // Monotonic receive time at frontend (usually metrology microcontroller uptime), IN NANOSECONDS 
pub const ADE9000_SAMPLE_MONOTSHI_32KSPS_I32: usize = 94; // Monotonic receive time at frontend (usually metrology microcontroller uptime), IN NANOSECONDS (32bit high)
pub const ADE9000_VOLTRAW_PRIMARY_32KSPS_I32: usize = 21; // raw 24-bit data for ade9000_voltage_primary_32ksps
pub const ADE9000_VOLTRAW_SECONDARY_32KSPS_I32: usize = 22; // raw 24-bit data for ade9000_voltage_secondary_32ksps
pub const ADE9000_VOLTRAW_TERTIARY_32KSPS_I32: usize = 23; // raw 24-bit data for ade9000 voltage channel C
pub const ADE9000_CURRAW_PHASEA_32KSPS_I32: usize = 25; // raw 24-bit data for ade9000_current_phaseA_32ksps
pub const ADE9000_CURRAW_PHASEB_32KSPS_I32: usize = 26; // raw 24-bit data for ade9000_current_phaseB_32ksps
pub const ADE9000_CURRAW_PHASEC_32KSPS_I32: usize = 27; // raw 24-bit data for ade9000 current channel C
pub const ADE9000_CURRAW_PHASEN_32KSPS_I32: usize = 28; // raw 24-bit data for ade9000 current channel N

pub const ADE9000_VOLTAGE_PRIMARY_CLEAN_32KSPS_F32: usize = 41; // EXPERIMENTAL: Primary voltage, calibrated, replaced with NaN for discontinuity
pub const ADE9000_VOLTAGE_SECONDARY_CLEAN_32KSPS_F32: usize = 42; // EXPERIMENTAL: Secondary voltage, calibrated, replaced with NaN for discontinuity
