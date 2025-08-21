{-# LANGUAGE NamedFieldPuns #-}
module Modules ( modules ) where

import Module (Module(..))
import Core (emptyCoreModification, Core(..), CoreModification(..))
import Value (Value(..), access)
import Modifier (addField, addHtml)
import Html (Html(..), Attr(..))
import Event (Event (..))


modules :: [Module]
modules =
  [ counterModule
  ]


counterModule :: Module
counterModule = Module
  { name = "Counter Module"
  , initialize
  , handler = const $ const $ pure emptyCoreModification
  }
  where
    event :: Event
    event = Event { moduleName = "Counter Module", eventName = "CounterEvent", arguments = Just $ VInt 1 }

    initialize :: Core -> IO CoreModification
    initialize c = do
      let counterState = [addField "Counter" $ VInt 0]
      let count = access "Counter" $ state c
      let counterUI = addHtml $ Div [] [ Btn [OnClick event] [ Text "Click" ]
                                         , Text $ "Counter: " ++ show count
                                         ]
      pure emptyCoreModification { uiMod = [counterUI], stateMod = counterState }
