module Module where

data Value
  = VInt Int
  | VFloat Double
  | VBool Bool
  | VStr String
  | VLst [Value]
  | VObj [(String, Value)]

data Html
  = Text String
  | Div {attrs :: [Attr], kids :: [Html]}
  | P {attrs :: [Attr], kids :: [Html]}
  | Btn {attrs :: [Attr], kids :: [Html]}

appendChild :: Html -> Html -> Html
appendChild x y = case x of
  (Text _) -> P [] [x, y]
  _ -> x {kids = y : kids x}

data Attr
  = Id String
  | OnClick Event

-- TODO: Rewrite Mod to actually work
-- | Modifies a node-like type
data Mod a
  -- Adds a Node
  = Add a
  -- Removes a Node
  | Rem a
  -- Modifies a Node
  | Mod a

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

type EventHandler = Event -> Core -> CoreModification

data Event = Event
  { moduleName :: String
  , eventName :: String
  , arguments :: Maybe Value
  }

data Module = Module
  { name :: String
  , initialize :: Core -> IO CoreModification
  }

emptyCoreModification :: CoreModification
emptyCoreModification = CoreModification
  { uiMod = []
  , stateMod = []
  , eventHandlers = []
  }
