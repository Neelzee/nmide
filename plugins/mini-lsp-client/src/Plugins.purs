module Plugins
  ( Plugin
  , THtml(..)
  , TMap
  , TMsg(..)
  , TValue(..)
  )
  where

import Prelude
import Data.Nullable (Nullable(..))
import Data.Tuple (Tuple)

data TValue = Int { x :: Int }
  | Float { x :: Number }
  | Str { x :: String }
  | Bool { x :: Boolean }
  | List { x :: Array TValue }
  | Obj { x :: Array (Tuple String TValue) }

type TMap = Array (Tuple String TValue)

data TMsg = Msg { x :: Tuple String TValue }

newtype THtml = THtml { kind :: String 
  , attrs :: Array Unit
  , kids :: Array THtml
  , text :: Nullable String
  }

type Plugin =
  { init :: Unit -> TMap
  , update :: TMsg -> TMap -> TMap
  , view :: TMap -> THtml
  }