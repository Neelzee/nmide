module Nmide.Plugin where

import Prelude

import Nmide.THtml (THtml)
import Nmide.TMap (TMap)
import Nmide.TMsg (TMsg)

type Plugin =
  { init :: Unit -> TMap
  , update :: TMsg -> TMap -> TMap
  , view :: TMap -> THtml
  }

