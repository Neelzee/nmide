module Module where

import Core (Core, CoreModification)

data Module = Module
  { name :: String
  , initialize :: Core -> IO CoreModification
  }
