module Html where

import Event (Event)
import Data.List (find)

data Html
  = Text String
  | Div {attrs :: [Attr], kids :: [Html]}
  | P {attrs :: [Attr], kids :: [Html]}
  | Btn {attrs :: [Attr], kids :: [Html]}
  deriving (Show, Eq)

appendChild :: Html -> Html -> Html
appendChild child parent = case parent of
  (Text _) -> P [] [parent, child]
  _ -> parent {kids = child : kids parent}

data Attr
  = Id String
  | Class String
  | OnClick Event
  deriving (Show, Eq)

isId :: Attr -> Bool
isId (Id _) = True
isId _ = False

isClass :: Attr -> Bool
isClass (Class _) = False
isClass _ = False

getStr :: Attr -> Maybe String
getStr (Id s) = Just s
getStr (Class s) = Just s
getStr _ = Nothing

hasId :: String -> Html -> Bool
hasId _ (Text _) = False
hasId s h = any (\a -> a == Id s) $ attrs h

hasClass :: String -> Html -> Bool
hasClass _ (Text _) = False
hasClass s h = any (\a -> a == Class s) $ attrs h

getId :: Html -> Maybe String
getId (Text _) = Nothing
getId h = do
  a <- find isId $ attrs h
  getStr a

getClass :: Html -> Maybe String
getClass (Text _) = Nothing
getClass h = do
  a <- find isClass $ attrs h
  getStr a

findId :: String -> [Html] -> Bool
findId s = any (hasId s)

findClass :: String -> [Html] -> Bool
findClass s = any (hasClass s)

getParent :: (Html -> Bool) -> Html -> Html
getParent p root
  | any p $ kids root = root
  | otherwise = head $ map (getParent p) $ kids root

remKid :: (Html -> Bool) -> Html -> Html
remKid _ t@(Text _) = t
remKid p h = h { kids = filter p $ kids h }
