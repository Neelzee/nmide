module Update where

import Data.List (List(..), (:))
import Data.Maybe (Maybe(..))
import Data.TMap (TMap, TValue(..), constTValue, lookup)
import Data.TMsg (TMsg, getMsg, getVal)
import Data.Tuple (Tuple(..))

update :: TMsg -> TMap -> TMap
update msg model = case lookup "message-history" model of
  Just l@(List _) -> Tuple "message-history" (constTValue l toTVal) : Nil
  _ -> Tuple "message-history" (Obj { obj: Nil }) : Nil
  where
    toTVal :: TValue
    toTVal = Obj
      { obj: Tuple "message" (Str { str: (getMsg msg) }) 
      : Tuple "value" (getVal msg)
      : Nil
      }