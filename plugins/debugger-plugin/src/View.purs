module View where


import Prelude

import Data.List (List(..), (:), fromFoldable, mapMaybe)
import Data.Map (keys)
import Data.Maybe (Maybe(..))
import Data.THtml (THtml(..), THtmlKind(..), btn, class_, div_, txt)
import Data.TMap (TMap, TValue(..), lookup, lookupObj, tStr)
import Data.TMsg (msg)
import Data.Tuple (fst)
import Effect.Nmide (getPluginsList)

view :: TMap -> THtml
view model = div_ (tabs : Nil)
  where
    tabs :: THtml
    tabs = THtml 
      { kind: Div
      , attrs: class_ "tab-container"
        : Nil
      , text: Nothing
      , kids: pluginTab
        : messageHistory
        : Nil
      }
    
    pluginTab :: THtml
    pluginTab = THtml
      { kind: Div
      , attrs: class_ "tab-plugin tab-content"
        : Nil
      , text: Just "Plugins"
      , kids: (map (\x -> renderPlugin (fst x)) getPluginsList)
      }
    
    getMsgHistory :: List TValue
    getMsgHistory = case (lookup "message-history" model) of
      Just (List { lst }) -> lst
      _ -> Nil
    
    messageHistory :: THtml
    messageHistory = THtml
      { kind: Div
      , attrs: class_ "tab-message tab-content"
        : Nil
      , text: Just "Messages"
      , kids: mapMaybe (renderMessage) getMsgHistory
      } 

renderPlugin :: String -> THtml
renderPlugin x = THtml
  { kind: P
  , text: Just x
  , attrs: Nil
  , kids: btn "init" (msg "toggle-init" (tStr x))
    : btn "update" (msg "toggle-update" (tStr x))
    : btn "view" (msg "toggle-view" (tStr x))
    : btn "reset" (msg "reset" (tStr x))
    : Nil
  }

renderMessage :: TValue -> Maybe THtml
renderMessage o@(Obj _) = do
    kids <- reMsg
    Just (THtml 
      { kind: Div
      , attrs: class_ "message-container"
        : Nil
      , text: Nothing
      , kids: kids
        : Nil
      })
  where
    reMsg :: Maybe THtml
    reMsg = do
      msg <- message
      val <- tValue
      Just (div_ (txt msg : txt (show val) : Nil))

    message :: Maybe String
    message = case (lookupObj "message" o) of
      Just (Str { str }) -> Just str
      _ -> Nothing
    tValue :: Maybe TValue
    tValue = lookupObj "value" o
renderMessage _ = Nothing