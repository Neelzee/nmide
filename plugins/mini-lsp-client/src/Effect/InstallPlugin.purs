module Effect.InstallPlugin
  ( installPlugin
  )
  where

import Prelude

import Effect (Effect)
import Plugins (Plugin)

foreign import installPlugin :: String -> Plugin -> Effect Unit