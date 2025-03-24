module Module where

import Core (Core, CoreModification)
import Event (Event)

data Module = Module
  { name :: String
  , initialize :: Core -> IO CoreModification
  , handler :: Event -> Core -> IO CoreModification
  }
