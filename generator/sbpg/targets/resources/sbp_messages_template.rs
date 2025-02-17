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
// Automatically generated from yaml/(((filepath)))
// with generate.py. Please do not hand edit!
//****************************************************************************/

//! (((description | replace("\n", "\n//! "))))

extern crate byteorder;
#[allow(unused_imports)]
use self::byteorder::{LittleEndian,ReadBytesExt};

((*- for i in includes *))
use super::(((i)))::*;
((*- endfor *))

((* for m in msgs *))
((*- if m.desc *))
/// (((m.short_desc)))
///
(((m.desc|commentify)))
///
((*- endif *))
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct (((m.identifier|camel_case))) {
    ((*- if m.sbp_id *))
    pub sender_id: Option<u16>,
    ((*- endif *))
    ((*- for f in m.fields *))
    ((*- if f.desc *))
    /// (((f.desc | replace("\n", " ") | wordwrap(width=72, wrapstring="\n    /// "))))
    ((*- endif *))
    pub (((f.identifier))): (((f|type_map))),
    ((*- endfor *))
}

impl (((m.identifier|camel_case))) {
    pub fn parse(_buf: &mut &[u8]) -> Result<(((m.identifier|camel_case))), ::Error> {
        Ok( (((m.identifier|camel_case))){
            ((*- if m.sbp_id *))
            sender_id: None,
            ((*- endif *))
            ((*- for f in m.fields *))
            (((f.identifier))): (((f|parse_type)))?,
            ((*- endfor *))
        } )
    }

    ((*- if not m.sbp_id *))
    pub fn parse_array(buf: &mut &[u8]) -> Result<Vec<(((m.identifier|camel_case)))>, ::Error> {
        let mut v = Vec::new();
        while buf.len() > 0 {
            v.push( (((m.identifier|camel_case)))::parse(buf)? );
        }
        Ok(v)
    }

    pub fn parse_array_limit(buf: &mut &[u8], n: usize) -> Result<Vec<(((m.identifier|camel_case)))>, ::Error> {
        let mut v = Vec::new();
        for _ in 0..n {
            v.push( (((m.identifier|camel_case)))::parse(buf)? );
        }
        Ok(v)
    }
    ((*- endif *))
}

((*- if m.sbp_id *))
impl super::SBPMessage for (((m.identifier|camel_case))) {
    const MSG_ID: u16 = (((m.sbp_id)));

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}
((*- endif *))

((* endfor *))
