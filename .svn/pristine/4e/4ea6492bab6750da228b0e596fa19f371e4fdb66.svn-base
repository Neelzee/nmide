{-# LANGUAGE NamedFieldPuns #-}
module Render where

import qualified Graphics.UI.Threepenny as UI
import Graphics.UI.Threepenny.Core (Element, UI, set, (#), (#+), element, on)
import Html (Html(..), Attr(..))
import Core (Core(..))

render :: Core -> Html -> UI Element
render _ (Text s) = UI.p # set UI.text s
render c (Div _as xs) = UI.div #+ map (render c) xs
render c (P _as xs) = UI.p #+ map (render c) xs
render c (Btn as xs) = do
  btn <- UI.button #+ map (render c) xs
  applyAttr c btn as

applyAttr :: Core -> Element -> [Attr] -> UI Element
applyAttr _ el [] = pure el
applyAttr c@Core { throwEvent } el ((OnClick event) : xs) = do
  _ <- on UI.click el $ const $ UI.liftIO $ throwEvent event
  applyAttr c el xs
applyAttr c el (_ : xs) = applyAttr c el xs
