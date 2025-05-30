module Core where

import Event (Event)
import Html (Attr, Html)
import Instr (Instr (NoOp))
import Value (State, Value)

type EventHandler = Event -> Core -> CoreModification

data CoreModification = CoreModification
  { _ui :: (Instr Html, Instr Attr, Instr String)
  , _state :: Instr Value
  }

data Core = Core
  { state :: State
  , ui :: Html
  , throwEvent :: Event -> IO ()
  }

emptyCoreModification :: CoreModification
emptyCoreModification =
  CoreModification
    { _ui = (NoOp, NoOp, NoOp)
    , _state = NoOp
    }
