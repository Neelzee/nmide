module Render where

import Graphics.UI.Threepenny (element)
import qualified Graphics.UI.Threepenny as UI
import Graphics.UI.Threepenny.Core (Element, UI, on, set, (#), (#+))
import Html (Html(..), Attr(..))
import Event (eventName)

render :: Html -> UI Element
render (Text s) = UI.p # set UI.text s
render (Div _as xs) = UI.div #+ map render xs
render (P _as xs) = UI.p #+ map render xs
render (Btn as xs) = do
  btn <- UI.button #+ map render xs
  applyAttr btn as

applyAttr :: Element -> [Attr] -> UI Element
applyAttr el [] = pure el
applyAttr el ((OnClick event) : xs) = do
  _ <- on UI.click el $ const $ element el # set UI.text (eventName event)
  applyAttr el xs
applyAttr el (_ : xs) = applyAttr el xs
