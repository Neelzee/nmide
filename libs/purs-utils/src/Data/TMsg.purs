module Data.TMsg
  ( TMsg'(..)
  , TMsg(..)
  , fromJsMsg
  , getMsg
  , getVal
  , isMsg
  , msg
  , toJsMsg
  )
  where

import Data.TMap (TValue, TValue', fromJsValue, toJsValue)
import Data.Tuple (Tuple(..), fst, snd)
import Prelude ((==))

newtype TMsg = Msg { msg :: Tuple String TValue }
newtype TMsg' = Msg' { msg :: Tuple String TValue' }

toJsMsg :: TMsg -> TMsg'
toJsMsg (Msg { msg: m }) = Msg' { msg: Tuple (fst m) (toJsValue (snd m)) }

fromJsMsg :: TMsg' -> TMsg
fromJsMsg (Msg' { msg: m }) = Msg { msg: Tuple (fst m) (fromJsValue (snd m)) }


getMsg :: TMsg -> String
getMsg (Msg m) = fst m.msg

getVal :: TMsg -> TValue
getVal (Msg m) = snd m.msg

isMsg :: String -> TMsg -> Boolean
isMsg x y = x == getMsg y


msg :: String -> TValue -> TMsg
msg x y = Msg { msg: Tuple x y }