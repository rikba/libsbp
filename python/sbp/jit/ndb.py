#!/usr/bin/env python
# Copyright (C) 2015-2018 Swift Navigation Inc.
# Contact: Swift Navigation <dev@swiftnav.com>
#
# This source is subject to the license found in the file 'LICENSE' which must
# be be distributed together with this source. All other rights reserved.
#
# THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
# EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
# WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.


"""
Messages for logging NDB events.

"""

import json

from sbp.jit.msg import SBP, SENDER_ID
from sbp.jit.msg import get_u8, get_u16, get_u32, get_u64
from sbp.jit.msg import get_s8, get_s16, get_s32, get_s64
from sbp.jit.msg import get_f32, get_f64
from sbp.jit.msg import get_string, get_fixed_string
from sbp.jit.msg import get_array, get_fixed_array
from sbp.jit.gnss import *

# Automatically generated from piksi/yaml/swiftnav/sbp/ndb.yaml with generate.py.
# Please do not hand edit!
SBP_MSG_NDB_EVENT = 0x0400
class MsgNdbEvent(SBP):
  """SBP class for message MSG_NDB_EVENT (0x0400).

  You can have MSG_NDB_EVENT inherit its fields directly
  from an inherited SBP object, or construct it inline using a dict
  of its fields.

  
  This message is sent out when an object is stored into NDB. If needed
message could also be sent out when fetching an object from NDB.


  """
  __slots__ = ['recv_time',
               'event',
               'object_type',
               'result',
               'data_source',
               'object_sid',
               'src_sid',
               'original_sender',
               ]
  @classmethod
  def parse_members(cls, buf, offset, length):
    o_0 = offset
    o_1, (__recv_time, offset, length) = offset, get_u64(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__event, offset, length) = offset, get_u8(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__object_type, offset, length) = offset, get_u8(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__result, offset, length) = offset, get_u8(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__data_source, offset, length) = offset, get_u8(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__object_sid, offset, length) = offset, GnssSignal.parse_members(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__src_sid, offset, length) = offset, GnssSignal.parse_members(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    o_1, (__original_sender, offset, length) = offset, get_u16(buf, offset, length)
    if o_1 == offset:
      return {}, o_0, length
    return {
      'recv_time' : __recv_time,
      'event' : __event,
      'object_type' : __object_type,
      'result' : __result,
      'data_source' : __data_source,
      'object_sid' : __object_sid,
      'src_sid' : __src_sid,
      'original_sender' : __original_sender,
    }, offset, length

  def _unpack_members(self, buf, offset, length):
    res, off, length = self.parse_members(buf, offset, length)
    if off == offset:
      return {}, offset, length
    self.recv_time = res['recv_time']
    self.event = res['event']
    self.object_type = res['object_type']
    self.result = res['result']
    self.data_source = res['data_source']
    self.object_sid = res['object_sid']
    self.src_sid = res['src_sid']
    self.original_sender = res['original_sender']
    return res, off, length
  

msg_classes = {
  0x0400: MsgNdbEvent,
}