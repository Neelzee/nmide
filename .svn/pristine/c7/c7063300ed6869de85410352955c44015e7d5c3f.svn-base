data CoreModification = CoreModification
  { uiModification :: [UIMod]
  , stateModification :: [StateMod]
  , eventHandlers :: [(String, EventHandler)]
  }

data Core = Core
  { state :: State
  , ui :: HTML
  , throwEvent :: Event -> IO ()
  }
