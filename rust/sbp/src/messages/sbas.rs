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
// Automatically generated from yaml/swiftnav/sbp/sbas.yaml
// with generate.py. Please do not hand edit!
//****************************************************************************/
//! SBAS data

extern crate byteorder;
#[allow(unused_imports)]
use self::byteorder::{LittleEndian, ReadBytesExt};
use super::gnss::*;

/// Raw SBAS data
///
/// This message is sent once per second per SBAS satellite. ME checks the
/// parity of the data block and sends only blocks that pass the check.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgSbasRaw {
    pub sender_id: Option<u16>,
    /// GNSS signal identifier.
    pub sid: GnssSignal,
    /// GPS time-of-week at the start of the data block.
    pub tow: u32,
    /// SBAS message type (0-63)
    pub message_type: u8,
    /// Raw SBAS data field of 212 bits (last byte padded with zeros).
    pub data: Vec<u8>,
}

impl MsgSbasRaw {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgSbasRaw, ::Error> {
        Ok(MsgSbasRaw {
            sender_id: None,
            sid: GnssSignal::parse(_buf)?,
            tow: _buf.read_u32::<LittleEndian>()?,
            message_type: _buf.read_u8()?,
            data: ::parser::read_u8_array_limit(_buf, 27)?,
        })
    }
}
impl super::SBPMessage for MsgSbasRaw {
    const MSG_ID: u16 = 30583;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}
