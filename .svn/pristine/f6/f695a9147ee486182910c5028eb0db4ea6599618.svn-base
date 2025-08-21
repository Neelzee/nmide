module Nmide.TMsg
  ( TMsg(..)
  , getMsg
  , getVal
  , isMsg
  , msg
  ) where

import Nmide.TMap (TValue)
import Data.Tuple (Tuple(..), fst, snd)
import Prelude ((==))

newtype TMsg = Msg { msg :: Tuple String TValue }

getMsg :: TMsg -> String
getMsg (Msg m) = fst m.msg

getVal :: TMsg -> TValue
getVal (Msg m) = snd m.msg

isMsg :: String -> TMsg -> Boolean
isMsg x y = x == getMsg y

msg :: String -> TValue -> TMsg
msg x y = Msg { msg: Tuple x y }
