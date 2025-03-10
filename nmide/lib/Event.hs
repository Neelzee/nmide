module Event where

import Value (Value)

data Event = Event
  { moduleName :: String
  , eventName :: String
  , arguments :: Maybe Value
  }
