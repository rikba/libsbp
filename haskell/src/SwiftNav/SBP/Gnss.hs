{-# OPTIONS_GHC -fno-warn-unused-imports #-}
{-# LANGUAGE NoImplicitPrelude           #-}
{-# LANGUAGE TemplateHaskell             #-}
{-# LANGUAGE RecordWildCards             #-}

-- |
-- Module:      SwiftNav.SBP.Gnss
-- Copyright:   Copyright (C) 2015-2018 Swift Navigation, Inc.
-- License:     LGPL-3
-- Maintainer:  Swift Navigation <dev@swiftnav.com>
-- Stability:   experimental
-- Portability: portable
--
-- Various structs shared between modules

module SwiftNav.SBP.Gnss
  ( module SwiftNav.SBP.Gnss
  ) where

import BasicPrelude
import Control.Lens
import Control.Monad.Loops
import Data.Binary
import Data.Binary.Get
import Data.Binary.IEEE754
import Data.Binary.Put
import Data.ByteString.Lazy    hiding (ByteString)
import Data.Int
import Data.Word
import SwiftNav.SBP.TH
import SwiftNav.SBP.Types

{-# ANN module ("HLint: ignore Use camelCase"::String) #-}
{-# ANN module ("HLint: ignore Redundant do"::String) #-}
{-# ANN module ("HLint: ignore Use newtype instead of data"::String) #-}


-- | GnssSignal.
--
-- Signal identifier containing constellation, band, and satellite identifier
data GnssSignal = GnssSignal
  { _gnssSignal_sat :: !uint32
    -- ^ Constellation-specific satellite identifier
  , _gnssSignal_code :: !uint32
    -- ^ Signal constellation, band and code
  } deriving ( Show, Read, Eq )

instance Binary GnssSignal where
  get = do
    _gnssSignal_sat <- getWord8
    _gnssSignal_code <- getWord8
    pure GnssSignal {..}

  put GnssSignal {..} = do
    putWord8 _gnssSignal_sat
    putWord8 _gnssSignal_code

$(makeJSON "_gnssSignal_" ''GnssSignal)
$(makeLenses ''GnssSignal)

-- | GnssSignalDep.
--
-- Deprecated.
data GnssSignalDep = GnssSignalDep
  { _gnssSignalDep_sat    :: !uint32
    -- ^ Constellation-specific satellite identifier.  Note: unlike GnssSignal,
    -- GPS satellites are encoded as (PRN - 1). Other constellations do not
    -- have this offset.
  , _gnssSignalDep_code   :: !uint32
    -- ^ Signal constellation, band and code
  , _gnssSignalDep_reserved :: !uint32
    -- ^ Reserved
  } deriving ( Show, Read, Eq )

instance Binary GnssSignalDep where
  get = do
    _gnssSignalDep_sat <- getWord16le
    _gnssSignalDep_code <- getWord8
    _gnssSignalDep_reserved <- getWord8
    pure GnssSignalDep {..}

  put GnssSignalDep {..} = do
    putWord16le _gnssSignalDep_sat
    putWord8 _gnssSignalDep_code
    putWord8 _gnssSignalDep_reserved

$(makeJSON "_gnssSignalDep_" ''GnssSignalDep)
$(makeLenses ''GnssSignalDep)

-- | GPSTimeDep.
--
-- A wire-appropriate GPS time, defined as the number of milliseconds since
-- beginning of the week on the Saturday/Sunday transition.
data GpsTimeDep = GpsTimeDep
  { _gpsTimeDep_tow :: !uint32
    -- ^ Milliseconds since start of GPS week
  , _gpsTimeDep_wn :: !uint32
    -- ^ GPS week number
  } deriving ( Show, Read, Eq )

instance Binary GpsTimeDep where
  get = do
    _gpsTimeDep_tow <- getWord32le
    _gpsTimeDep_wn <- getWord16le
    pure GpsTimeDep {..}

  put GpsTimeDep {..} = do
    putWord32le _gpsTimeDep_tow
    putWord16le _gpsTimeDep_wn

$(makeJSON "_gpsTimeDep_" ''GpsTimeDep)
$(makeLenses ''GpsTimeDep)

-- | GPSTimeSec.
--
-- A GPS time, defined as the number of seconds since beginning of the week on
-- the Saturday/Sunday transition.
data GpsTimeSec = GpsTimeSec
  { _gpsTimeSec_tow :: !uint32
    -- ^ Seconds since start of GPS week
  , _gpsTimeSec_wn :: !uint32
    -- ^ GPS week number
  } deriving ( Show, Read, Eq )

instance Binary GpsTimeSec where
  get = do
    _gpsTimeSec_tow <- getWord32le
    _gpsTimeSec_wn <- getWord16le
    pure GpsTimeSec {..}

  put GpsTimeSec {..} = do
    putWord32le _gpsTimeSec_tow
    putWord16le _gpsTimeSec_wn

$(makeJSON "_gpsTimeSec_" ''GpsTimeSec)
$(makeLenses ''GpsTimeSec)

-- | GPSTime.
--
-- A wire-appropriate receiver clock time, defined as the time since the
-- beginning of the week on the Saturday/Sunday transition. In most cases,
-- observations are epoch aligned so ns field will be 0.
data GpsTime = GpsTime
  { _gpsTime_tow       :: !uint32
    -- ^ Milliseconds since start of GPS week
  , _gpsTime_ns_residual :: !sint32
    -- ^ Nanosecond residual of millisecond-rounded TOW (ranges from -500000 to
    -- 500000)
  , _gpsTime_wn        :: !uint32
    -- ^ GPS week number
  } deriving ( Show, Read, Eq )

instance Binary GpsTime where
  get = do
    _gpsTime_tow <- getWord32le
    _gpsTime_ns_residual <- fromIntegral <$> getWord32le
    _gpsTime_wn <- getWord16le
    pure GpsTime {..}

  put GpsTime {..} = do
    putWord32le _gpsTime_tow
    putWord32le $ fromIntegral _gpsTime_ns_residual
    putWord16le _gpsTime_wn

$(makeJSON "_gpsTime_" ''GpsTime)
$(makeLenses ''GpsTime)

-- | CarrierPhase.
--
-- Carrier phase measurement in cycles represented as a 40-bit fixed point
-- number with Q32.8 layout, i.e. 32-bits of whole cycles and 8-bits of
-- fractional cycles. This phase has the same sign as the pseudorange.
data CarrierPhase = CarrierPhase
  { _carrierPhase_i :: !sint32
    -- ^ Carrier phase whole cycles
  , _carrierPhase_f :: !uint32
    -- ^ Carrier phase fractional part
  } deriving ( Show, Read, Eq )

instance Binary CarrierPhase where
  get = do
    _carrierPhase_i <- fromIntegral <$> getWord32le
    _carrierPhase_f <- getWord8
    pure CarrierPhase {..}

  put CarrierPhase {..} = do
    putWord32le $ fromIntegral _carrierPhase_i
    putWord8 _carrierPhase_f

$(makeJSON "_carrierPhase_" ''CarrierPhase)
$(makeLenses ''CarrierPhase)
