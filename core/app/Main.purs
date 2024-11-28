module Main (main) where

import Prelude

import Effect (Effect)
import Nmide as N
import Web.Event.Event (Event)
import Web.Event.EventTarget (addEventListener, eventListener)
import Web.HTML (window)
import Web.HTML.Event.EventTypes (domcontentloaded)
import Web.HTML.Window (toEventTarget)

main :: Effect Unit
main = do
  w <- window
  appEvent <- eventListener startApp
  addEventListener domcontentloaded appEvent true (toEventTarget w)

startApp :: Event -> Effect Unit
startApp _ = N.app
