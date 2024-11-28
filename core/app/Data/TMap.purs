module TMap
  ( foo
  , bar
  , decodeJsonTMap
  , decodeJsonTMapE
  , TValue(..)
  , TValueInt(..)
  , TValueStr(..)
  , TValueList(..)
  , TValueObj(..)
  , TValueFloat(..)
  , TValueBool(..)
  , TMap(..)
  ) where

import Prelude

import Control.Alt ((<|>))
import Data.Argonaut (class DecodeJson, class EncodeJson, Json, JsonDecodeError, decodeJson, encodeJson, getField, jsonEmptyObject, (:=), (~>))
import Data.Array as A
import Data.Either (Either(..))
import Data.String (joinWith)
import Data.Tuple (Tuple(..))
import Foreign.Object (Object)

type TValueInt = { int :: Int }
type TValueStr = { str :: String }
type TValueBool = { bool :: Boolean }
type TValueFloat = { float :: Number }
type TValueList a = { list :: Array a }
type TValueObj a = { obj :: Array (Tuple String a) }

data TValue
  = TValueInt TValueInt
  | TValueFloat TValueFloat
  | TValueBool TValueBool
  | TValueStr TValueStr
  | TValueList (TValueList TValue)
  | TValueObj (TValueObj TValue)

instance showTValue :: Show TValue where
  show (TValueInt { int }) = "{ int: " <> show int <> " }"
  show (TValueFloat { float }) = "{ float: " <> show float <> " }"
  show (TValueBool { bool }) = "{ bool: " <> show bool <> " }"
  show (TValueStr { str }) = "{ str: " <> show str <> " }"
  show (TValueList { list }) = "{ list: [ " <> (joinWith ", " $ A.fromFoldable $ map show list) <> " ] }"
  show (TValueObj { obj }) = "{ list: [ " <> (joinWith ", " $ A.fromFoldable $ map showObj obj) <> " ] }"

instance decodeJsonTValue :: DecodeJson TValue where
  decodeJson json = decodeTValue json

instance encodeJsonTValue :: EncodeJson TValue where
  encodeJson (TValueInt { int }) = "int" := int ~> jsonEmptyObject
  encodeJson (TValueFloat { float }) = "float" := float ~> jsonEmptyObject
  encodeJson (TValueBool { bool }) = "bool" := bool ~> jsonEmptyObject
  encodeJson (TValueStr { str }) = "str" := str ~> jsonEmptyObject
  encodeJson (TValueList { list }) = "list" := (map encodeJson list) ~> jsonEmptyObject
  encodeJson (TValueObj { obj }) = "obj" := (map encodeJson obj) ~> jsonEmptyObject

instance eqTValue :: Eq TValue where
  eq :: TValue -> TValue -> Boolean
  eq (TValueInt { int: a }) (TValueInt { int: b }) = a == b
  eq (TValueFloat { float: a }) (TValueFloat { float: b }) = a == b
  eq (TValueBool { bool: a }) (TValueBool { bool: b }) = a == b
  eq (TValueStr { str: a }) (TValueStr { str: b }) = a == b
  eq (TValueList { list: a }) (TValueList { list: b }) = a == b
  eq (TValueObj { obj: a }) (TValueObj { obj: b }) = a == b
  eq _ _ = false

decodeTValue :: Json -> Either JsonDecodeError TValue
decodeTValue json = do
  obj <- decodeJson json
  decodeTValueInt obj
    <|> decodeTValueStr obj
    <|> decodeTValueFloat obj
    <|> decodeTValueBool obj
    <|> decodeTValueList obj
    <|> decodeTValueObj obj

decodeTValueInt :: Object Json -> Either JsonDecodeError TValue
decodeTValueInt json = do
  int <- getField json "int"
  Right $ TValueInt { int }

decodeTValueFloat :: Object Json -> Either JsonDecodeError TValue
decodeTValueFloat json = do
  float <- getField json "float"
  Right $ TValueFloat { float }

decodeTValueBool :: Object Json -> Either JsonDecodeError TValue
decodeTValueBool json = do
  bool <- getField json "bool"
  Right $ TValueBool { bool }

decodeTValueStr :: Object Json -> Either JsonDecodeError TValue
decodeTValueStr json = do
  str <- getField json "str"
  Right $ TValueStr { str }

decodeTValueList :: Object Json -> Either JsonDecodeError TValue
decodeTValueList json = do
  list <- getField json "list"
  Right $ TValueList { list }

decodeTValueObj :: Object Json -> Either JsonDecodeError TValue
decodeTValueObj json = do
  obj <- getField json "obj"
  Right $ TValueObj { obj }

showObj :: Tuple String TValue -> String
showObj (Tuple x y) = "[ " <> x <> ", " <> (show y) <> " ]"

decodeJsonTMap :: Json -> TMap
decodeJsonTMap json = case decodeJson json of
  Right v -> v
  Left _ -> []

decodeJsonTMapE :: Json -> Either JsonDecodeError TMap
decodeJsonTMapE json = decodeJson json

type TMap = Array (Tuple String TValue)

foo :: TValue
foo = TValueObj
  { obj: [ Tuple "foo" (TValueInt { int: 10 }) ]
  }

bar :: TMap
bar = [ Tuple "FooBar" foo ]
