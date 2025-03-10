module Html where

import Event (Event)

data Html
  = Text String
  | Div {attrs :: [Attr], kids :: [Html]}
  | P {attrs :: [Attr], kids :: [Html]}
  | Btn {attrs :: [Attr], kids :: [Html]}

appendChild :: Html -> Html -> Html
appendChild x y = case x of
  (Text _) -> P [] [x, y]
  _ -> x {kids = y : kids x}

data Attr
  = Id String
  | OnClick Event
