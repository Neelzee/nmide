module Nmide.TMap
  ( TMap(..)
  , TValue(..)
  , lookup
  , lookupObj
  , tBool
  , tFloat
  , tInt
  , tList
  , tObj
  , tStr
  ) where

import Prelude

import Data.Array (uncons)
import Data.Maybe (Maybe(..))
import Data.Tuple (Tuple(..))

data TValue
  = Int { int :: Int }
  | Float { float :: Number }
  | Str { str :: String }
  | Bool { bool :: Boolean }
  | List { lst :: Array TValue }
  | Obj { obj :: Array (Tuple String TValue) }

instance showTValue :: Show TValue where
  show (Int { int }) = "{ int: " <> show int <> " }"
  show (Float { float }) = "{ float: " <> show float <> " }"
  show (Str { str }) = "{ str: " <> str <> " }"
  show (Bool { bool }) = "{ bool: " <> show bool <> " }"
  show (List { lst }) = "{ lst: " <> show (map show lst) <> " }"
  show (Obj { obj }) = "{ lst: " <> show (map show obj) <> " }"

tInt :: Int -> TValue
tInt int = Int { int }

tFloat :: Number -> TValue
tFloat float = Float { float }

tStr :: String -> TValue
tStr str = Str { str }

tBool :: Boolean -> TValue
tBool bool = Bool { bool }

tList :: Array TValue -> TValue
tList lst = List { lst }

tObj :: Array (Tuple String TValue) -> TValue
tObj obj = Obj { obj }

type TMap = Array (Tuple String TValue)

lookup :: forall a. String -> Array (Tuple String a) -> Maybe a
lookup k xs = case uncons xs of
  Just { head: Tuple yk yv, tail: ys } ->
    if k == yk then
      Just yv
    else
      lookup k ys
  Nothing -> Nothing

lookupObj :: String -> TValue -> Maybe TValue
lookupObj k (Obj { obj }) = lookup k obj
lookupObj _ _ = Nothing

