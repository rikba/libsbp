/*
 * Copyright (C) 2015-2018 Swift Navigation Inc.
 * Contact: Swift Navigation <dev@swiftnav.com>
 *
 * This source is subject to the license found in the file 'LICENSE' which must
 * be be distributed together with this source. All other rights reserved.
 *
 * THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
 * EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
 */

package com.swiftnav.sbp.navigation;

import java.math.BigInteger;

import com.swiftnav.sbp.SBPMessage;
import com.swiftnav.sbp.SBPBinaryException;
import com.swiftnav.sbp.SBPStruct;

import org.json.JSONObject;
import org.json.JSONArray;


/** SBP class for message MSG_PROTECTION_LEVEL (0x0216).
 *
 * You can have MSG_PROTECTION_LEVEL inherent its fields directly from
 * an inherited SBP object, or construct it inline using a dict of its
 * fields.
 *
 * This message reports the baseline heading pointing from the base station
 * to the rover relative to True North. The full GPS time is given by the
 * preceding MSG_GPS_TIME with the matching time-of-week (tow). */

public class MsgProtectionLevel extends SBPMessage {
    public static final int TYPE = 0x0216;

    
    /** GPS Time of Week */
    public long tow;
    
    /** Vertical protection level */
    public long vpl;
    
    /** Horizontal protection level */
    public long hpl;
    
    /** ECEF X coordinate */
    public double x;
    
    /** ECEF Y coordinate */
    public double y;
    
    /** ECEF Z coordinate */
    public double z;
    
    /** Status flags */
    public int flags;
    

    public MsgProtectionLevel (int sender) { super(sender, TYPE); }
    public MsgProtectionLevel () { super(TYPE); }
    public MsgProtectionLevel (SBPMessage msg) throws SBPBinaryException {
        super(msg);
        assert msg.type != TYPE;
    }

    @Override
    protected void parse(Parser parser) throws SBPBinaryException {
        /* Parse fields from binary */
        tow = parser.getU32();
        vpl = parser.getU32();
        hpl = parser.getU32();
        x = parser.getDouble();
        y = parser.getDouble();
        z = parser.getDouble();
        flags = parser.getU8();
    }

    @Override
    protected void build(Builder builder) {
        builder.putU32(tow);
        builder.putU32(vpl);
        builder.putU32(hpl);
        builder.putDouble(x);
        builder.putDouble(y);
        builder.putDouble(z);
        builder.putU8(flags);
    }

    @Override
    public JSONObject toJSON() {
        JSONObject obj = super.toJSON();
        obj.put("tow", tow);
        obj.put("vpl", vpl);
        obj.put("hpl", hpl);
        obj.put("x", x);
        obj.put("y", y);
        obj.put("z", z);
        obj.put("flags", flags);
        return obj;
    }
}