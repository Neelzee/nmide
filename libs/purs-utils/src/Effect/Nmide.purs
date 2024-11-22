module Effect.Nmide
  ( addPlugin
  , getPluginsList
  )
  where


import Prelude

import Data.List (List, fromFoldable)
import Data.Plugin (JsPlugin)
import Data.Tuple (Tuple)
import Effect (Effect)

foreign import getPlugins :: Array (Tuple String JsPlugin)

getPluginsList :: List (Tuple String JsPlugin)
getPluginsList = fromFoldable getPlugins

foreign import addPlugin :: String -> JsPlugin -> Effect Unit