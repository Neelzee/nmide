module Main where

import Prelude

import Data.List (List(..), (:))
import Data.Plugin (toJsPlugin)
import Effect (Effect)
import Effect.Nmide (addPlugin)
import Update (update)
import Data.Tuple (Tuple(..))
import View (view)
import Data.TMap (TValue(..))

main :: Effect Unit
main = addPlugin "DebuggerPlugin" (toJsPlugin { init: Tuple "message-history" (Obj { obj: Nil }) : Nil, update, view })
