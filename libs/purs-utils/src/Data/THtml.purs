module Data.THtml
  ( TAttr(..)
  , THtml'(..)
  , THtml(..)
  , THtmlKind(..)
  , btn
  , class_
  , div_
  , emptyHtml
  , id_
  , isKind
  , lookupAttrs
  , onclick
  , src
  , toJsHtml
  , txt
  )
  where

import Data.Array (fromFoldable)
import Data.List (List(..), (:))
import Data.Maybe (Maybe(..))
import Data.Nullable (Nullable, toNullable)
import Data.String (toLower)
import Data.TMsg (TMsg)
import Prelude (class Eq, otherwise, (==), map)

data THtmlKind 
  = A
  | Abbr
  | Address
  | Area
  | Article
  | Aside
  | Audio
  | B
  | Base
  | Bdi
  | Bdo
  | Blockquote
  | Body
  | Br
  | Button
  | Canvas
  | Caption
  | Cite
  | Code
  | Col
  | Colgroup
  | Data
  | Datalist
  | Dd
  | Del
  | Details
  | Dfn
  | Dialog
  | Div
  | Dl
  | Dt
  | Em
  | Embed
  | Frag
  | Fieldset
  | Figcaption
  | Figure
  | Footer
  | Form
  | H1
  | H2
  | H3
  | H4
  | H5
  | H6
  | Head
  | Header
  | Hgroup
  | Hr
  | Html
  | I
  | Iframe
  | Img
  | Input
  | Ins
  | Kbd
  | Label
  | Legend
  | Li
  | Link
  | Main
  | Map
  | Mark
  | Meta
  | Meter
  | Nav
  | Noscript
  | Object
  | Ol
  | Optgroup
  | Option
  | Output
  | P
  | Param
  | Picture
  | Pre
  | Progress
  | Q
  | Rp
  | Rt
  | Ruby
  | S
  | Samp
  | Script
  | Section
  | Select
  | Small
  | Source
  | Span
  | Strong
  | Style
  | Sub
  | Summary
  | Sup
  | Svg
  | Table
  | Tbody
  | Td
  | Template
  | Text
  | Textarea
  | Tfoot
  | Th
  | Thead
  | Time
  | Title
  | Tr
  | Track
  | U
  | Ul
  | Var
  | Video
  | Wbr

derive instance Eq THtmlKind

newtype THtml = THtml { kind :: THtmlKind 
  , attrs :: List TAttr
  , kids :: List THtml
  , text :: Maybe String
  }

newtype THtml' = THtml' { kind :: THtmlKind 
  , attrs :: Array TAttr
  , kids :: Array THtml'
  , text :: Nullable String
  }

toJsHtml :: THtml -> THtml'
toJsHtml (THtml { kind, attrs, kids, text }) = THtml'
  { kind
  , attrs: (fromFoldable attrs)
  , kids: (fromFoldable (map toJsHtml kids))
  , text: (toNullable text)
  }

data TAttr = Src String 
  | Id String
  | Class String
  | EmitInput String
  | OnClick TMsg
  | OnInput TMsg

src :: String -> TAttr
src s = Src s

id_ :: String -> TAttr
id_ s = Id s

class_ :: String -> TAttr 
class_ s = Class s

onclick :: TMsg -> TAttr
onclick m = OnClick m

isAttr :: String -> TAttr -> Boolean
isAttr s (Id _) = toLower s == "id"
isAttr s (Class _) = toLower s == "class"
isAttr s (Src _) = toLower s == "src"
isAttr s (EmitInput _) = toLower s == "emitinput"
isAttr s (OnClick _) = toLower s == "onclick"
isAttr s (OnInput _) = toLower s == "OnInput"

isKind :: THtmlKind -> THtml -> Boolean
isKind k (THtml h) = h.kind == k

lookupAttrs :: String -> THtml -> Maybe TAttr
lookupAttrs s (THtml h) = find' h.attrs
  where
    find' :: List TAttr -> Maybe TAttr
    find' Nil = Nothing
    find' (x:xs)
      | isAttr s x = Just x
      | otherwise = find' xs

emptyHtml :: THtml
emptyHtml = THtml { kind: Frag, attrs: Nil, kids: Nil, text: Nothing }

div_ :: List THtml -> THtml
div_ kids = THtml { kind: Div, attrs: Nil, kids, text: Nothing }

btn :: String -> TMsg -> THtml
btn s msg = THtml { kind: Button, attrs: onclick msg : Nil, kids: Nil, text: Just s }

txt :: String -> THtml
txt t = THtml { kind: Text, attrs: Nil, kids: Nil, text: Just t }