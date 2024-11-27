module Main where

import Prelude
import State

import Data.Argonaut (Json, JsonDecodeError, decodeJson, encodeJson)
import Data.Either (Either(..))
import Effect (Effect)
import Nmide as N
import Web.Event.Event (EventType(..))
import Web.Event.EventTarget (addEventListener, eventListener)
import Web.HTML (window)
import Web.HTML.Window (toEventTarget)

main :: Effect Unit
main = do
  w <- window
  let windowTarget = toEventTarget w
  domContentLoaded <- eventListener (\_ -> N.app unit)
  addEventListener (EventType "DOMContentLoaded") domContentLoaded false windowTarget

