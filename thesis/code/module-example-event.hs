newtype EventHandler =
  Event -> Core -> CoreModification

mkEmptyEvent :: String -> String -> Event
mkEmptyEvent moduleName  eventName = Event
  { moduleName
  , eventName
  , arguments = Nothing
  }

data Event = Event
  { moduleName :: String
  , eventName :: String
  , arguments :: Maybe Value
  }
