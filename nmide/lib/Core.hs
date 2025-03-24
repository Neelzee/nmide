module Core where

import Mod (Mod)
import Html (Html)
import Value (State)
import Event (Event)

type EventHandler = Event -> Core -> CoreModification

data CoreModification = CoreModification
  { uiMod :: [Mod Html Core]
  , stateMod :: [Mod State Core]
  }

data Core = Core
  { state :: State
  , ui :: Html
  , throwEvent :: Event -> IO ()
  }

emptyCoreModification :: CoreModification
emptyCoreModification = CoreModification
  { uiMod = []
  , stateMod = []
  }
