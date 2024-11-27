module TMap
  ( foo
  , bar
  , decodeState
  , TValue(..)
  , TValueInt(..)
  , TValueStr(..)
  , TValueLst(..)
  , TValueObj(..)
  , TMap(..)
  ) where

import Prelude

import Control.Alt ((<|>))
import Control.Monad.State (State)
import Data.Argonaut
  ( class DecodeJson
  , class EncodeJson
  , Json
  , JsonDecodeError(..)
  , decodeJson
  , encodeJson
  , getField
  , jsonEmptyObject
  , jsonEmptyArray
  , (:=)
  , (~>)
  )
import Data.Array (fromFoldable)
import Data.Either (Either(..))
import Data.List (List(..), (:))
import Data.Maybe (Maybe(..))
import Data.String (joinWith)
import Data.Tuple (Tuple(..))
import Foreign.Object (Object(..))

type TValueInt = { int :: Int }
type TValueStr = { str :: String }
type TValueLst a = { lst :: List a }
type TValueObj a = { obj :: List (Tuple String a) }

data TValuePrimitive
  = Int
  | Str
  | Lst (List TValuePrimitive)
  | Obj (List (Tuple String TValuePrimitive))

data TValue
  = TValueInt TValueInt
  | TValueStr TValueStr
  | TValueLst (TValueLst TValue)
  | TValueObj (TValueObj TValue)

instance showTValue :: Show TValue where
  show (TValueInt { int }) = "{ int: " <> show int <> " }"
  show (TValueStr { str }) = "{ str: " <> show str <> " }"
  show (TValueLst { lst }) = "{ lst: [ " <> (joinWith ", " $ fromFoldable $ map show lst) <> " ] }"
  show (TValueObj { obj }) = "{ lst: [ " <> (joinWith ", " $ fromFoldable $ map showObj obj) <> " ] }"

instance decodeJsonTValue :: DecodeJson TValue where
  decodeJson json = decodeTValue json

instance encodeJsonTValue :: EncodeJson TValue where
  encodeJson (TValueInt { int }) = "int" := int ~> jsonEmptyObject
  encodeJson (TValueStr { str }) = "str" := str ~> jsonEmptyObject
  encodeJson (TValueLst { lst }) = "lst" := (map encodeJson lst) ~> jsonEmptyObject
  encodeJson (TValueObj { obj }) = "obj" := (map encodeJson obj) ~> jsonEmptyObject

instance eqTValue :: Eq TValue where
  eq :: TValue -> TValue -> Boolean
  eq (TValueInt { int: a }) (TValueInt { int: b }) = a == b
  eq (TValueStr { str: a }) (TValueStr { str: b }) = a == b
  eq (TValueLst { lst: a }) (TValueLst { lst: b }) = a == b
  eq (TValueObj { obj: a }) (TValueObj { obj: b }) = a == b
  eq _ _ = false

decodeState :: Json -> Either JsonDecodeError (List (Tuple String TMap))
decodeState json = decodeJson json

decodeTValue :: Json -> Either JsonDecodeError TValue
decodeTValue json = do
  obj <- decodeJson json
  decodeTValueInt obj
    <|> decodeTValueStr obj
    <|> decodeTValueLst obj
    <|> decodeTValueObj obj

decodeTValueInt :: Object Json -> Either JsonDecodeError TValue
decodeTValueInt json = do
  int <- getField json "int"
  Right $ TValueInt { int }

decodeTValueStr :: Object Json -> Either JsonDecodeError TValue
decodeTValueStr json = do
  str <- getField json "str"
  Right $ TValueStr { str }

decodeTValueLst :: Object Json -> Either JsonDecodeError TValue
decodeTValueLst json = do
  lst <- getField json "lst"
  Right $ TValueLst { lst }

decodeTValueObj :: Object Json -> Either JsonDecodeError TValue
decodeTValueObj json = do
  obj <- getField json "obj"
  Right $ TValueObj { obj }

showObj :: Tuple String TValue -> String
showObj (Tuple x y) = "[ " <> x <> ", " <> (show y) <> " ]"

type TMap = List (Tuple String TValue)

foo :: TValue
foo = TValueObj
  { obj: Tuple "foo" (TValueInt { int: 10 })
      : Nil
  }

bar :: TMap
bar = Tuple "FooBar" foo : Nil