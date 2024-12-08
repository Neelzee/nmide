module Nmide.THtml
  ( TAttr(..)
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
  , txt
  ) where

import Data.Array (uncons)
import Data.Generic.Rep (class Generic)
import Data.Maybe (Maybe(..))
import Data.Nullable (Nullable, notNull, null)
import Data.Show.Generic (genericShow)
import Data.String (toLower)
import Nmide.TMsg (TMsg)
import Prelude (class Eq, class Show, (==))

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

derive instance Generic THtmlKind _

instance showTHtmlKind :: Show THtmlKind where
  show = genericShow

newtype THtml = THtml
  { kind :: THtmlKind
  , attrs :: Array TAttr
  , kids :: Array THtml
  , text :: Nullable String
  }

data TAttr
  = Src String
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
  find' :: Array TAttr -> Maybe TAttr
  find' xs = case uncons xs of
    Just { head: y, tail: ys } ->
      if (isAttr s y) then
        Just y
      else
        find' ys
    _ -> Nothing

emptyHtml :: THtml
emptyHtml = THtml { kind: Frag, attrs: [], kids: [], text: null }

div_ :: Array THtml -> THtml
div_ kids = THtml { kind: Div, attrs: [], kids, text: null }

btn :: String -> TMsg -> THtml
btn s msg = THtml { kind: Button, attrs: [ onclick msg ], kids: [], text: notNull s }

txt :: String -> THtml
txt t = THtml { kind: Text, attrs: [], kids: [], text: notNull t }
