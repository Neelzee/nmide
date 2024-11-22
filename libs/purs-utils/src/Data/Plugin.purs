module Data.Plugin where

import Prelude

import Data.THtml (THtml, THtml', toJsHtml)
import Data.TMap (TMap, TMap', fromJsModel, toJsModel)
import Data.TMsg (TMsg, TMsg', fromJsMsg)

type Plugin =
  { init :: TMap
  , update :: TMsg -> TMap -> TMap
  , view :: TMap -> THtml
  }

type JsPlugin =
  { init :: Unit -> TMap'
  , update :: TMsg' -> TMap' -> TMap'
  , view :: TMap' -> THtml'
  }

toJsPlugin :: Plugin -> JsPlugin
toJsPlugin { init, update, view} =
  { init: (\_ -> toJsModel init)
  , update: update'
  , view: view'
  }
  where
    update' :: TMsg' -> TMap' -> TMap'
    update' msg model = toJsModel (update (fromJsMsg msg) (fromJsModel model))
    view' :: TMap' -> THtml'
    view' model = toJsHtml (view (fromJsModel model))
