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
// Automatically generated from yaml/swiftnav/sbp/settings.yaml
// with generate.py. Please do not hand edit!
//****************************************************************************/
//!
//! Messages for reading, writing, and discovering device settings. Settings
//! with a "string" field have multiple values in this field delimited with a
//! null character (the c style null terminator).  For instance, when querying
//! the 'firmware_version' setting in the 'system_info' section, the following
//! array of characters needs to be sent for the string field in
//! MSG_SETTINGS_READ: "system_info\0firmware_version\0", where the delimiting
//! null characters are specified with the escape sequence '\0' and all
//! quotation marks should be omitted.
//!
//!
//! In the message descriptions below, the generic strings SECTION_SETTING and
//! SETTING are used to refer to the two strings that comprise the identifier
//! of an individual setting.In firmware_version example above, SECTION_SETTING
//! is the 'system_info', and the SETTING portion is 'firmware_version'.
//!
//! See the "Software Settings Manual" on support.swiftnav.com for detailed
//! documentation about all settings and sections available for each Swift
//! firmware version. Settings manuals are available for each firmware version
//! at the following link: @@https://support.swiftnav.com/customer/en/portal/articles/2628580-piksi-multi-specifications#settings[Piksi Multi Specifications].
//! The latest settings document is also available at the following link:
//! @@http://swiftnav.com/latest/piksi-multi-settings[Latest settings document] .
//! See lastly @@https://github.com/swift-nav/piksi_tools/blob/master/piksi_tools/settings.py[settings.py] ,
//! the open source python command line utility for reading, writing, and
//! saving settings in the piksi_tools repository on github as a helpful
//! reference and example.
//!

extern crate byteorder;
#[allow(unused_imports)]
use self::byteorder::{LittleEndian, ReadBytesExt};

/// Write device configuration settings (host => device)
///
/// The setting message writes the device configuration for a particular
/// setting via A NULL-terminated and NULL-delimited string with contents
/// "SECTION_SETTING\0SETTING\0VALUE\0" where the '\0' escape sequence denotes
/// the NULL character and where quotation marks are omitted. A device will
/// only process to this message when it is received from sender ID 0x42.
/// An example string that could be sent to a device is
/// "solution\0soln_freq\010\0".
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgSettingsWrite {
    pub sender_id: Option<u16>,
    /// A NULL-terminated and NULL-delimited string with contents
    /// "SECTION_SETTING\0SETTING\0VALUE\0"
    pub setting: String,
}

