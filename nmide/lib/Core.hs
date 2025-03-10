module Core where

import Mod (Mod)
import Html (Html)
import Value (Value)
import Event (Event)

type EventHandler = Event -> Core -> CoreModification

data CoreModification = CoreModification
  { uiMod :: [Mod Html]
  , stateMod :: [Mod Value]
  , eventHandlers :: [(String, EventHandler)]
  }

data Core = Core
  { state :: [(String, Value)]
  , ui :: Html
  , throwEvent :: Event -> IO ()
  , handlers :: [(String, EventHandler)]
  , events :: [String]
  }


emptyCoreModification :: CoreModification
emptyCoreModification = CoreModification
  { uiMod = []
  , stateMod = []
  , eventHandlers = []
  }
