module Data.TMap
  ( TMap
  , TMap'
  , TValue'(..)
  , TValue(..)
  , constTValue
  , fromJsModel
  , fromJsValue
  , lookup
  , lookupObj
  , tBool
  , tFloat
  , tInt
  , tList
  , tObj
  , tStr
  , toJsModel
  , toJsValue
  )
  where

import Prelude

import Data.Array (fromFoldable)
import Data.List (List(..), (:))
import Data.List as Lists
import Data.Maybe (Maybe(..))
import Data.Tuple (Tuple(..), fst, snd)

data TValue = Int { int :: Int }
  | Float { float :: Number }
  | Str { str :: String }
  | Bool { bool :: Boolean }
  | List { lst :: List TValue }
  | Obj { obj :: List (Tuple String TValue) }


data TValue' = Int' { int :: Int }
  | Float' { float :: Number }
  | Str' { str :: String }
  | Bool' { bool :: Boolean }
  | List' { lst :: Array TValue' }
  | Obj' { obj :: Array (Tuple String TValue') }

toJsValue :: TValue -> TValue'
toJsValue (Int i) = Int' i
toJsValue (Float f) = Float' f
toJsValue (Bool b) = Bool' b
toJsValue (Str s) = Str' s
toJsValue (List { lst }) = List' { lst: (map toJsValue (fromFoldable lst)) }
toJsValue (Obj { obj }) = Obj' { obj: (map (\x -> Tuple (fst x) (toJsValue (snd x))) (fromFoldable obj)) }


fromJsValue :: TValue' -> TValue
fromJsValue (Int' i) = Int i
fromJsValue (Float' f) = Float f
fromJsValue (Bool' b) = Bool b
fromJsValue (Str' s) = Str s
fromJsValue (List' { lst }) = List { lst: (Lists.fromFoldable (map fromJsValue lst)) }
fromJsValue (Obj' { obj }) = Obj { obj: (map (\x -> Tuple (fst x) (fromJsValue (snd x))) (Lists.fromFoldable obj)) }

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

tList :: List TValue -> TValue
tList lst = List { lst }

tObj :: List (Tuple String TValue) -> TValue
tObj obj = Obj { obj }

type TMap = List (Tuple String TValue)

type TMap' = Array (Tuple String TValue')

toJsModel :: TMap -> TMap'
toJsModel model = fromFoldable (map (\x -> Tuple (fst x) (toJsValue (snd x))) model)

fromJsModel :: TMap' -> TMap
fromJsModel model = Lists.fromFoldable (map (\x -> Tuple (fst x) (fromJsValue (snd x))) model)

lookup :: String -> TMap -> Maybe TValue
lookup _ Nil = Nothing
lookup k ((Tuple xk xv):xs)
  | k == xk = Just xv
  | otherwise = lookup k xs

lookupObj :: String -> TValue -> Maybe TValue
lookupObj k (Obj { obj }) = lookup' obj k 
  where
    lookup' :: forall a b. Eq a => List (Tuple a b) -> a -> Maybe b
    lookup' Nil _ = Nothing
    lookup' ((Tuple xk xv):xs) k'
      | xk == k' = Just xv
      | otherwise = lookup' xs k'
lookupObj _ _ = Nothing

constTValue :: TValue -> TValue -> TValue
constTValue (List { lst }) x = List { lst: (x : lst) }
constTValue x _ = x
