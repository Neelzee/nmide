module TMap
  ( foo
  , bar
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
import Data.Argonaut (class DecodeJson, class EncodeJson, Json, JsonDecodeError, decodeJson, encodeJson, getField, jsonEmptyObject, parseJson, (:=), (~>))
import Data.Argonaut.Decode.Generic (genericDecodeJson)
import Data.Argonaut.Encode.Generic (genericEncodeJson)
import Data.Array as A
import Data.Either (Either(..))
import Data.Generic.Rep (class Generic)
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

derive instance genericTValue :: Generic TValue _

instance encodeJsonTValue :: EncodeJson TValue where
  encodeJson x = genericEncodeJson x

instance decodeJsonTValue :: DecodeJson TValue where
  decodeJson x = genericDecodeJson x

instance showTValue :: Show TValue where
  show (TValueInt { int }) = "{ int: " <> show int <> " }"
  show (TValueFloat { float }) = "{ float: " <> show float <> " }"
  show (TValueBool { bool }) = "{ bool: " <> show bool <> " }"
  show (TValueStr { str }) = "{ str: " <> show str <> " }"
  show (TValueList { list }) = "{ list: [ " <> (joinWith ", " $ A.fromFoldable $ map show list) <> " ] }"
  show (TValueObj { obj }) = "{ list: [ " <> (joinWith ", " $ A.fromFoldable $ map showObj obj) <> " ] }"

instance eqTValue :: Eq TValue where
  eq :: TValue -> TValue -> Boolean
  eq (TValueInt { int: a }) (TValueInt { int: b }) = a == b
  eq (TValueFloat { float: a }) (TValueFloat { float: b }) = a == b
  eq (TValueBool { bool: a }) (TValueBool { bool: b }) = a == b
  eq (TValueStr { str: a }) (TValueStr { str: b }) = a == b
  eq (TValueList { list: a }) (TValueList { list: b }) = a == b
  eq (TValueObj { obj: a }) (TValueObj { obj: b }) = a == b
  eq _ _ = false

showObj :: Tuple String TValue -> String
showObj (Tuple x y) = "[ " <> x <> ", " <> (show y) <> " ]"

type TMap = Array (Tuple String TValue)

foo :: TValue
foo = TValueObj
  { obj: [ Tuple "foo" (TValueInt { int: 10 }) ]
  }

bar :: TMap
bar = [ Tuple "FooBar" foo ]
