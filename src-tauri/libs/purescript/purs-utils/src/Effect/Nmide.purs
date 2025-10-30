module Effect.Nmide
  ( addPlugin
  , getPlugins
  ) where

import Prelude

import Nmide.Plugin (Plugin)
import Data.Tuple (Tuple)
import Effect (Effect)

foreign import getPlugins :: Effect (Array (Tuple String Plugin))

foreign import addPlugin :: String -> Plugin -> Effect Unit
