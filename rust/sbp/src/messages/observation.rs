// Copyright (C) 2015-2018 Swift Navigation Inc.
// Contact: Swift Navigation <dev@swiftnav.com>
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

//****************************************************************************
// Automatically generated from yaml/swiftnav/sbp/observation.yaml
// with generate.py. Please do not hand edit!
//****************************************************************************/
//! Satellite observation messages from the device.

extern crate byteorder;
#[allow(unused_imports)]
use self::byteorder::{LittleEndian, ReadBytesExt};
use super::gnss::*;

/// Header for observation message.
///
/// Header of a GNSS observation message.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct ObservationHeader {
    /// GNSS time of this observation
    pub t: GPSTime,
    /// Total number of observations. First nibble is the size of the sequence
    /// (n), second nibble is the zero-indexed counter (ith packet of n)
    pub n_obs: u8,
}

impl ObservationHeader {
    pub fn parse(_buf: &mut &[u8]) -> Result<ObservationHeader, ::Error> {
        Ok(ObservationHeader {
            t: GPSTime::parse(_buf)?,
            n_obs: _buf.read_u8()?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<ObservationHeader>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(ObservationHeader::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(buf: &mut &[u8], n: usize) -> Result<Vec<ObservationHeader>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(ObservationHeader::parse(buf)?);
        }
        Ok(v)
    }
}

/// GNSS doppler measurement.
///
/// Doppler measurement in Hz represented as a 24-bit
/// fixed point number with Q16.8 layout, i.e. 16-bits of whole
/// doppler and 8-bits of fractional doppler. This doppler is defined
/// as positive for approaching satellites.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Doppler {
    /// Doppler whole Hz
    pub i: i16,
    /// Doppler fractional part
    pub f: u8,
}

impl Doppler {
    pub fn parse(_buf: &mut &[u8]) -> Result<Doppler, ::Error> {
        Ok(Doppler {
            i: _buf.read_i16::<LittleEndian>()?,
            f: _buf.read_u8()?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<Doppler>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(Doppler::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(buf: &mut &[u8], n: usize) -> Result<Vec<Doppler>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(Doppler::parse(buf)?);
        }
        Ok(v)
    }
}

/// GNSS observations for a particular satellite signal.
///
/// Pseudorange and carrier phase observation for a satellite being tracked.
/// The observations are interoperable with 3rd party receivers and conform with
/// typical RTCM 3.1 message GPS/GLO observations.
///
/// Carrier phase observations are not guaranteed to be aligned to the RINEX 3
/// or RTCM 3.3 MSM reference signal and no 1/4 cycle adjustments are currently
/// peformed.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct PackedObsContent {
    /// Pseudorange observation
    pub P: u32,
    /// Carrier phase observation with typical sign convention.
    pub L: CarrierPhase,
    /// Doppler observation with typical sign convention.
    pub D: Doppler,
    /// Carrier-to-Noise density.  Zero implies invalid cn0.
    pub cn0: u8,
    /// Lock timer. This value gives an indication of the time for which a
    /// signal has maintained continuous phase lock. Whenever a signal has lost
    /// and regained lock, this value is reset to zero. It is encoded according
    /// to DF402 from the RTCM 10403.2 Amendment 2 specification.  Valid values
    /// range from 0 to 15 and the most significant nibble is reserved for
    /// future use.
    pub lock: u8,
    /// Measurement status flags. A bit field of flags providing the status of
    /// this observation.  If this field is 0 it means only the Cn0 estimate for
    /// the signal is valid.
    pub flags: u8,
    /// GNSS signal identifier (16 bit)
    pub sid: GnssSignal,
}

impl PackedObsContent {
    pub fn parse(_buf: &mut &[u8]) -> Result<PackedObsContent, ::Error> {
        Ok(PackedObsContent {
            P: _buf.read_u32::<LittleEndian>()?,
            L: CarrierPhase::parse(_buf)?,
            D: Doppler::parse(_buf)?,
            cn0: _buf.read_u8()?,
            lock: _buf.read_u8()?,
            flags: _buf.read_u8()?,
            sid: GnssSignal::parse(_buf)?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<PackedObsContent>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(PackedObsContent::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(buf: &mut &[u8], n: usize) -> Result<Vec<PackedObsContent>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(PackedObsContent::parse(buf)?);
        }
        Ok(v)
    }
}

/// Network correction for a particular satellite signal.
///
/// Pseudorange and carrier phase network corrections for a satellite signal.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct PackedOsrContent {
    /// Pseudorange observation
    pub P: u32,
    /// Carrier phase observation with typical sign convention.
    pub L: CarrierPhase,
    /// Lock timer. This value gives an indication of the time for which a
    /// signal has maintained continuous phase lock. Whenever a signal has lost
    /// and regained lock, this value is reset to zero. It is encoded according
    /// to DF402 from the RTCM 10403.2 Amendment 2 specification.  Valid values
    /// range from 0 to 15 and the most significant nibble is reserved for
    /// future use.
    pub lock: u8,
    /// Correction flags.
    pub flags: u8,
    /// GNSS signal identifier (16 bit)
    pub sid: GnssSignal,
    /// Slant ionospheric correction standard deviation
    pub iono_std: u16,
    /// Slant tropospheric correction standard deviation
    pub tropo_std: u16,
    /// Orbit/clock/bias correction projected on range standard deviation
    pub range_std: u16,
}

impl PackedOsrContent {
    pub fn parse(_buf: &mut &[u8]) -> Result<PackedOsrContent, ::Error> {
        Ok(PackedOsrContent {
            P: _buf.read_u32::<LittleEndian>()?,
            L: CarrierPhase::parse(_buf)?,
            lock: _buf.read_u8()?,
            flags: _buf.read_u8()?,
            sid: GnssSignal::parse(_buf)?,
            iono_std: _buf.read_u16::<LittleEndian>()?,
            tropo_std: _buf.read_u16::<LittleEndian>()?,
            range_std: _buf.read_u16::<LittleEndian>()?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<PackedOsrContent>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(PackedOsrContent::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(buf: &mut &[u8], n: usize) -> Result<Vec<PackedOsrContent>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(PackedOsrContent::parse(buf)?);
        }
        Ok(v)
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct EphemerisCommonContent {
    /// GNSS signal identifier (16 bit)
    pub sid: GnssSignal,
    /// Time of Ephemerides
    pub toe: GPSTimeSec,
    /// User Range Accuracy
    pub ura: f32,
    /// Curve fit interval
    pub fit_interval: u32,
    /// Status of ephemeris, 1 = valid, 0 = invalid
    pub valid: u8,
    /// Satellite health status. GPS: ICD-GPS-200, chapter 20.3.3.3.1.4 SBAS: 0
    /// = valid, non-zero = invalid GLO: 0 = valid, non-zero = invalid
    pub health_bits: u8,
}

impl EphemerisCommonContent {
    pub fn parse(_buf: &mut &[u8]) -> Result<EphemerisCommonContent, ::Error> {
        Ok(EphemerisCommonContent {
            sid: GnssSignal::parse(_buf)?,
            toe: GPSTimeSec::parse(_buf)?,
            ura: _buf.read_f32::<LittleEndian>()?,
            fit_interval: _buf.read_u32::<LittleEndian>()?,
            valid: _buf.read_u8()?,
            health_bits: _buf.read_u8()?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<EphemerisCommonContent>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(EphemerisCommonContent::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(
        buf: &mut &[u8],
        n: usize,
    ) -> Result<Vec<EphemerisCommonContent>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(EphemerisCommonContent::parse(buf)?);
        }
        Ok(v)
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct EphemerisCommonContentDepB {
    /// GNSS signal identifier (16 bit)
    pub sid: GnssSignal,
    /// Time of Ephemerides
    pub toe: GPSTimeSec,
    /// User Range Accuracy
    pub ura: f64,
    /// Curve fit interval
    pub fit_interval: u32,
    /// Status of ephemeris, 1 = valid, 0 = invalid
    pub valid: u8,
    /// Satellite health status. GPS: ICD-GPS-200, chapter 20.3.3.3.1.4 Others:
    /// 0 = valid, non-zero = invalid
    pub health_bits: u8,
}

impl EphemerisCommonContentDepB {
    pub fn parse(_buf: &mut &[u8]) -> Result<EphemerisCommonContentDepB, ::Error> {
        Ok(EphemerisCommonContentDepB {
            sid: GnssSignal::parse(_buf)?,
            toe: GPSTimeSec::parse(_buf)?,
            ura: _buf.read_f64::<LittleEndian>()?,
            fit_interval: _buf.read_u32::<LittleEndian>()?,
            valid: _buf.read_u8()?,
            health_bits: _buf.read_u8()?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<EphemerisCommonContentDepB>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(EphemerisCommonContentDepB::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(
        buf: &mut &[u8],
        n: usize,
    ) -> Result<Vec<EphemerisCommonContentDepB>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(EphemerisCommonContentDepB::parse(buf)?);
        }
        Ok(v)
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct EphemerisCommonContentDepA {
    /// GNSS signal identifier
    pub sid: GnssSignalDep,
    /// Time of Ephemerides
    pub toe: GPSTimeDep,
    /// User Range Accuracy
    pub ura: f64,
    /// Curve fit interval
    pub fit_interval: u32,
    /// Status of ephemeris, 1 = valid, 0 = invalid
    pub valid: u8,
    /// Satellite health status. GPS: ICD-GPS-200, chapter 20.3.3.3.1.4 SBAS: 0
    /// = valid, non-zero = invalid GLO: 0 = valid, non-zero = invalid
    pub health_bits: u8,
}

impl EphemerisCommonContentDepA {
    pub fn parse(_buf: &mut &[u8]) -> Result<EphemerisCommonContentDepA, ::Error> {
        Ok(EphemerisCommonContentDepA {
            sid: GnssSignalDep::parse(_buf)?,
            toe: GPSTimeDep::parse(_buf)?,
            ura: _buf.read_f64::<LittleEndian>()?,
            fit_interval: _buf.read_u32::<LittleEndian>()?,
            valid: _buf.read_u8()?,
            health_bits: _buf.read_u8()?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<EphemerisCommonContentDepA>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(EphemerisCommonContentDepA::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(
        buf: &mut &[u8],
        n: usize,
    ) -> Result<Vec<EphemerisCommonContentDepA>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(EphemerisCommonContentDepA::parse(buf)?);
        }
        Ok(v)
    }
}

/// Header for observation message.
///
/// Header of a GPS observation message.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct ObservationHeaderDep {
    /// GPS time of this observation
    pub t: GPSTimeDep,
    /// Total number of observations. First nibble is the size of the sequence
    /// (n), second nibble is the zero-indexed counter (ith packet of n)
    pub n_obs: u8,
}

impl ObservationHeaderDep {
    pub fn parse(_buf: &mut &[u8]) -> Result<ObservationHeaderDep, ::Error> {
        Ok(ObservationHeaderDep {
            t: GPSTimeDep::parse(_buf)?,
            n_obs: _buf.read_u8()?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<ObservationHeaderDep>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(ObservationHeaderDep::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(
        buf: &mut &[u8],
        n: usize,
    ) -> Result<Vec<ObservationHeaderDep>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(ObservationHeaderDep::parse(buf)?);
        }
        Ok(v)
    }
}

/// GPS carrier phase measurement.
///
/// Carrier phase measurement in cycles represented as a 40-bit
/// fixed point number with Q32.8 layout, i.e. 32-bits of whole
/// cycles and 8-bits of fractional cycles. This has the opposite
/// sign convention than a typical GPS receiver and the phase has
/// the opposite sign as the pseudorange.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct CarrierPhaseDepA {
    /// Carrier phase whole cycles
    pub i: i32,
    /// Carrier phase fractional part
    pub f: u8,
}

impl CarrierPhaseDepA {
    pub fn parse(_buf: &mut &[u8]) -> Result<CarrierPhaseDepA, ::Error> {
        Ok(CarrierPhaseDepA {
            i: _buf.read_i32::<LittleEndian>()?,
            f: _buf.read_u8()?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<CarrierPhaseDepA>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(CarrierPhaseDepA::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(buf: &mut &[u8], n: usize) -> Result<Vec<CarrierPhaseDepA>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(CarrierPhaseDepA::parse(buf)?);
        }
        Ok(v)
    }
}

/// Deprecated
///
/// Deprecated.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct PackedObsContentDepA {
    /// Pseudorange observation
    pub P: u32,
    /// Carrier phase observation with opposite sign from typical convention
    pub L: CarrierPhaseDepA,
    /// Carrier-to-Noise density
    pub cn0: u8,
    /// Lock indicator. This value changes whenever a satellite signal has lost
    /// and regained lock, indicating that the carrier phase ambiguity may have
    /// changed.
    pub lock: u16,
    /// PRN-1 identifier of the satellite signal
    pub prn: u8,
}

impl PackedObsContentDepA {
    pub fn parse(_buf: &mut &[u8]) -> Result<PackedObsContentDepA, ::Error> {
        Ok(PackedObsContentDepA {
            P: _buf.read_u32::<LittleEndian>()?,
            L: CarrierPhaseDepA::parse(_buf)?,
            cn0: _buf.read_u8()?,
            lock: _buf.read_u16::<LittleEndian>()?,
            prn: _buf.read_u8()?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<PackedObsContentDepA>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(PackedObsContentDepA::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(
        buf: &mut &[u8],
        n: usize,
    ) -> Result<Vec<PackedObsContentDepA>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(PackedObsContentDepA::parse(buf)?);
        }
        Ok(v)
    }
}

/// GPS observations for a particular satellite signal.
///
/// Pseudorange and carrier phase observation for a satellite being
/// tracked.  Pseudoranges are referenced to a nominal pseudorange.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct PackedObsContentDepB {
    /// Pseudorange observation
    pub P: u32,
    /// Carrier phase observation with opposite sign from typical convention.
    pub L: CarrierPhaseDepA,
    /// Carrier-to-Noise density
    pub cn0: u8,
    /// Lock indicator. This value changes whenever a satellite signal has lost
    /// and regained lock, indicating that the carrier phase ambiguity may have
    /// changed.
    pub lock: u16,
    /// GNSS signal identifier
    pub sid: GnssSignalDep,
}

impl PackedObsContentDepB {
    pub fn parse(_buf: &mut &[u8]) -> Result<PackedObsContentDepB, ::Error> {
        Ok(PackedObsContentDepB {
            P: _buf.read_u32::<LittleEndian>()?,
            L: CarrierPhaseDepA::parse(_buf)?,
            cn0: _buf.read_u8()?,
            lock: _buf.read_u16::<LittleEndian>()?,
            sid: GnssSignalDep::parse(_buf)?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<PackedObsContentDepB>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(PackedObsContentDepB::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(
        buf: &mut &[u8],
        n: usize,
    ) -> Result<Vec<PackedObsContentDepB>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(PackedObsContentDepB::parse(buf)?);
        }
        Ok(v)
    }
}

/// GPS observations for a particular satellite signal.
///
/// Pseudorange and carrier phase observation for a satellite being
/// tracked. The observations are be interoperable with 3rd party
/// receivers and conform with typical RTCMv3 GNSS observations.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct PackedObsContentDepC {
    /// Pseudorange observation
    pub P: u32,
    /// Carrier phase observation with typical sign convention.
    pub L: CarrierPhase,
    /// Carrier-to-Noise density
    pub cn0: u8,
    /// Lock indicator. This value changes whenever a satellite signal has lost
    /// and regained lock, indicating that the carrier phase ambiguity may have
    /// changed.
    pub lock: u16,
    /// GNSS signal identifier
    pub sid: GnssSignalDep,
}

impl PackedObsContentDepC {
    pub fn parse(_buf: &mut &[u8]) -> Result<PackedObsContentDepC, ::Error> {
        Ok(PackedObsContentDepC {
            P: _buf.read_u32::<LittleEndian>()?,
            L: CarrierPhase::parse(_buf)?,
            cn0: _buf.read_u8()?,
            lock: _buf.read_u16::<LittleEndian>()?,
            sid: GnssSignalDep::parse(_buf)?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<PackedObsContentDepC>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(PackedObsContentDepC::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(
        buf: &mut &[u8],
        n: usize,
    ) -> Result<Vec<PackedObsContentDepC>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(PackedObsContentDepC::parse(buf)?);
        }
        Ok(v)
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct GnssCapb {
    /// GPS SV active mask
    pub gps_active: u64,
    /// GPS L2C active mask
    pub gps_l2c: u64,
    /// GPS L5 active mask
    pub gps_l5: u64,
    /// GLO active mask
    pub glo_active: u32,
    /// GLO L2OF active mask
    pub glo_l2of: u32,
    /// GLO L3 active mask
    pub glo_l3: u32,
    /// SBAS active mask (PRNs 120..158, AN 7/62.2.2-18/18 Table B-23,
    /// https://www.caat.or.th/wp-content/uploads/2018/03/SL-2018.18.E-1.pdf)
    pub sbas_active: u64,
    /// SBAS L5 active mask (PRNs 120..158, AN 7/62.2.2-18/18 Table B-23,
    /// https://www.caat.or.th/wp-content/uploads/2018/03/SL-2018.18.E-1.pdf)
    pub sbas_l5: u64,
    /// BDS active mask
    pub bds_active: u64,
    /// BDS D2NAV active mask
    pub bds_d2nav: u64,
    /// BDS B2 active mask
    pub bds_b2: u64,
    /// BDS B2A active mask
    pub bds_b2a: u64,
    /// QZSS active mask
    pub qzss_active: u32,
    /// GAL active mask
    pub gal_active: u64,
    /// GAL E5 active mask
    pub gal_e5: u64,
}

impl GnssCapb {
    pub fn parse(_buf: &mut &[u8]) -> Result<GnssCapb, ::Error> {
        Ok(GnssCapb {
            gps_active: _buf.read_u64::<LittleEndian>()?,
            gps_l2c: _buf.read_u64::<LittleEndian>()?,
            gps_l5: _buf.read_u64::<LittleEndian>()?,
            glo_active: _buf.read_u32::<LittleEndian>()?,
            glo_l2of: _buf.read_u32::<LittleEndian>()?,
            glo_l3: _buf.read_u32::<LittleEndian>()?,
            sbas_active: _buf.read_u64::<LittleEndian>()?,
            sbas_l5: _buf.read_u64::<LittleEndian>()?,
            bds_active: _buf.read_u64::<LittleEndian>()?,
            bds_d2nav: _buf.read_u64::<LittleEndian>()?,
            bds_b2: _buf.read_u64::<LittleEndian>()?,
            bds_b2a: _buf.read_u64::<LittleEndian>()?,
            qzss_active: _buf.read_u32::<LittleEndian>()?,
            gal_active: _buf.read_u64::<LittleEndian>()?,
            gal_e5: _buf.read_u64::<LittleEndian>()?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<GnssCapb>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(GnssCapb::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(buf: &mut &[u8], n: usize) -> Result<Vec<GnssCapb>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(GnssCapb::parse(buf)?);
        }
        Ok(v)
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct AlmanacCommonContent {
    /// GNSS signal identifier
    pub sid: GnssSignal,
    /// Reference time of almanac
    pub toa: GPSTimeSec,
    /// User Range Accuracy
    pub ura: f64,
    /// Curve fit interval
    pub fit_interval: u32,
    /// Status of almanac, 1 = valid, 0 = invalid
    pub valid: u8,
    /// Satellite health status for GPS:   - bits 5-7: NAV data health status.
    /// See IS-GPS-200H     Table 20-VII: NAV Data Health Indications.   - bits
    /// 0-4: Signal health status. See IS-GPS-200H     Table 20-VIII. Codes for
    /// Health of SV Signal     Components. Satellite health status for GLO:
    /// See GLO ICD 5.1 table 5.1 for details   - bit 0: C(n), "unhealthy" flag
    /// that is transmitted within     non-immediate data and indicates overall
    /// constellation status     at the moment of almanac uploading.     '0'
    /// indicates malfunction of n-satellite.     '1' indicates that n-satellite
    /// is operational.   - bit 1: Bn(ln), '0' indicates the satellite is
    /// operational     and suitable for navigation.
    pub health_bits: u8,
}

impl AlmanacCommonContent {
    pub fn parse(_buf: &mut &[u8]) -> Result<AlmanacCommonContent, ::Error> {
        Ok(AlmanacCommonContent {
            sid: GnssSignal::parse(_buf)?,
            toa: GPSTimeSec::parse(_buf)?,
            ura: _buf.read_f64::<LittleEndian>()?,
            fit_interval: _buf.read_u32::<LittleEndian>()?,
            valid: _buf.read_u8()?,
            health_bits: _buf.read_u8()?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<AlmanacCommonContent>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(AlmanacCommonContent::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(
        buf: &mut &[u8],
        n: usize,
    ) -> Result<Vec<AlmanacCommonContent>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(AlmanacCommonContent::parse(buf)?);
        }
        Ok(v)
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct AlmanacCommonContentDep {
    /// GNSS signal identifier
    pub sid: GnssSignalDep,
    /// Reference time of almanac
    pub toa: GPSTimeSec,
    /// User Range Accuracy
    pub ura: f64,
    /// Curve fit interval
    pub fit_interval: u32,
    /// Status of almanac, 1 = valid, 0 = invalid
    pub valid: u8,
    /// Satellite health status for GPS:   - bits 5-7: NAV data health status.
    /// See IS-GPS-200H     Table 20-VII: NAV Data Health Indications.   - bits
    /// 0-4: Signal health status. See IS-GPS-200H     Table 20-VIII. Codes for
    /// Health of SV Signal     Components. Satellite health status for GLO:
    /// See GLO ICD 5.1 table 5.1 for details   - bit 0: C(n), "unhealthy" flag
    /// that is transmitted within     non-immediate data and indicates overall
    /// constellation status     at the moment of almanac uploading.     '0'
    /// indicates malfunction of n-satellite.     '1' indicates that n-satellite
    /// is operational.   - bit 1: Bn(ln), '0' indicates the satellite is
    /// operational     and suitable for navigation.
    pub health_bits: u8,
}

impl AlmanacCommonContentDep {
    pub fn parse(_buf: &mut &[u8]) -> Result<AlmanacCommonContentDep, ::Error> {
        Ok(AlmanacCommonContentDep {
            sid: GnssSignalDep::parse(_buf)?,
            toa: GPSTimeSec::parse(_buf)?,
            ura: _buf.read_f64::<LittleEndian>()?,
            fit_interval: _buf.read_u32::<LittleEndian>()?,
            valid: _buf.read_u8()?,
            health_bits: _buf.read_u8()?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<AlmanacCommonContentDep>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(AlmanacCommonContentDep::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(
        buf: &mut &[u8],
        n: usize,
    ) -> Result<Vec<AlmanacCommonContentDep>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(AlmanacCommonContentDep::parse(buf)?);
        }
        Ok(v)
    }
}

/// Satellite azimuth and elevation.
///
/// Satellite azimuth and elevation.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct SvAzEl {
    /// GNSS signal identifier
    pub sid: GnssSignal,
    /// Azimuth angle (range 0..179)
    pub az: u8,
    /// Elevation angle (range -90..90)
    pub el: i8,
}

impl SvAzEl {
    pub fn parse(_buf: &mut &[u8]) -> Result<SvAzEl, ::Error> {
        Ok(SvAzEl {
            sid: GnssSignal::parse(_buf)?,
            az: _buf.read_u8()?,
            el: _buf.read_i8()?,
        })
    }
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<SvAzEl>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push(SvAzEl::parse(buf)?);
        }
        Ok(v)
    }

    pub fn parse_array_limit(buf: &mut &[u8], n: usize) -> Result<Vec<SvAzEl>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push(SvAzEl::parse(buf)?);
        }
        Ok(v)
    }
}

/// Deprecated
///
/// Deprecated.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisDepA {
    pub sender_id: Option<u16>,
    /// Group delay differential between L1 and L2
    pub tgd: f64,
    /// Amplitude of the sine harmonic correction term to the orbit radius
    pub c_rs: f64,
    /// Amplitude of the cosine harmonic correction term to the orbit radius
    pub c_rc: f64,
    /// Amplitude of the cosine harmonic correction term to the argument of
    /// latitude
    pub c_uc: f64,
    /// Amplitude of the sine harmonic correction term to the argument of
    /// latitude
    pub c_us: f64,
    /// Amplitude of the cosine harmonic correction term to the angle of
    /// inclination
    pub c_ic: f64,
    /// Amplitude of the sine harmonic correction term to the angle of
    /// inclination
    pub c_is: f64,
    /// Mean motion difference
    pub dn: f64,
    /// Mean anomaly at reference time
    pub m0: f64,
    /// Eccentricity of satellite orbit
    pub ecc: f64,
    /// Square root of the semi-major axis of orbit
    pub sqrta: f64,
    /// Longitude of ascending node of orbit plane at weekly epoch
    pub omega0: f64,
    /// Rate of right ascension
    pub omegadot: f64,
    /// Argument of perigee
    pub w: f64,
    /// Inclination
    pub inc: f64,
    /// Inclination first derivative
    pub inc_dot: f64,
    /// Polynomial clock correction coefficient (clock bias)
    pub af0: f64,
    /// Polynomial clock correction coefficient (clock drift)
    pub af1: f64,
    /// Polynomial clock correction coefficient (rate of clock drift)
    pub af2: f64,
    /// Time of week
    pub toe_tow: f64,
    /// Week number
    pub toe_wn: u16,
    /// Clock reference time of week
    pub toc_tow: f64,
    /// Clock reference week number
    pub toc_wn: u16,
    /// Is valid?
    pub valid: u8,
    /// Satellite is healthy?
    pub healthy: u8,
    /// PRN being tracked
    pub prn: u8,
}

impl MsgEphemerisDepA {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisDepA, ::Error> {
        Ok(MsgEphemerisDepA {
            sender_id: None,
            tgd: _buf.read_f64::<LittleEndian>()?,
            c_rs: _buf.read_f64::<LittleEndian>()?,
            c_rc: _buf.read_f64::<LittleEndian>()?,
            c_uc: _buf.read_f64::<LittleEndian>()?,
            c_us: _buf.read_f64::<LittleEndian>()?,
            c_ic: _buf.read_f64::<LittleEndian>()?,
            c_is: _buf.read_f64::<LittleEndian>()?,
            dn: _buf.read_f64::<LittleEndian>()?,
            m0: _buf.read_f64::<LittleEndian>()?,
            ecc: _buf.read_f64::<LittleEndian>()?,
            sqrta: _buf.read_f64::<LittleEndian>()?,
            omega0: _buf.read_f64::<LittleEndian>()?,
            omegadot: _buf.read_f64::<LittleEndian>()?,
            w: _buf.read_f64::<LittleEndian>()?,
            inc: _buf.read_f64::<LittleEndian>()?,
            inc_dot: _buf.read_f64::<LittleEndian>()?,
            af0: _buf.read_f64::<LittleEndian>()?,
            af1: _buf.read_f64::<LittleEndian>()?,
            af2: _buf.read_f64::<LittleEndian>()?,
            toe_tow: _buf.read_f64::<LittleEndian>()?,
            toe_wn: _buf.read_u16::<LittleEndian>()?,
            toc_tow: _buf.read_f64::<LittleEndian>()?,
            toc_wn: _buf.read_u16::<LittleEndian>()?,
            valid: _buf.read_u8()?,
            healthy: _buf.read_u8()?,
            prn: _buf.read_u8()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisDepA {
    const MSG_ID: u16 = 26;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Deprecated
///
/// This observation message has been deprecated in favor of
/// observations that are more interoperable. This message
/// should be used for observations referenced to
/// a nominal pseudorange which are not interoperable with
/// most 3rd party GNSS receievers or typical RTCMv3
/// observations.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgObsDepB {
    pub sender_id: Option<u16>,
    /// Header of a GPS observation message
    pub header: ObservationHeaderDep,
    /// Pseudorange and carrier phase observation for a satellite being tracked.
    pub obs: Vec<PackedObsContentDepB>,
}

impl MsgObsDepB {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgObsDepB, ::Error> {
        Ok(MsgObsDepB {
            sender_id: None,
            header: ObservationHeaderDep::parse(_buf)?,
            obs: PackedObsContentDepB::parse_array(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgObsDepB {
    const MSG_ID: u16 = 67;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Base station position
///
/// The base station position message is the position reported by
/// the base station itself. It is used for pseudo-absolute RTK
/// positioning, and is required to be a high-accuracy surveyed
/// location of the base station. Any error here will result in an
/// error in the pseudo-absolute position output.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgBasePosLLH {
    pub sender_id: Option<u16>,
    /// Latitude
    pub lat: f64,
    /// Longitude
    pub lon: f64,
    /// Height
    pub height: f64,
}

impl MsgBasePosLLH {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgBasePosLLH, ::Error> {
        Ok(MsgBasePosLLH {
            sender_id: None,
            lat: _buf.read_f64::<LittleEndian>()?,
            lon: _buf.read_f64::<LittleEndian>()?,
            height: _buf.read_f64::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgBasePosLLH {
    const MSG_ID: u16 = 68;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Deprecated
///
/// Deprecated.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgObsDepA {
    pub sender_id: Option<u16>,
    /// Header of a GPS observation message
    pub header: ObservationHeaderDep,
    /// Pseudorange and carrier phase observation for a satellite being tracked.
    pub obs: Vec<PackedObsContentDepA>,
}

impl MsgObsDepA {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgObsDepA, ::Error> {
        Ok(MsgObsDepA {
            sender_id: None,
            header: ObservationHeaderDep::parse(_buf)?,
            obs: PackedObsContentDepA::parse_array(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgObsDepA {
    const MSG_ID: u16 = 69;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Deprecated
///
/// Deprecated.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisDepB {
    pub sender_id: Option<u16>,
    /// Group delay differential between L1 and L2
    pub tgd: f64,
    /// Amplitude of the sine harmonic correction term to the orbit radius
    pub c_rs: f64,
    /// Amplitude of the cosine harmonic correction term to the orbit radius
    pub c_rc: f64,
    /// Amplitude of the cosine harmonic correction term to the argument of
    /// latitude
    pub c_uc: f64,
    /// Amplitude of the sine harmonic correction term to the argument of
    /// latitude
    pub c_us: f64,
    /// Amplitude of the cosine harmonic correction term to the angle of
    /// inclination
    pub c_ic: f64,
    /// Amplitude of the sine harmonic correction term to the angle of
    /// inclination
    pub c_is: f64,
    /// Mean motion difference
    pub dn: f64,
    /// Mean anomaly at reference time
    pub m0: f64,
    /// Eccentricity of satellite orbit
    pub ecc: f64,
    /// Square root of the semi-major axis of orbit
    pub sqrta: f64,
    /// Longitude of ascending node of orbit plane at weekly epoch
    pub omega0: f64,
    /// Rate of right ascension
    pub omegadot: f64,
    /// Argument of perigee
    pub w: f64,
    /// Inclination
    pub inc: f64,
    /// Inclination first derivative
    pub inc_dot: f64,
    /// Polynomial clock correction coefficient (clock bias)
    pub af0: f64,
    /// Polynomial clock correction coefficient (clock drift)
    pub af1: f64,
    /// Polynomial clock correction coefficient (rate of clock drift)
    pub af2: f64,
    /// Time of week
    pub toe_tow: f64,
    /// Week number
    pub toe_wn: u16,
    /// Clock reference time of week
    pub toc_tow: f64,
    /// Clock reference week number
    pub toc_wn: u16,
    /// Is valid?
    pub valid: u8,
    /// Satellite is healthy?
    pub healthy: u8,
    /// PRN being tracked
    pub prn: u8,
    /// Issue of ephemeris data
    pub iode: u8,
}

impl MsgEphemerisDepB {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisDepB, ::Error> {
        Ok(MsgEphemerisDepB {
            sender_id: None,
            tgd: _buf.read_f64::<LittleEndian>()?,
            c_rs: _buf.read_f64::<LittleEndian>()?,
            c_rc: _buf.read_f64::<LittleEndian>()?,
            c_uc: _buf.read_f64::<LittleEndian>()?,
            c_us: _buf.read_f64::<LittleEndian>()?,
            c_ic: _buf.read_f64::<LittleEndian>()?,
            c_is: _buf.read_f64::<LittleEndian>()?,
            dn: _buf.read_f64::<LittleEndian>()?,
            m0: _buf.read_f64::<LittleEndian>()?,
            ecc: _buf.read_f64::<LittleEndian>()?,
            sqrta: _buf.read_f64::<LittleEndian>()?,
            omega0: _buf.read_f64::<LittleEndian>()?,
            omegadot: _buf.read_f64::<LittleEndian>()?,
            w: _buf.read_f64::<LittleEndian>()?,
            inc: _buf.read_f64::<LittleEndian>()?,
            inc_dot: _buf.read_f64::<LittleEndian>()?,
            af0: _buf.read_f64::<LittleEndian>()?,
            af1: _buf.read_f64::<LittleEndian>()?,
            af2: _buf.read_f64::<LittleEndian>()?,
            toe_tow: _buf.read_f64::<LittleEndian>()?,
            toe_wn: _buf.read_u16::<LittleEndian>()?,
            toc_tow: _buf.read_f64::<LittleEndian>()?,
            toc_wn: _buf.read_u16::<LittleEndian>()?,
            valid: _buf.read_u8()?,
            healthy: _buf.read_u8()?,
            prn: _buf.read_u8()?,
            iode: _buf.read_u8()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisDepB {
    const MSG_ID: u16 = 70;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris
///
/// The ephemeris message returns a set of satellite orbit
/// parameters that is used to calculate GPS satellite position,
/// velocity, and clock offset. Please see the Navstar GPS
/// Space Segment/Navigation user interfaces (ICD-GPS-200, Table
/// 20-III) for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisDepC {
    pub sender_id: Option<u16>,
    /// Group delay differential between L1 and L2
    pub tgd: f64,
    /// Amplitude of the sine harmonic correction term to the orbit radius
    pub c_rs: f64,
    /// Amplitude of the cosine harmonic correction term to the orbit radius
    pub c_rc: f64,
    /// Amplitude of the cosine harmonic correction term to the argument of
    /// latitude
    pub c_uc: f64,
    /// Amplitude of the sine harmonic correction term to the argument of
    /// latitude
    pub c_us: f64,
    /// Amplitude of the cosine harmonic correction term to the angle of
    /// inclination
    pub c_ic: f64,
    /// Amplitude of the sine harmonic correction term to the angle of
    /// inclination
    pub c_is: f64,
    /// Mean motion difference
    pub dn: f64,
    /// Mean anomaly at reference time
    pub m0: f64,
    /// Eccentricity of satellite orbit
    pub ecc: f64,
    /// Square root of the semi-major axis of orbit
    pub sqrta: f64,
    /// Longitude of ascending node of orbit plane at weekly epoch
    pub omega0: f64,
    /// Rate of right ascension
    pub omegadot: f64,
    /// Argument of perigee
    pub w: f64,
    /// Inclination
    pub inc: f64,
    /// Inclination first derivative
    pub inc_dot: f64,
    /// Polynomial clock correction coefficient (clock bias)
    pub af0: f64,
    /// Polynomial clock correction coefficient (clock drift)
    pub af1: f64,
    /// Polynomial clock correction coefficient (rate of clock drift)
    pub af2: f64,
    /// Time of week
    pub toe_tow: f64,
    /// Week number
    pub toe_wn: u16,
    /// Clock reference time of week
    pub toc_tow: f64,
    /// Clock reference week number
    pub toc_wn: u16,
    /// Is valid?
    pub valid: u8,
    /// Satellite is healthy?
    pub healthy: u8,
    /// GNSS signal identifier
    pub sid: GnssSignalDep,
    /// Issue of ephemeris data
    pub iode: u8,
    /// Issue of clock data
    pub iodc: u16,
    /// Reserved field
    pub reserved: u32,
}

impl MsgEphemerisDepC {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisDepC, ::Error> {
        Ok(MsgEphemerisDepC {
            sender_id: None,
            tgd: _buf.read_f64::<LittleEndian>()?,
            c_rs: _buf.read_f64::<LittleEndian>()?,
            c_rc: _buf.read_f64::<LittleEndian>()?,
            c_uc: _buf.read_f64::<LittleEndian>()?,
            c_us: _buf.read_f64::<LittleEndian>()?,
            c_ic: _buf.read_f64::<LittleEndian>()?,
            c_is: _buf.read_f64::<LittleEndian>()?,
            dn: _buf.read_f64::<LittleEndian>()?,
            m0: _buf.read_f64::<LittleEndian>()?,
            ecc: _buf.read_f64::<LittleEndian>()?,
            sqrta: _buf.read_f64::<LittleEndian>()?,
            omega0: _buf.read_f64::<LittleEndian>()?,
            omegadot: _buf.read_f64::<LittleEndian>()?,
            w: _buf.read_f64::<LittleEndian>()?,
            inc: _buf.read_f64::<LittleEndian>()?,
            inc_dot: _buf.read_f64::<LittleEndian>()?,
            af0: _buf.read_f64::<LittleEndian>()?,
            af1: _buf.read_f64::<LittleEndian>()?,
            af2: _buf.read_f64::<LittleEndian>()?,
            toe_tow: _buf.read_f64::<LittleEndian>()?,
            toe_wn: _buf.read_u16::<LittleEndian>()?,
            toc_tow: _buf.read_f64::<LittleEndian>()?,
            toc_wn: _buf.read_u16::<LittleEndian>()?,
            valid: _buf.read_u8()?,
            healthy: _buf.read_u8()?,
            sid: GnssSignalDep::parse(_buf)?,
            iode: _buf.read_u8()?,
            iodc: _buf.read_u16::<LittleEndian>()?,
            reserved: _buf.read_u32::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisDepC {
    const MSG_ID: u16 = 71;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Base station position in ECEF
///
/// The base station position message is the position reported by
/// the base station itself in absolute Earth Centered Earth Fixed
/// coordinates. It is used for pseudo-absolute RTK positioning, and
/// is required to be a high-accuracy surveyed location of the base
/// station. Any error here will result in an error in the
/// pseudo-absolute position output.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgBasePosECEF {
    pub sender_id: Option<u16>,
    /// ECEF X coodinate
    pub x: f64,
    /// ECEF Y coordinate
    pub y: f64,
    /// ECEF Z coordinate
    pub z: f64,
}

impl MsgBasePosECEF {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgBasePosECEF, ::Error> {
        Ok(MsgBasePosECEF {
            sender_id: None,
            x: _buf.read_f64::<LittleEndian>()?,
            y: _buf.read_f64::<LittleEndian>()?,
            z: _buf.read_f64::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgBasePosECEF {
    const MSG_ID: u16 = 72;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Deprecated
///
/// The GPS observations message reports all the raw pseudorange and
/// carrier phase observations for the satellites being tracked by
/// the device. Carrier phase observation here is represented as a
/// 40-bit fixed point number with Q32.8 layout (i.e. 32-bits of
/// whole cycles and 8-bits of fractional cycles). The observations
/// are interoperable with 3rd party receivers and conform
/// with typical RTCMv3 GNSS observations.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgObsDepC {
    pub sender_id: Option<u16>,
    /// Header of a GPS observation message
    pub header: ObservationHeaderDep,
    /// Pseudorange and carrier phase observation for a satellite being tracked.
    pub obs: Vec<PackedObsContentDepC>,
}

impl MsgObsDepC {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgObsDepC, ::Error> {
        Ok(MsgObsDepC {
            sender_id: None,
            header: ObservationHeaderDep::parse(_buf)?,
            obs: PackedObsContentDepC::parse_array(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgObsDepC {
    const MSG_ID: u16 = 73;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// GPS satellite observations
///
/// The GPS observations message reports all the raw pseudorange and
/// carrier phase observations for the satellites being tracked by
/// the device. Carrier phase observation here is represented as a
/// 40-bit fixed point number with Q32.8 layout (i.e. 32-bits of
/// whole cycles and 8-bits of fractional cycles). The observations
/// are be interoperable with 3rd party receivers and conform
/// with typical RTCMv3 GNSS observations.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgObs {
    pub sender_id: Option<u16>,
    /// Header of a GPS observation message
    pub header: ObservationHeader,
    /// Pseudorange and carrier phase observation for a satellite being tracked.
    pub obs: Vec<PackedObsContent>,
}

impl MsgObs {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgObs, ::Error> {
        Ok(MsgObs {
            sender_id: None,
            header: ObservationHeader::parse(_buf)?,
            obs: PackedObsContent::parse_array(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgObs {
    const MSG_ID: u16 = 74;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris for GPS
///
/// The almanac message returns a set of satellite orbit parameters. Almanac
/// data is not very precise and is considered valid for up to several months.
/// Please see the Navstar GPS Space Segment/Navigation user interfaces
/// (ICD-GPS-200, Chapter 20.3.3.5.1.2 Almanac Data) for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgAlmanacGPSDep {
    pub sender_id: Option<u16>,
    /// Values common for all almanac types
    pub common: AlmanacCommonContentDep,
    /// Mean anomaly at reference time
    pub m0: f64,
    /// Eccentricity of satellite orbit
    pub ecc: f64,
    /// Square root of the semi-major axis of orbit
    pub sqrta: f64,
    /// Longitude of ascending node of orbit plane at weekly epoch
    pub omega0: f64,
    /// Rate of right ascension
    pub omegadot: f64,
    /// Argument of perigee
    pub w: f64,
    /// Inclination
    pub inc: f64,
    /// Polynomial clock correction coefficient (clock bias)
    pub af0: f64,
    /// Polynomial clock correction coefficient (clock drift)
    pub af1: f64,
}

impl MsgAlmanacGPSDep {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgAlmanacGPSDep, ::Error> {
        Ok(MsgAlmanacGPSDep {
            sender_id: None,
            common: AlmanacCommonContentDep::parse(_buf)?,
            m0: _buf.read_f64::<LittleEndian>()?,
            ecc: _buf.read_f64::<LittleEndian>()?,
            sqrta: _buf.read_f64::<LittleEndian>()?,
            omega0: _buf.read_f64::<LittleEndian>()?,
            omegadot: _buf.read_f64::<LittleEndian>()?,
            w: _buf.read_f64::<LittleEndian>()?,
            inc: _buf.read_f64::<LittleEndian>()?,
            af0: _buf.read_f64::<LittleEndian>()?,
            af1: _buf.read_f64::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgAlmanacGPSDep {
    const MSG_ID: u16 = 112;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris for GLO
///
/// The almanac message returns a set of satellite orbit parameters. Almanac
/// data is not very precise and is considered valid for up to several months.
/// Please see the GLO ICD 5.1 "Chapter 4.5 Non-immediate information and
/// almanac" for details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgAlmanacGloDep {
    pub sender_id: Option<u16>,
    /// Values common for all almanac types
    pub common: AlmanacCommonContentDep,
    /// Longitude of the first ascending node of the orbit in PZ-90.02
    /// coordinate system
    pub lambda_na: f64,
    /// Time of the first ascending node passage
    pub t_lambda_na: f64,
    /// Value of inclination at instant of t_lambda
    pub i: f64,
    /// Value of Draconian period at instant of t_lambda
    pub t: f64,
    /// Rate of change of the Draconian period
    pub t_dot: f64,
    /// Eccentricity at instant of t_lambda
    pub epsilon: f64,
    /// Argument of perigee at instant of t_lambda
    pub omega: f64,
}

impl MsgAlmanacGloDep {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgAlmanacGloDep, ::Error> {
        Ok(MsgAlmanacGloDep {
            sender_id: None,
            common: AlmanacCommonContentDep::parse(_buf)?,
            lambda_na: _buf.read_f64::<LittleEndian>()?,
            t_lambda_na: _buf.read_f64::<LittleEndian>()?,
            i: _buf.read_f64::<LittleEndian>()?,
            t: _buf.read_f64::<LittleEndian>()?,
            t_dot: _buf.read_f64::<LittleEndian>()?,
            epsilon: _buf.read_f64::<LittleEndian>()?,
            omega: _buf.read_f64::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgAlmanacGloDep {
    const MSG_ID: u16 = 113;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris for GPS
///
/// The almanac message returns a set of satellite orbit parameters. Almanac
/// data is not very precise and is considered valid for up to several months.
/// Please see the Navstar GPS Space Segment/Navigation user interfaces
/// (ICD-GPS-200, Chapter 20.3.3.5.1.2 Almanac Data) for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgAlmanacGPS {
    pub sender_id: Option<u16>,
    /// Values common for all almanac types
    pub common: AlmanacCommonContent,
    /// Mean anomaly at reference time
    pub m0: f64,
    /// Eccentricity of satellite orbit
    pub ecc: f64,
    /// Square root of the semi-major axis of orbit
    pub sqrta: f64,
    /// Longitude of ascending node of orbit plane at weekly epoch
    pub omega0: f64,
    /// Rate of right ascension
    pub omegadot: f64,
    /// Argument of perigee
    pub w: f64,
    /// Inclination
    pub inc: f64,
    /// Polynomial clock correction coefficient (clock bias)
    pub af0: f64,
    /// Polynomial clock correction coefficient (clock drift)
    pub af1: f64,
}

impl MsgAlmanacGPS {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgAlmanacGPS, ::Error> {
        Ok(MsgAlmanacGPS {
            sender_id: None,
            common: AlmanacCommonContent::parse(_buf)?,
            m0: _buf.read_f64::<LittleEndian>()?,
            ecc: _buf.read_f64::<LittleEndian>()?,
            sqrta: _buf.read_f64::<LittleEndian>()?,
            omega0: _buf.read_f64::<LittleEndian>()?,
            omegadot: _buf.read_f64::<LittleEndian>()?,
            w: _buf.read_f64::<LittleEndian>()?,
            inc: _buf.read_f64::<LittleEndian>()?,
            af0: _buf.read_f64::<LittleEndian>()?,
            af1: _buf.read_f64::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgAlmanacGPS {
    const MSG_ID: u16 = 114;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris for GLO
///
/// The almanac message returns a set of satellite orbit parameters. Almanac
/// data is not very precise and is considered valid for up to several months.
/// Please see the GLO ICD 5.1 "Chapter 4.5 Non-immediate information and
/// almanac" for details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgAlmanacGlo {
    pub sender_id: Option<u16>,
    /// Values common for all almanac types
    pub common: AlmanacCommonContent,
    /// Longitude of the first ascending node of the orbit in PZ-90.02
    /// coordinate system
    pub lambda_na: f64,
    /// Time of the first ascending node passage
    pub t_lambda_na: f64,
    /// Value of inclination at instant of t_lambda
    pub i: f64,
    /// Value of Draconian period at instant of t_lambda
    pub t: f64,
    /// Rate of change of the Draconian period
    pub t_dot: f64,
    /// Eccentricity at instant of t_lambda
    pub epsilon: f64,
    /// Argument of perigee at instant of t_lambda
    pub omega: f64,
}

impl MsgAlmanacGlo {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgAlmanacGlo, ::Error> {
        Ok(MsgAlmanacGlo {
            sender_id: None,
            common: AlmanacCommonContent::parse(_buf)?,
            lambda_na: _buf.read_f64::<LittleEndian>()?,
            t_lambda_na: _buf.read_f64::<LittleEndian>()?,
            i: _buf.read_f64::<LittleEndian>()?,
            t: _buf.read_f64::<LittleEndian>()?,
            t_dot: _buf.read_f64::<LittleEndian>()?,
            epsilon: _buf.read_f64::<LittleEndian>()?,
            omega: _buf.read_f64::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgAlmanacGlo {
    const MSG_ID: u16 = 115;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// GLONASS L1/L2 Code-Phase biases
///
/// The GLONASS L1/L2 Code-Phase biases allows to perform
/// GPS+GLONASS integer ambiguity resolution for baselines
/// with mixed receiver types (e.g. receiver of different
/// manufacturers)
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgGloBiases {
    pub sender_id: Option<u16>,
    /// GLONASS FDMA signals mask
    pub mask: u8,
    /// GLONASS L1 C/A Code-Phase Bias
    pub l1ca_bias: i16,
    /// GLONASS L1 P Code-Phase Bias
    pub l1p_bias: i16,
    /// GLONASS L2 C/A Code-Phase Bias
    pub l2ca_bias: i16,
    /// GLONASS L2 P Code-Phase Bias
    pub l2p_bias: i16,
}

impl MsgGloBiases {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgGloBiases, ::Error> {
        Ok(MsgGloBiases {
            sender_id: None,
            mask: _buf.read_u8()?,
            l1ca_bias: _buf.read_i16::<LittleEndian>()?,
            l1p_bias: _buf.read_i16::<LittleEndian>()?,
            l2ca_bias: _buf.read_i16::<LittleEndian>()?,
            l2p_bias: _buf.read_i16::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgGloBiases {
    const MSG_ID: u16 = 117;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris
///
/// The ephemeris message returns a set of satellite orbit
/// parameters that is used to calculate GPS satellite position,
/// velocity, and clock offset. Please see the Navstar GPS
/// Space Segment/Navigation user interfaces (ICD-GPS-200, Table
/// 20-III) for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisDepD {
    pub sender_id: Option<u16>,
    /// Group delay differential between L1 and L2
    pub tgd: f64,
    /// Amplitude of the sine harmonic correction term to the orbit radius
    pub c_rs: f64,
    /// Amplitude of the cosine harmonic correction term to the orbit radius
    pub c_rc: f64,
    /// Amplitude of the cosine harmonic correction term to the argument of
    /// latitude
    pub c_uc: f64,
    /// Amplitude of the sine harmonic correction term to the argument of
    /// latitude
    pub c_us: f64,
    /// Amplitude of the cosine harmonic correction term to the angle of
    /// inclination
    pub c_ic: f64,
    /// Amplitude of the sine harmonic correction term to the angle of
    /// inclination
    pub c_is: f64,
    /// Mean motion difference
    pub dn: f64,
    /// Mean anomaly at reference time
    pub m0: f64,
    /// Eccentricity of satellite orbit
    pub ecc: f64,
    /// Square root of the semi-major axis of orbit
    pub sqrta: f64,
    /// Longitude of ascending node of orbit plane at weekly epoch
    pub omega0: f64,
    /// Rate of right ascension
    pub omegadot: f64,
    /// Argument of perigee
    pub w: f64,
    /// Inclination
    pub inc: f64,
    /// Inclination first derivative
    pub inc_dot: f64,
    /// Polynomial clock correction coefficient (clock bias)
    pub af0: f64,
    /// Polynomial clock correction coefficient (clock drift)
    pub af1: f64,
    /// Polynomial clock correction coefficient (rate of clock drift)
    pub af2: f64,
    /// Time of week
    pub toe_tow: f64,
    /// Week number
    pub toe_wn: u16,
    /// Clock reference time of week
    pub toc_tow: f64,
    /// Clock reference week number
    pub toc_wn: u16,
    /// Is valid?
    pub valid: u8,
    /// Satellite is healthy?
    pub healthy: u8,
    /// GNSS signal identifier
    pub sid: GnssSignalDep,
    /// Issue of ephemeris data
    pub iode: u8,
    /// Issue of clock data
    pub iodc: u16,
    /// Reserved field
    pub reserved: u32,
}

impl MsgEphemerisDepD {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisDepD, ::Error> {
        Ok(MsgEphemerisDepD {
            sender_id: None,
            tgd: _buf.read_f64::<LittleEndian>()?,
            c_rs: _buf.read_f64::<LittleEndian>()?,
            c_rc: _buf.read_f64::<LittleEndian>()?,
            c_uc: _buf.read_f64::<LittleEndian>()?,
            c_us: _buf.read_f64::<LittleEndian>()?,
            c_ic: _buf.read_f64::<LittleEndian>()?,
            c_is: _buf.read_f64::<LittleEndian>()?,
            dn: _buf.read_f64::<LittleEndian>()?,
            m0: _buf.read_f64::<LittleEndian>()?,
            ecc: _buf.read_f64::<LittleEndian>()?,
            sqrta: _buf.read_f64::<LittleEndian>()?,
            omega0: _buf.read_f64::<LittleEndian>()?,
            omegadot: _buf.read_f64::<LittleEndian>()?,
            w: _buf.read_f64::<LittleEndian>()?,
            inc: _buf.read_f64::<LittleEndian>()?,
            inc_dot: _buf.read_f64::<LittleEndian>()?,
            af0: _buf.read_f64::<LittleEndian>()?,
            af1: _buf.read_f64::<LittleEndian>()?,
            af2: _buf.read_f64::<LittleEndian>()?,
            toe_tow: _buf.read_f64::<LittleEndian>()?,
            toe_wn: _buf.read_u16::<LittleEndian>()?,
            toc_tow: _buf.read_f64::<LittleEndian>()?,
            toc_wn: _buf.read_u16::<LittleEndian>()?,
            valid: _buf.read_u8()?,
            healthy: _buf.read_u8()?,
            sid: GnssSignalDep::parse(_buf)?,
            iode: _buf.read_u8()?,
            iodc: _buf.read_u16::<LittleEndian>()?,
            reserved: _buf.read_u32::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisDepD {
    const MSG_ID: u16 = 128;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris for GPS
///
/// The ephemeris message returns a set of satellite orbit
/// parameters that is used to calculate GPS satellite position,
/// velocity, and clock offset. Please see the Navstar GPS
/// Space Segment/Navigation user interfaces (ICD-GPS-200, Table
/// 20-III) for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisGPSDepE {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContentDepA,
    /// Group delay differential between L1 and L2
    pub tgd: f64,
    /// Amplitude of the sine harmonic correction term to the orbit radius
    pub c_rs: f64,
    /// Amplitude of the cosine harmonic correction term to the orbit radius
    pub c_rc: f64,
    /// Amplitude of the cosine harmonic correction term to the argument of
    /// latitude
    pub c_uc: f64,
    /// Amplitude of the sine harmonic correction term to the argument of
    /// latitude
    pub c_us: f64,
    /// Amplitude of the cosine harmonic correction term to the angle of
    /// inclination
    pub c_ic: f64,
    /// Amplitude of the sine harmonic correction term to the angle of
    /// inclination
    pub c_is: f64,
    /// Mean motion difference
    pub dn: f64,
    /// Mean anomaly at reference time
    pub m0: f64,
    /// Eccentricity of satellite orbit
    pub ecc: f64,
    /// Square root of the semi-major axis of orbit
    pub sqrta: f64,
    /// Longitude of ascending node of orbit plane at weekly epoch
    pub omega0: f64,
    /// Rate of right ascension
    pub omegadot: f64,
    /// Argument of perigee
    pub w: f64,
    /// Inclination
    pub inc: f64,
    /// Inclination first derivative
    pub inc_dot: f64,
    /// Polynomial clock correction coefficient (clock bias)
    pub af0: f64,
    /// Polynomial clock correction coefficient (clock drift)
    pub af1: f64,
    /// Polynomial clock correction coefficient (rate of clock drift)
    pub af2: f64,
    /// Clock reference
    pub toc: GPSTimeDep,
    /// Issue of ephemeris data
    pub iode: u8,
    /// Issue of clock data
    pub iodc: u16,
}

impl MsgEphemerisGPSDepE {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisGPSDepE, ::Error> {
        Ok(MsgEphemerisGPSDepE {
            sender_id: None,
            common: EphemerisCommonContentDepA::parse(_buf)?,
            tgd: _buf.read_f64::<LittleEndian>()?,
            c_rs: _buf.read_f64::<LittleEndian>()?,
            c_rc: _buf.read_f64::<LittleEndian>()?,
            c_uc: _buf.read_f64::<LittleEndian>()?,
            c_us: _buf.read_f64::<LittleEndian>()?,
            c_ic: _buf.read_f64::<LittleEndian>()?,
            c_is: _buf.read_f64::<LittleEndian>()?,
            dn: _buf.read_f64::<LittleEndian>()?,
            m0: _buf.read_f64::<LittleEndian>()?,
            ecc: _buf.read_f64::<LittleEndian>()?,
            sqrta: _buf.read_f64::<LittleEndian>()?,
            omega0: _buf.read_f64::<LittleEndian>()?,
            omegadot: _buf.read_f64::<LittleEndian>()?,
            w: _buf.read_f64::<LittleEndian>()?,
            inc: _buf.read_f64::<LittleEndian>()?,
            inc_dot: _buf.read_f64::<LittleEndian>()?,
            af0: _buf.read_f64::<LittleEndian>()?,
            af1: _buf.read_f64::<LittleEndian>()?,
            af2: _buf.read_f64::<LittleEndian>()?,
            toc: GPSTimeDep::parse(_buf)?,
            iode: _buf.read_u8()?,
            iodc: _buf.read_u16::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisGPSDepE {
    const MSG_ID: u16 = 129;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisSbasDepA {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContentDepA,
    /// Position of the GEO at time toe
    pub pos: Vec<f64>,
    /// Velocity of the GEO at time toe
    pub vel: Vec<f64>,
    /// Acceleration of the GEO at time toe
    pub acc: Vec<f64>,
    /// Time offset of the GEO clock w.r.t. SBAS Network Time
    pub a_gf0: f64,
    /// Drift of the GEO clock w.r.t. SBAS Network Time
    pub a_gf1: f64,
}

impl MsgEphemerisSbasDepA {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisSbasDepA, ::Error> {
        Ok(MsgEphemerisSbasDepA {
            sender_id: None,
            common: EphemerisCommonContentDepA::parse(_buf)?,
            pos: ::parser::read_double_array_limit(_buf, 3)?,
            vel: ::parser::read_double_array_limit(_buf, 3)?,
            acc: ::parser::read_double_array_limit(_buf, 3)?,
            a_gf0: _buf.read_f64::<LittleEndian>()?,
            a_gf1: _buf.read_f64::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisSbasDepA {
    const MSG_ID: u16 = 130;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris for GLO
///
/// The ephemeris message returns a set of satellite orbit
/// parameters that is used to calculate GLO satellite position,
/// velocity, and clock offset. Please see the GLO ICD 5.1 "Table 4.5
/// Characteristics of words of immediate information (ephemeris parameters)"
/// for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisGloDepA {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContentDepA,
    /// Relative deviation of predicted carrier frequency from nominal
    pub gamma: f64,
    /// Correction to the SV time
    pub tau: f64,
    /// Position of the SV at tb in PZ-90.02 coordinates system
    pub pos: Vec<f64>,
    /// Velocity vector of the SV at tb in PZ-90.02 coordinates system
    pub vel: Vec<f64>,
    /// Acceleration vector of the SV at tb in PZ-90.02 coordinates sys
    pub acc: Vec<f64>,
}

impl MsgEphemerisGloDepA {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisGloDepA, ::Error> {
        Ok(MsgEphemerisGloDepA {
            sender_id: None,
            common: EphemerisCommonContentDepA::parse(_buf)?,
            gamma: _buf.read_f64::<LittleEndian>()?,
            tau: _buf.read_f64::<LittleEndian>()?,
            pos: ::parser::read_double_array_limit(_buf, 3)?,
            vel: ::parser::read_double_array_limit(_buf, 3)?,
            acc: ::parser::read_double_array_limit(_buf, 3)?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisGloDepA {
    const MSG_ID: u16 = 131;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Deprecated
///
/// This observation message has been deprecated in favor of
/// ephemeris message using floats for size reduction.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisSbasDepB {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContentDepB,
    /// Position of the GEO at time toe
    pub pos: Vec<f64>,
    /// Velocity of the GEO at time toe
    pub vel: Vec<f64>,
    /// Acceleration of the GEO at time toe
    pub acc: Vec<f64>,
    /// Time offset of the GEO clock w.r.t. SBAS Network Time
    pub a_gf0: f64,
    /// Drift of the GEO clock w.r.t. SBAS Network Time
    pub a_gf1: f64,
}

impl MsgEphemerisSbasDepB {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisSbasDepB, ::Error> {
        Ok(MsgEphemerisSbasDepB {
            sender_id: None,
            common: EphemerisCommonContentDepB::parse(_buf)?,
            pos: ::parser::read_double_array_limit(_buf, 3)?,
            vel: ::parser::read_double_array_limit(_buf, 3)?,
            acc: ::parser::read_double_array_limit(_buf, 3)?,
            a_gf0: _buf.read_f64::<LittleEndian>()?,
            a_gf1: _buf.read_f64::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisSbasDepB {
    const MSG_ID: u16 = 132;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris for GLO
///
/// The ephemeris message returns a set of satellite orbit
/// parameters that is used to calculate GLO satellite position,
/// velocity, and clock offset. Please see the GLO ICD 5.1 "Table 4.5
/// Characteristics of words of immediate information (ephemeris parameters)"
/// for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisGloDepB {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContentDepB,
    /// Relative deviation of predicted carrier frequency from nominal
    pub gamma: f64,
    /// Correction to the SV time
    pub tau: f64,
    /// Position of the SV at tb in PZ-90.02 coordinates system
    pub pos: Vec<f64>,
    /// Velocity vector of the SV at tb in PZ-90.02 coordinates system
    pub vel: Vec<f64>,
    /// Acceleration vector of the SV at tb in PZ-90.02 coordinates sys
    pub acc: Vec<f64>,
}

impl MsgEphemerisGloDepB {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisGloDepB, ::Error> {
        Ok(MsgEphemerisGloDepB {
            sender_id: None,
            common: EphemerisCommonContentDepB::parse(_buf)?,
            gamma: _buf.read_f64::<LittleEndian>()?,
            tau: _buf.read_f64::<LittleEndian>()?,
            pos: ::parser::read_double_array_limit(_buf, 3)?,
            vel: ::parser::read_double_array_limit(_buf, 3)?,
            acc: ::parser::read_double_array_limit(_buf, 3)?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisGloDepB {
    const MSG_ID: u16 = 133;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Deprecated
///
/// This observation message has been deprecated in favor of
/// ephemeris message using floats for size reduction.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisGPSDepF {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContentDepB,
    /// Group delay differential between L1 and L2
    pub tgd: f64,
    /// Amplitude of the sine harmonic correction term to the orbit radius
    pub c_rs: f64,
    /// Amplitude of the cosine harmonic correction term to the orbit radius
    pub c_rc: f64,
    /// Amplitude of the cosine harmonic correction term to the argument of
    /// latitude
    pub c_uc: f64,
    /// Amplitude of the sine harmonic correction term to the argument of
    /// latitude
    pub c_us: f64,
    /// Amplitude of the cosine harmonic correction term to the angle of
    /// inclination
    pub c_ic: f64,
    /// Amplitude of the sine harmonic correction term to the angle of
    /// inclination
    pub c_is: f64,
    /// Mean motion difference
    pub dn: f64,
    /// Mean anomaly at reference time
    pub m0: f64,
    /// Eccentricity of satellite orbit
    pub ecc: f64,
    /// Square root of the semi-major axis of orbit
    pub sqrta: f64,
    /// Longitude of ascending node of orbit plane at weekly epoch
    pub omega0: f64,
    /// Rate of right ascension
    pub omegadot: f64,
    /// Argument of perigee
    pub w: f64,
    /// Inclination
    pub inc: f64,
    /// Inclination first derivative
    pub inc_dot: f64,
    /// Polynomial clock correction coefficient (clock bias)
    pub af0: f64,
    /// Polynomial clock correction coefficient (clock drift)
    pub af1: f64,
    /// Polynomial clock correction coefficient (rate of clock drift)
    pub af2: f64,
    /// Clock reference
    pub toc: GPSTimeSec,
    /// Issue of ephemeris data
    pub iode: u8,
    /// Issue of clock data
    pub iodc: u16,
}

impl MsgEphemerisGPSDepF {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisGPSDepF, ::Error> {
        Ok(MsgEphemerisGPSDepF {
            sender_id: None,
            common: EphemerisCommonContentDepB::parse(_buf)?,
            tgd: _buf.read_f64::<LittleEndian>()?,
            c_rs: _buf.read_f64::<LittleEndian>()?,
            c_rc: _buf.read_f64::<LittleEndian>()?,
            c_uc: _buf.read_f64::<LittleEndian>()?,
            c_us: _buf.read_f64::<LittleEndian>()?,
            c_ic: _buf.read_f64::<LittleEndian>()?,
            c_is: _buf.read_f64::<LittleEndian>()?,
            dn: _buf.read_f64::<LittleEndian>()?,
            m0: _buf.read_f64::<LittleEndian>()?,
            ecc: _buf.read_f64::<LittleEndian>()?,
            sqrta: _buf.read_f64::<LittleEndian>()?,
            omega0: _buf.read_f64::<LittleEndian>()?,
            omegadot: _buf.read_f64::<LittleEndian>()?,
            w: _buf.read_f64::<LittleEndian>()?,
            inc: _buf.read_f64::<LittleEndian>()?,
            inc_dot: _buf.read_f64::<LittleEndian>()?,
            af0: _buf.read_f64::<LittleEndian>()?,
            af1: _buf.read_f64::<LittleEndian>()?,
            af2: _buf.read_f64::<LittleEndian>()?,
            toc: GPSTimeSec::parse(_buf)?,
            iode: _buf.read_u8()?,
            iodc: _buf.read_u16::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisGPSDepF {
    const MSG_ID: u16 = 134;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris for GLO
///
/// The ephemeris message returns a set of satellite orbit
/// parameters that is used to calculate GLO satellite position,
/// velocity, and clock offset. Please see the GLO ICD 5.1 "Table 4.5
/// Characteristics of words of immediate information (ephemeris parameters)"
/// for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisGloDepC {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContentDepB,
    /// Relative deviation of predicted carrier frequency from nominal
    pub gamma: f64,
    /// Correction to the SV time
    pub tau: f64,
    /// Equipment delay between L1 and L2
    pub d_tau: f64,
    /// Position of the SV at tb in PZ-90.02 coordinates system
    pub pos: Vec<f64>,
    /// Velocity vector of the SV at tb in PZ-90.02 coordinates system
    pub vel: Vec<f64>,
    /// Acceleration vector of the SV at tb in PZ-90.02 coordinates sys
    pub acc: Vec<f64>,
    /// Frequency slot. FCN+8 (that is [1..14]). 0 or 0xFF for invalid
    pub fcn: u8,
}

impl MsgEphemerisGloDepC {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisGloDepC, ::Error> {
        Ok(MsgEphemerisGloDepC {
            sender_id: None,
            common: EphemerisCommonContentDepB::parse(_buf)?,
            gamma: _buf.read_f64::<LittleEndian>()?,
            tau: _buf.read_f64::<LittleEndian>()?,
            d_tau: _buf.read_f64::<LittleEndian>()?,
            pos: ::parser::read_double_array_limit(_buf, 3)?,
            vel: ::parser::read_double_array_limit(_buf, 3)?,
            acc: ::parser::read_double_array_limit(_buf, 3)?,
            fcn: _buf.read_u8()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisGloDepC {
    const MSG_ID: u16 = 135;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Deprecated
///
/// This observation message has been deprecated in favor of
/// ephemeris message using floats for size reduction.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisGloDepD {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContentDepB,
    /// Relative deviation of predicted carrier frequency from nominal
    pub gamma: f64,
    /// Correction to the SV time
    pub tau: f64,
    /// Equipment delay between L1 and L2
    pub d_tau: f64,
    /// Position of the SV at tb in PZ-90.02 coordinates system
    pub pos: Vec<f64>,
    /// Velocity vector of the SV at tb in PZ-90.02 coordinates system
    pub vel: Vec<f64>,
    /// Acceleration vector of the SV at tb in PZ-90.02 coordinates sys
    pub acc: Vec<f64>,
    /// Frequency slot. FCN+8 (that is [1..14]). 0 or 0xFF for invalid
    pub fcn: u8,
    /// Issue of ephemeris data
    pub iod: u8,
}

impl MsgEphemerisGloDepD {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisGloDepD, ::Error> {
        Ok(MsgEphemerisGloDepD {
            sender_id: None,
            common: EphemerisCommonContentDepB::parse(_buf)?,
            gamma: _buf.read_f64::<LittleEndian>()?,
            tau: _buf.read_f64::<LittleEndian>()?,
            d_tau: _buf.read_f64::<LittleEndian>()?,
            pos: ::parser::read_double_array_limit(_buf, 3)?,
            vel: ::parser::read_double_array_limit(_buf, 3)?,
            acc: ::parser::read_double_array_limit(_buf, 3)?,
            fcn: _buf.read_u8()?,
            iod: _buf.read_u8()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisGloDepD {
    const MSG_ID: u16 = 136;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris for BDS
///
/// The ephemeris message returns a set of satellite orbit
/// parameters that is used to calculate BDS satellite position,
/// velocity, and clock offset. Please see the BeiDou Navigation
/// Satellite System SIS-ICD Version 2.1, Table 5-9 for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisBds {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContent,
    /// Group delay differential for B1
    pub tgd1: f32,
    /// Group delay differential for B2
    pub tgd2: f32,
    /// Amplitude of the sine harmonic correction term to the orbit radius
    pub c_rs: f32,
    /// Amplitude of the cosine harmonic correction term to the orbit radius
    pub c_rc: f32,
    /// Amplitude of the cosine harmonic correction term to the argument of
    /// latitude
    pub c_uc: f32,
    /// Amplitude of the sine harmonic correction term to the argument of
    /// latitude
    pub c_us: f32,
    /// Amplitude of the cosine harmonic correction term to the angle of
    /// inclination
    pub c_ic: f32,
    /// Amplitude of the sine harmonic correction term to the angle of
    /// inclination
    pub c_is: f32,
    /// Mean motion difference
    pub dn: f64,
    /// Mean anomaly at reference time
    pub m0: f64,
    /// Eccentricity of satellite orbit
    pub ecc: f64,
    /// Square root of the semi-major axis of orbit
    pub sqrta: f64,
    /// Longitude of ascending node of orbit plane at weekly epoch
    pub omega0: f64,
    /// Rate of right ascension
    pub omegadot: f64,
    /// Argument of perigee
    pub w: f64,
    /// Inclination
    pub inc: f64,
    /// Inclination first derivative
    pub inc_dot: f64,
    /// Polynomial clock correction coefficient (clock bias)
    pub af0: f64,
    /// Polynomial clock correction coefficient (clock drift)
    pub af1: f32,
    /// Polynomial clock correction coefficient (rate of clock drift)
    pub af2: f32,
    /// Clock reference
    pub toc: GPSTimeSec,
    /// Issue of ephemeris data
    pub iode: u8,
    /// Issue of clock data
    pub iodc: u16,
}

impl MsgEphemerisBds {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisBds, ::Error> {
        Ok(MsgEphemerisBds {
            sender_id: None,
            common: EphemerisCommonContent::parse(_buf)?,
            tgd1: _buf.read_f32::<LittleEndian>()?,
            tgd2: _buf.read_f32::<LittleEndian>()?,
            c_rs: _buf.read_f32::<LittleEndian>()?,
            c_rc: _buf.read_f32::<LittleEndian>()?,
            c_uc: _buf.read_f32::<LittleEndian>()?,
            c_us: _buf.read_f32::<LittleEndian>()?,
            c_ic: _buf.read_f32::<LittleEndian>()?,
            c_is: _buf.read_f32::<LittleEndian>()?,
            dn: _buf.read_f64::<LittleEndian>()?,
            m0: _buf.read_f64::<LittleEndian>()?,
            ecc: _buf.read_f64::<LittleEndian>()?,
            sqrta: _buf.read_f64::<LittleEndian>()?,
            omega0: _buf.read_f64::<LittleEndian>()?,
            omegadot: _buf.read_f64::<LittleEndian>()?,
            w: _buf.read_f64::<LittleEndian>()?,
            inc: _buf.read_f64::<LittleEndian>()?,
            inc_dot: _buf.read_f64::<LittleEndian>()?,
            af0: _buf.read_f64::<LittleEndian>()?,
            af1: _buf.read_f32::<LittleEndian>()?,
            af2: _buf.read_f32::<LittleEndian>()?,
            toc: GPSTimeSec::parse(_buf)?,
            iode: _buf.read_u8()?,
            iodc: _buf.read_u16::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisBds {
    const MSG_ID: u16 = 137;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris for GPS
///
/// The ephemeris message returns a set of satellite orbit
/// parameters that is used to calculate GPS satellite position,
/// velocity, and clock offset. Please see the Navstar GPS
/// Space Segment/Navigation user interfaces (ICD-GPS-200, Table
/// 20-III) for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisGPS {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContent,
    /// Group delay differential between L1 and L2
    pub tgd: f32,
    /// Amplitude of the sine harmonic correction term to the orbit radius
    pub c_rs: f32,
    /// Amplitude of the cosine harmonic correction term to the orbit radius
    pub c_rc: f32,
    /// Amplitude of the cosine harmonic correction term to the argument of
    /// latitude
    pub c_uc: f32,
    /// Amplitude of the sine harmonic correction term to the argument of
    /// latitude
    pub c_us: f32,
    /// Amplitude of the cosine harmonic correction term to the angle of
    /// inclination
    pub c_ic: f32,
    /// Amplitude of the sine harmonic correction term to the angle of
    /// inclination
    pub c_is: f32,
    /// Mean motion difference
    pub dn: f64,
    /// Mean anomaly at reference time
    pub m0: f64,
    /// Eccentricity of satellite orbit
    pub ecc: f64,
    /// Square root of the semi-major axis of orbit
    pub sqrta: f64,
    /// Longitude of ascending node of orbit plane at weekly epoch
    pub omega0: f64,
    /// Rate of right ascension
    pub omegadot: f64,
    /// Argument of perigee
    pub w: f64,
    /// Inclination
    pub inc: f64,
    /// Inclination first derivative
    pub inc_dot: f64,
    /// Polynomial clock correction coefficient (clock bias)
    pub af0: f32,
    /// Polynomial clock correction coefficient (clock drift)
    pub af1: f32,
    /// Polynomial clock correction coefficient (rate of clock drift)
    pub af2: f32,
    /// Clock reference
    pub toc: GPSTimeSec,
    /// Issue of ephemeris data
    pub iode: u8,
    /// Issue of clock data
    pub iodc: u16,
}

impl MsgEphemerisGPS {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisGPS, ::Error> {
        Ok(MsgEphemerisGPS {
            sender_id: None,
            common: EphemerisCommonContent::parse(_buf)?,
            tgd: _buf.read_f32::<LittleEndian>()?,
            c_rs: _buf.read_f32::<LittleEndian>()?,
            c_rc: _buf.read_f32::<LittleEndian>()?,
            c_uc: _buf.read_f32::<LittleEndian>()?,
            c_us: _buf.read_f32::<LittleEndian>()?,
            c_ic: _buf.read_f32::<LittleEndian>()?,
            c_is: _buf.read_f32::<LittleEndian>()?,
            dn: _buf.read_f64::<LittleEndian>()?,
            m0: _buf.read_f64::<LittleEndian>()?,
            ecc: _buf.read_f64::<LittleEndian>()?,
            sqrta: _buf.read_f64::<LittleEndian>()?,
            omega0: _buf.read_f64::<LittleEndian>()?,
            omegadot: _buf.read_f64::<LittleEndian>()?,
            w: _buf.read_f64::<LittleEndian>()?,
            inc: _buf.read_f64::<LittleEndian>()?,
            inc_dot: _buf.read_f64::<LittleEndian>()?,
            af0: _buf.read_f32::<LittleEndian>()?,
            af1: _buf.read_f32::<LittleEndian>()?,
            af2: _buf.read_f32::<LittleEndian>()?,
            toc: GPSTimeSec::parse(_buf)?,
            iode: _buf.read_u8()?,
            iodc: _buf.read_u16::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisGPS {
    const MSG_ID: u16 = 138;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris for GLO
///
/// The ephemeris message returns a set of satellite orbit
/// parameters that is used to calculate GLO satellite position,
/// velocity, and clock offset. Please see the GLO ICD 5.1 "Table 4.5
/// Characteristics of words of immediate information (ephemeris parameters)"
/// for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisGlo {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContent,
    /// Relative deviation of predicted carrier frequency from nominal
    pub gamma: f32,
    /// Correction to the SV time
    pub tau: f32,
    /// Equipment delay between L1 and L2
    pub d_tau: f32,
    /// Position of the SV at tb in PZ-90.02 coordinates system
    pub pos: Vec<f64>,
    /// Velocity vector of the SV at tb in PZ-90.02 coordinates system
    pub vel: Vec<f64>,
    /// Acceleration vector of the SV at tb in PZ-90.02 coordinates sys
    pub acc: Vec<f32>,
    /// Frequency slot. FCN+8 (that is [1..14]). 0 or 0xFF for invalid
    pub fcn: u8,
    /// Issue of ephemeris data
    pub iod: u8,
}

impl MsgEphemerisGlo {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisGlo, ::Error> {
        Ok(MsgEphemerisGlo {
            sender_id: None,
            common: EphemerisCommonContent::parse(_buf)?,
            gamma: _buf.read_f32::<LittleEndian>()?,
            tau: _buf.read_f32::<LittleEndian>()?,
            d_tau: _buf.read_f32::<LittleEndian>()?,
            pos: ::parser::read_double_array_limit(_buf, 3)?,
            vel: ::parser::read_double_array_limit(_buf, 3)?,
            acc: ::parser::read_float_array_limit(_buf, 3)?,
            fcn: _buf.read_u8()?,
            iod: _buf.read_u8()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisGlo {
    const MSG_ID: u16 = 139;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisSbas {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContent,
    /// Position of the GEO at time toe
    pub pos: Vec<f64>,
    /// Velocity of the GEO at time toe
    pub vel: Vec<f32>,
    /// Acceleration of the GEO at time toe
    pub acc: Vec<f32>,
    /// Time offset of the GEO clock w.r.t. SBAS Network Time
    pub a_gf0: f32,
    /// Drift of the GEO clock w.r.t. SBAS Network Time
    pub a_gf1: f32,
}

impl MsgEphemerisSbas {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisSbas, ::Error> {
        Ok(MsgEphemerisSbas {
            sender_id: None,
            common: EphemerisCommonContent::parse(_buf)?,
            pos: ::parser::read_double_array_limit(_buf, 3)?,
            vel: ::parser::read_float_array_limit(_buf, 3)?,
            acc: ::parser::read_float_array_limit(_buf, 3)?,
            a_gf0: _buf.read_f32::<LittleEndian>()?,
            a_gf1: _buf.read_f32::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisSbas {
    const MSG_ID: u16 = 140;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris for Galileo
///
/// The ephemeris message returns a set of satellite orbit
/// parameters that is used to calculate Galileo satellite position,
/// velocity, and clock offset. Please see the Signal In Space ICD
/// OS SIS ICD, Issue 1.3, December 2016 for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisGal {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContent,
    /// E1-E5a Broadcast Group Delay
    pub bgd_e1e5a: f32,
    /// E1-E5b Broadcast Group Delay
    pub bgd_e1e5b: f32,
    /// Amplitude of the sine harmonic correction term to the orbit radius
    pub c_rs: f32,
    /// Amplitude of the cosine harmonic correction term to the orbit radius
    pub c_rc: f32,
    /// Amplitude of the cosine harmonic correction term to the argument of
    /// latitude
    pub c_uc: f32,
    /// Amplitude of the sine harmonic correction term to the argument of
    /// latitude
    pub c_us: f32,
    /// Amplitude of the cosine harmonic correction term to the angle of
    /// inclination
    pub c_ic: f32,
    /// Amplitude of the sine harmonic correction term to the angle of
    /// inclination
    pub c_is: f32,
    /// Mean motion difference
    pub dn: f64,
    /// Mean anomaly at reference time
    pub m0: f64,
    /// Eccentricity of satellite orbit
    pub ecc: f64,
    /// Square root of the semi-major axis of orbit
    pub sqrta: f64,
    /// Longitude of ascending node of orbit plane at weekly epoch
    pub omega0: f64,
    /// Rate of right ascension
    pub omegadot: f64,
    /// Argument of perigee
    pub w: f64,
    /// Inclination
    pub inc: f64,
    /// Inclination first derivative
    pub inc_dot: f64,
    /// Polynomial clock correction coefficient (clock bias)
    pub af0: f64,
    /// Polynomial clock correction coefficient (clock drift)
    pub af1: f64,
    /// Polynomial clock correction coefficient (rate of clock drift)
    pub af2: f32,
    /// Clock reference
    pub toc: GPSTimeSec,
    /// Issue of ephemeris data
    pub iode: u16,
    /// Issue of clock data
    pub iodc: u16,
    /// 0=I/NAV, 1=F/NAV, ...
    pub source: u8,
}

impl MsgEphemerisGal {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisGal, ::Error> {
        Ok(MsgEphemerisGal {
            sender_id: None,
            common: EphemerisCommonContent::parse(_buf)?,
            bgd_e1e5a: _buf.read_f32::<LittleEndian>()?,
            bgd_e1e5b: _buf.read_f32::<LittleEndian>()?,
            c_rs: _buf.read_f32::<LittleEndian>()?,
            c_rc: _buf.read_f32::<LittleEndian>()?,
            c_uc: _buf.read_f32::<LittleEndian>()?,
            c_us: _buf.read_f32::<LittleEndian>()?,
            c_ic: _buf.read_f32::<LittleEndian>()?,
            c_is: _buf.read_f32::<LittleEndian>()?,
            dn: _buf.read_f64::<LittleEndian>()?,
            m0: _buf.read_f64::<LittleEndian>()?,
            ecc: _buf.read_f64::<LittleEndian>()?,
            sqrta: _buf.read_f64::<LittleEndian>()?,
            omega0: _buf.read_f64::<LittleEndian>()?,
            omegadot: _buf.read_f64::<LittleEndian>()?,
            w: _buf.read_f64::<LittleEndian>()?,
            inc: _buf.read_f64::<LittleEndian>()?,
            inc_dot: _buf.read_f64::<LittleEndian>()?,
            af0: _buf.read_f64::<LittleEndian>()?,
            af1: _buf.read_f64::<LittleEndian>()?,
            af2: _buf.read_f32::<LittleEndian>()?,
            toc: GPSTimeSec::parse(_buf)?,
            iode: _buf.read_u16::<LittleEndian>()?,
            iodc: _buf.read_u16::<LittleEndian>()?,
            source: _buf.read_u8()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisGal {
    const MSG_ID: u16 = 141;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite broadcast ephemeris for QZSS
///
/// The ephemeris message returns a set of satellite orbit
/// parameters that is used to calculate QZSS satellite position,
/// velocity, and clock offset.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisQzss {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContent,
    /// Group delay differential between L1 and L2
    pub tgd: f32,
    /// Amplitude of the sine harmonic correction term to the orbit radius
    pub c_rs: f32,
    /// Amplitude of the cosine harmonic correction term to the orbit radius
    pub c_rc: f32,
    /// Amplitude of the cosine harmonic correction term to the argument of
    /// latitude
    pub c_uc: f32,
    /// Amplitude of the sine harmonic correction term to the argument of
    /// latitude
    pub c_us: f32,
    /// Amplitude of the cosine harmonic correction term to the angle of
    /// inclination
    pub c_ic: f32,
    /// Amplitude of the sine harmonic correction term to the angle of
    /// inclination
    pub c_is: f32,
    /// Mean motion difference
    pub dn: f64,
    /// Mean anomaly at reference time
    pub m0: f64,
    /// Eccentricity of satellite orbit
    pub ecc: f64,
    /// Square root of the semi-major axis of orbit
    pub sqrta: f64,
    /// Longitude of ascending node of orbit plane at weekly epoch
    pub omega0: f64,
    /// Rate of right ascension
    pub omegadot: f64,
    /// Argument of perigee
    pub w: f64,
    /// Inclination
    pub inc: f64,
    /// Inclination first derivative
    pub inc_dot: f64,
    /// Polynomial clock correction coefficient (clock bias)
    pub af0: f32,
    /// Polynomial clock correction coefficient (clock drift)
    pub af1: f32,
    /// Polynomial clock correction coefficient (rate of clock drift)
    pub af2: f32,
    /// Clock reference
    pub toc: GPSTimeSec,
    /// Issue of ephemeris data
    pub iode: u8,
    /// Issue of clock data
    pub iodc: u16,
}

impl MsgEphemerisQzss {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisQzss, ::Error> {
        Ok(MsgEphemerisQzss {
            sender_id: None,
            common: EphemerisCommonContent::parse(_buf)?,
            tgd: _buf.read_f32::<LittleEndian>()?,
            c_rs: _buf.read_f32::<LittleEndian>()?,
            c_rc: _buf.read_f32::<LittleEndian>()?,
            c_uc: _buf.read_f32::<LittleEndian>()?,
            c_us: _buf.read_f32::<LittleEndian>()?,
            c_ic: _buf.read_f32::<LittleEndian>()?,
            c_is: _buf.read_f32::<LittleEndian>()?,
            dn: _buf.read_f64::<LittleEndian>()?,
            m0: _buf.read_f64::<LittleEndian>()?,
            ecc: _buf.read_f64::<LittleEndian>()?,
            sqrta: _buf.read_f64::<LittleEndian>()?,
            omega0: _buf.read_f64::<LittleEndian>()?,
            omegadot: _buf.read_f64::<LittleEndian>()?,
            w: _buf.read_f64::<LittleEndian>()?,
            inc: _buf.read_f64::<LittleEndian>()?,
            inc_dot: _buf.read_f64::<LittleEndian>()?,
            af0: _buf.read_f32::<LittleEndian>()?,
            af1: _buf.read_f32::<LittleEndian>()?,
            af2: _buf.read_f32::<LittleEndian>()?,
            toc: GPSTimeSec::parse(_buf)?,
            iode: _buf.read_u8()?,
            iodc: _buf.read_u16::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisQzss {
    const MSG_ID: u16 = 142;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Iono corrections
///
/// The ionospheric parameters which allow the "L1 only" or "L2 only" user to
/// utilize the ionospheric model for computation of the ionospheric delay.
/// Please see ICD-GPS-200 (Chapter 20.3.3.5.1.7) for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgIono {
    pub sender_id: Option<u16>,
    /// Navigation Message Correction Table Valitidy Time
    pub t_nmct: GPSTimeSec,
    pub a0: f64,
    pub a1: f64,
    pub a2: f64,
    pub a3: f64,
    pub b0: f64,
    pub b1: f64,
    pub b2: f64,
    pub b3: f64,
}

impl MsgIono {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgIono, ::Error> {
        Ok(MsgIono {
            sender_id: None,
            t_nmct: GPSTimeSec::parse(_buf)?,
            a0: _buf.read_f64::<LittleEndian>()?,
            a1: _buf.read_f64::<LittleEndian>()?,
            a2: _buf.read_f64::<LittleEndian>()?,
            a3: _buf.read_f64::<LittleEndian>()?,
            b0: _buf.read_f64::<LittleEndian>()?,
            b1: _buf.read_f64::<LittleEndian>()?,
            b2: _buf.read_f64::<LittleEndian>()?,
            b3: _buf.read_f64::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgIono {
    const MSG_ID: u16 = 144;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// L2C capability mask
///
/// Please see ICD-GPS-200 (Chapter 20.3.3.5.1.4) for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgSvConfigurationGPSDep {
    pub sender_id: Option<u16>,
    /// Navigation Message Correction Table Valitidy Time
    pub t_nmct: GPSTimeSec,
    /// L2C capability mask, SV32 bit being MSB, SV1 bit being LSB
    pub l2c_mask: u32,
}

impl MsgSvConfigurationGPSDep {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgSvConfigurationGPSDep, ::Error> {
        Ok(MsgSvConfigurationGPSDep {
            sender_id: None,
            t_nmct: GPSTimeSec::parse(_buf)?,
            l2c_mask: _buf.read_u32::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgSvConfigurationGPSDep {
    const MSG_ID: u16 = 145;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Group Delay
///
/// Please see ICD-GPS-200 (30.3.3.3.1.1) for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgGroupDelayDepA {
    pub sender_id: Option<u16>,
    /// Data Predict Time of Week
    pub t_op: GPSTimeDep,
    /// Satellite number
    pub prn: u8,
    /// bit-field indicating validity of the values, LSB indicating tgd validity
    /// etc. 1 = value is valid, 0 = value is not valid.
    pub valid: u8,
    pub tgd: i16,
    pub isc_l1ca: i16,
    pub isc_l2c: i16,
}

impl MsgGroupDelayDepA {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgGroupDelayDepA, ::Error> {
        Ok(MsgGroupDelayDepA {
            sender_id: None,
            t_op: GPSTimeDep::parse(_buf)?,
            prn: _buf.read_u8()?,
            valid: _buf.read_u8()?,
            tgd: _buf.read_i16::<LittleEndian>()?,
            isc_l1ca: _buf.read_i16::<LittleEndian>()?,
            isc_l2c: _buf.read_i16::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgGroupDelayDepA {
    const MSG_ID: u16 = 146;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Group Delay
///
/// Please see ICD-GPS-200 (30.3.3.3.1.1) for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgGroupDelayDepB {
    pub sender_id: Option<u16>,
    /// Data Predict Time of Week
    pub t_op: GPSTimeSec,
    /// GNSS signal identifier
    pub sid: GnssSignalDep,
    /// bit-field indicating validity of the values, LSB indicating tgd validity
    /// etc. 1 = value is valid, 0 = value is not valid.
    pub valid: u8,
    pub tgd: i16,
    pub isc_l1ca: i16,
    pub isc_l2c: i16,
}

impl MsgGroupDelayDepB {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgGroupDelayDepB, ::Error> {
        Ok(MsgGroupDelayDepB {
            sender_id: None,
            t_op: GPSTimeSec::parse(_buf)?,
            sid: GnssSignalDep::parse(_buf)?,
            valid: _buf.read_u8()?,
            tgd: _buf.read_i16::<LittleEndian>()?,
            isc_l1ca: _buf.read_i16::<LittleEndian>()?,
            isc_l2c: _buf.read_i16::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgGroupDelayDepB {
    const MSG_ID: u16 = 147;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Group Delay
///
/// Please see ICD-GPS-200 (30.3.3.3.1.1) for more details.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgGroupDelay {
    pub sender_id: Option<u16>,
    /// Data Predict Time of Week
    pub t_op: GPSTimeSec,
    /// GNSS signal identifier
    pub sid: GnssSignal,
    /// bit-field indicating validity of the values, LSB indicating tgd validity
    /// etc. 1 = value is valid, 0 = value is not valid.
    pub valid: u8,
    pub tgd: i16,
    pub isc_l1ca: i16,
    pub isc_l2c: i16,
}

impl MsgGroupDelay {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgGroupDelay, ::Error> {
        Ok(MsgGroupDelay {
            sender_id: None,
            t_op: GPSTimeSec::parse(_buf)?,
            sid: GnssSignal::parse(_buf)?,
            valid: _buf.read_u8()?,
            tgd: _buf.read_i16::<LittleEndian>()?,
            isc_l1ca: _buf.read_i16::<LittleEndian>()?,
            isc_l2c: _buf.read_i16::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgGroupDelay {
    const MSG_ID: u16 = 148;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Deprecated
///
/// This observation message has been deprecated in favor of
/// an ephemeris message with explicit source of NAV data.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgEphemerisGalDepA {
    pub sender_id: Option<u16>,
    /// Values common for all ephemeris types
    pub common: EphemerisCommonContent,
    /// E1-E5a Broadcast Group Delay
    pub bgd_e1e5a: f32,
    /// E1-E5b Broadcast Group Delay
    pub bgd_e1e5b: f32,
    /// Amplitude of the sine harmonic correction term to the orbit radius
    pub c_rs: f32,
    /// Amplitude of the cosine harmonic correction term to the orbit radius
    pub c_rc: f32,
    /// Amplitude of the cosine harmonic correction term to the argument of
    /// latitude
    pub c_uc: f32,
    /// Amplitude of the sine harmonic correction term to the argument of
    /// latitude
    pub c_us: f32,
    /// Amplitude of the cosine harmonic correction term to the angle of
    /// inclination
    pub c_ic: f32,
    /// Amplitude of the sine harmonic correction term to the angle of
    /// inclination
    pub c_is: f32,
    /// Mean motion difference
    pub dn: f64,
    /// Mean anomaly at reference time
    pub m0: f64,
    /// Eccentricity of satellite orbit
    pub ecc: f64,
    /// Square root of the semi-major axis of orbit
    pub sqrta: f64,
    /// Longitude of ascending node of orbit plane at weekly epoch
    pub omega0: f64,
    /// Rate of right ascension
    pub omegadot: f64,
    /// Argument of perigee
    pub w: f64,
    /// Inclination
    pub inc: f64,
    /// Inclination first derivative
    pub inc_dot: f64,
    /// Polynomial clock correction coefficient (clock bias)
    pub af0: f64,
    /// Polynomial clock correction coefficient (clock drift)
    pub af1: f64,
    /// Polynomial clock correction coefficient (rate of clock drift)
    pub af2: f32,
    /// Clock reference
    pub toc: GPSTimeSec,
    /// Issue of ephemeris data
    pub iode: u16,
    /// Issue of clock data
    pub iodc: u16,
}

impl MsgEphemerisGalDepA {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgEphemerisGalDepA, ::Error> {
        Ok(MsgEphemerisGalDepA {
            sender_id: None,
            common: EphemerisCommonContent::parse(_buf)?,
            bgd_e1e5a: _buf.read_f32::<LittleEndian>()?,
            bgd_e1e5b: _buf.read_f32::<LittleEndian>()?,
            c_rs: _buf.read_f32::<LittleEndian>()?,
            c_rc: _buf.read_f32::<LittleEndian>()?,
            c_uc: _buf.read_f32::<LittleEndian>()?,
            c_us: _buf.read_f32::<LittleEndian>()?,
            c_ic: _buf.read_f32::<LittleEndian>()?,
            c_is: _buf.read_f32::<LittleEndian>()?,
            dn: _buf.read_f64::<LittleEndian>()?,
            m0: _buf.read_f64::<LittleEndian>()?,
            ecc: _buf.read_f64::<LittleEndian>()?,
            sqrta: _buf.read_f64::<LittleEndian>()?,
            omega0: _buf.read_f64::<LittleEndian>()?,
            omegadot: _buf.read_f64::<LittleEndian>()?,
            w: _buf.read_f64::<LittleEndian>()?,
            inc: _buf.read_f64::<LittleEndian>()?,
            inc_dot: _buf.read_f64::<LittleEndian>()?,
            af0: _buf.read_f64::<LittleEndian>()?,
            af1: _buf.read_f64::<LittleEndian>()?,
            af2: _buf.read_f32::<LittleEndian>()?,
            toc: GPSTimeSec::parse(_buf)?,
            iode: _buf.read_u16::<LittleEndian>()?,
            iodc: _buf.read_u16::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgEphemerisGalDepA {
    const MSG_ID: u16 = 149;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgGnssCapb {
    pub sender_id: Option<u16>,
    /// Navigation Message Correction Table Validity Time
    pub t_nmct: GPSTimeSec,
    /// GNSS capabilities masks
    pub gc: GnssCapb,
}

impl MsgGnssCapb {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgGnssCapb, ::Error> {
        Ok(MsgGnssCapb {
            sender_id: None,
            t_nmct: GPSTimeSec::parse(_buf)?,
            gc: GnssCapb::parse(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgGnssCapb {
    const MSG_ID: u16 = 150;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Satellite azimuths and elevations
///
/// Azimuth and elevation angles of all the visible satellites
/// that the device does have ephemeris or almanac for.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgSvAzEl {
    pub sender_id: Option<u16>,
    /// Azimuth and elevation per satellite
    pub azel: Vec<SvAzEl>,
}

impl MsgSvAzEl {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgSvAzEl, ::Error> {
        Ok(MsgSvAzEl {
            sender_id: None,
            azel: SvAzEl::parse_array(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgSvAzEl {
    const MSG_ID: u16 = 151;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// OSR corrections
///
/// The OSR message contains network corrections in an observation-like format
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgOsr {
    pub sender_id: Option<u16>,
    /// Header of a GPS observation message
    pub header: ObservationHeader,
    /// Network correction for a satellite signal.
    pub obs: Vec<PackedOsrContent>,
}

impl MsgOsr {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgOsr, ::Error> {
        Ok(MsgOsr {
            sender_id: None,
            header: ObservationHeader::parse(_buf)?,
            obs: PackedOsrContent::parse_array(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgOsr {
    const MSG_ID: u16 = 1600;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}