impl MsgSettingsWrite {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgSettingsWrite, ::Error> {
        Ok(MsgSettingsWrite {
            sender_id: None,
            setting: ::parser::read_string(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgSettingsWrite {
    const MSG_ID: u16 = 160;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Save settings to flash (host => device)
///
/// The save settings message persists the device's current settings
/// configuration to its onboard flash memory file system.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgSettingsSave {
    pub sender_id: Option<u16>,
}

impl MsgSettingsSave {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgSettingsSave, ::Error> {
        Ok(MsgSettingsSave { sender_id: None })
    }
}
impl super::SBPMessage for MsgSettingsSave {
    const MSG_ID: u16 = 161;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Read setting by direct index (host => device)
///
/// The settings message for iterating through the settings
/// values. A device will respond to this message with a
/// "MSG_SETTINGS_READ_BY_INDEX_RESP".
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgSettingsReadByIndexReq {
    pub sender_id: Option<u16>,
    /// An index into the device settings, with values ranging from 0 to
    /// length(settings)
    pub index: u16,
}

impl MsgSettingsReadByIndexReq {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgSettingsReadByIndexReq, ::Error> {
        Ok(MsgSettingsReadByIndexReq {
            sender_id: None,
            index: _buf.read_u16::<LittleEndian>()?,
        })
    }
}
impl super::SBPMessage for MsgSettingsReadByIndexReq {
    const MSG_ID: u16 = 162;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Read device configuration settings (host => device)
///
/// The setting message that reads the device configuration. The string
/// field is a NULL-terminated and NULL-delimited string with contents
/// "SECTION_SETTING\0SETTING\0" where the '\0' escape sequence denotes the
/// NULL character and where quotation marks are omitted. An example
/// string that could be sent to a device is "solution\0soln_freq\0". A
/// device will only respond to this message when it is received from
/// sender ID 0x42. A device should respond with a MSG_SETTINGS_READ_RESP
/// message (msg_id 0x00A5).
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgSettingsReadReq {
    pub sender_id: Option<u16>,
    /// A NULL-terminated and NULL-delimited string with contents
    /// "SECTION_SETTING\0SETTING\0"
    pub setting: String,
}

impl MsgSettingsReadReq {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgSettingsReadReq, ::Error> {
        Ok(MsgSettingsReadReq {
            sender_id: None,
            setting: ::parser::read_string(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgSettingsReadReq {
    const MSG_ID: u16 = 164;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Read device configuration settings (host <= device)
///
/// The setting message wich which the device responds after a
/// MSG_SETTING_READ_REQ is sent to device. The string field is a
/// NULL-terminated and NULL-delimited string with contents
/// "SECTION_SETTING\0SETTING\0VALUE\0" where the '\0' escape sequence
/// denotes the NULL character and where quotation marks are omitted. An
/// example string that could be sent from device is
/// "solution\0soln_freq\010\0".
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgSettingsReadResp {
    pub sender_id: Option<u16>,
    /// A NULL-terminated and NULL-delimited string with contents
    /// "SECTION_SETTING\0SETTING\0VALUE\0"
    pub setting: String,
}

impl MsgSettingsReadResp {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgSettingsReadResp, ::Error> {
        Ok(MsgSettingsReadResp {
            sender_id: None,
            setting: ::parser::read_string(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgSettingsReadResp {
    const MSG_ID: u16 = 165;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Finished reading settings (host <= device)
///
/// The settings message for indicating end of the settings values.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgSettingsReadByIndexDone {
    pub sender_id: Option<u16>,
}

impl MsgSettingsReadByIndexDone {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgSettingsReadByIndexDone, ::Error> {
        Ok(MsgSettingsReadByIndexDone { sender_id: None })
    }
}
impl super::SBPMessage for MsgSettingsReadByIndexDone {
    const MSG_ID: u16 = 166;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Read setting by direct index (host <= device)
///
/// The settings message that reports the value of a setting at an index.
///
/// In the string field, it reports NULL-terminated and delimited string
/// with contents "SECTION_SETTING\0SETTING\0VALUE\0FORMAT_TYPE\0". where
/// the '\0' escape sequence denotes the NULL character and where quotation
/// marks are omitted. The FORMAT_TYPE field is optional and denotes
/// possible string values of the setting as a hint to the user. If
/// included, the format type portion of the string has the format
/// "enum:value1,value2,value3". An example string that could be sent from
/// the device is "simulator\0enabled\0True\0enum:True,False\0"
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgSettingsReadByIndexResp {
    pub sender_id: Option<u16>,
    /// An index into the device settings, with values ranging from 0 to
    /// length(settings)
    pub index: u16,
    /// A NULL-terminated and delimited string with contents
    /// "SECTION_SETTING\0SETTING\0VALUE\0FORMAT_TYPE\0"
    pub setting: String,
}

impl MsgSettingsReadByIndexResp {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgSettingsReadByIndexResp, ::Error> {
        Ok(MsgSettingsReadByIndexResp {
            sender_id: None,
            index: _buf.read_u16::<LittleEndian>()?,
            setting: ::parser::read_string(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgSettingsReadByIndexResp {
    const MSG_ID: u16 = 167;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Register setting and default value (device => host)
///
/// This message registers the presence and default value of a setting
/// with a settings daemon.  The host should reply with MSG_SETTINGS_WRITE
/// for this setting to set the initial value.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgSettingsRegister {
    pub sender_id: Option<u16>,
    /// A NULL-terminated and delimited string with contents
    /// "SECTION_SETTING\0SETTING\0VALUE".
    pub setting: String,
}

impl MsgSettingsRegister {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgSettingsRegister, ::Error> {
        Ok(MsgSettingsRegister {
            sender_id: None,
            setting: ::parser::read_string(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgSettingsRegister {
    const MSG_ID: u16 = 174;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Acknowledgement with status of MSG_SETTINGS_WRITE
///
/// Return the status of a write request with the new value of the
/// setting.  If the requested value is rejected, the current value
/// will be returned. The string field is a NULL-terminated and NULL-delimited
/// string with contents "SECTION_SETTING\0SETTING\0VALUE\0" where the '\0'
/// escape sequence denotes the NULL character and where quotation marks
/// are omitted. An example string that could be sent from device is
/// "solution\0soln_freq\010\0".
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgSettingsWriteResp {
    pub sender_id: Option<u16>,
    /// Write status
    pub status: u8,
    /// A NULL-terminated and delimited string with contents
    /// "SECTION_SETTING\0SETTING\0VALUE\0"
    pub setting: String,
}

impl MsgSettingsWriteResp {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgSettingsWriteResp, ::Error> {
        Ok(MsgSettingsWriteResp {
            sender_id: None,
            status: _buf.read_u8()?,
            setting: ::parser::read_string(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgSettingsWriteResp {
    const MSG_ID: u16 = 175;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}

/// Register setting and default value (device <= host)
///
/// This message responds to setting registration with the effective value.
/// The effective value shall differ from the given default value if setting
/// was already registered or is available in the permanent setting storage
/// and had a different value.
///
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct MsgSettingsRegisterResp {
    pub sender_id: Option<u16>,
    /// Register status
    pub status: u8,
    /// A NULL-terminated and delimited string with contents
    /// "SECTION_SETTING\0SETTING\0VALUE". The meaning of value is defined
    /// according to the status field.
    pub setting: String,
}

impl MsgSettingsRegisterResp {
    pub fn parse(_buf: &mut &[u8]) -> Result<MsgSettingsRegisterResp, ::Error> {
        Ok(MsgSettingsRegisterResp {
            sender_id: None,
            status: _buf.read_u8()?,
            setting: ::parser::read_string(_buf)?,
        })
    }
}
impl super::SBPMessage for MsgSettingsRegisterResp {
    const MSG_ID: u16 = 431;

    fn get_sender_id(&self) -> Option<u16> {
        self.sender_id
    }

    fn set_sender_id(&mut self, new_id: u16) {
        self.sender_id = Some(new_id);
    }
}
