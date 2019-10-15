#!/usr/bin/env python
# Copyright (C) 2015 Swift Navigation Inc.
# Contact: Bhaskar Mookerji <mookerji@swiftnav.com>
#
# This source is subject to the license found in the file 'LICENSE' which must
# be be distributed together with this source. All other rights reserved.
#
# THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
# EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
# WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.

"""Generator for Protocol Buffers target.

This module consumes the YAML spec and generates some message class
files.

"""

import os
import copy
from sbpg.targets.templating import *
from sbpg.utils import markdown_links

MESSAGES_TEMPLATE_NAME = 'message_template.ros.j2'

TYPE_MAP = {
  'u8': 'uint8',
  'u16': 'uint16',
  'u32': 'uint32',
  'u64': 'uint64',
  's8': 'int8',
  's16': 'int16',
  's32': 'int32',
  's64': 'int64',
  'float': 'float32',
  'double': 'float64',
  'string': 'string',
}

def to_comment(value):
  """
  Builds a comment.
  """
  if value is None:
    return ''
  if len(value.split('\n')) == 1:
    return "# " + value
  else:
    return '\n'.join(['# ' + l for l in value.split('\n')[:-1]])

def to_unit(value):
    return to_comment('[' + str(value) + ']')

def remove_prefix(s):
    prefix = "Msg"
    if s.startswith(prefix):
        return s[len(prefix):]
    else:
        return s

def to_identifier(s):
  """
  Convert snake_case to camel_case.
  """
  if s.startswith('GPS'):
      s = 'Gps' + s[3:]
  s = ''.join([i.capitalize() for i in s.split('_')]) if '_' in s else s
  return remove_prefix(s)

def to_type(f, type_map=TYPE_MAP):
    name = f.type_id
    if name.startswith('GPS'):
        name = 'Gps' + name[3:]
    if type_map.get(name, None):
        return type_map.get(name, None)
    elif name == 'array':
        fill = f.options['fill'].value
        f_ = copy.copy(f)
        f_.type_id = fill
        return "%s[]" % to_type(f_)
    return name

def to_title(s):
    return s.title()

def is_deprecated(definition):
    if "DEP" in definition.identifier.upper():
        return True
    if not definition.desc:
        return False
    return "legacy" in definition.desc

JENV.filters['ros_to_identifier'] = to_identifier
JENV.filters['ros_to_type'] = to_type
JENV.filters['ros_to_comment'] = to_comment
JENV.filters['ros_to_unit'] = to_unit
JENV.filters['ros_to_title'] = to_title

def render_source(output_dir, package_spec):
    """
    Render and output to a directory given a package specification.
    """
    path, name = package_spec.filepath
    # Create a new ROS message for each SBP message.
    msg_output_dir = output_dir + '/msg'
    if not os.path.exists(msg_output_dir):
        os.mkdir(msg_output_dir)
    ros_template = JENV.get_template(MESSAGES_TEMPLATE_NAME)
    for definition in package_spec.definitions:
        if is_deprecated(definition):
            continue
        destination_filename = '%s/%s.msg' % (msg_output_dir, to_identifier(definition.identifier))
        with open(destination_filename, 'w') as f:
            f.write(ros_template.render(
                name=name,
                message=definition
            ))
