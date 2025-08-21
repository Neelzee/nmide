{-# LANGUAGE NamedFieldPuns #-}

module Modifier where

import Core ( Core(..), CoreModification(..) )
import Mod ( Mod(..), Nav(..) )
import Prelude hiding ( id, mod )
import Value ( Value, State )
import Html
  ( Html(..)
  , getParent
  , appendChild
  , getId
  , hasId
  , getClass
  , hasClass
  , remKid
  )
import Utils ( (?:) )

addField :: String -> Value -> Mod State Core
addField fld val = Add { nav = mkAddState (fld, val) }

addHtml :: Html -> Mod Html Core
addHtml h = Add { nav = mkAddHtml h }

aplMods :: Core -> CoreModification -> (Core, [String])
aplMods c CoreModification { uiMod, stateMod } = let
    foldlE :: (b -> a -> Either b String) -> b -> [a] -> (b, [String])
    foldlE _ acc [] = (acc, [])
    foldlE f acc (x:xs) = case f acc x of
      Left acc' -> foldlE f acc' xs
      Right r -> let
          (acc', rs) = foldlE f acc' xs
        in
          (acc', r : rs)
    (core, reps) = foldlE aplMod c uiMod
    (core', reps') = foldlE aplMod c stateMod
    core'' = c { ui = ui core, state = state core' }
    reps'' = reps ++ reps'
  in
    (core'', reps'')

aplMod :: Core -> Mod a Core -> Either Core String
aplMod core x =
  let
    n = nav x
    root' = root n core
    nodes = root' : nxt n root'
  in
    case x of
      (Add {}) -> addMod core x nodes
      _ -> modMod core x nodes

{-| Probably need this to be some Either type instead, as a predicate might
 not succeed
-}
addMod :: Core -> Mod a Core -> [a] -> Either Core String
addMod _ _ [] = Right "Could not find the parent to add child to"
addMod c x@Add { nav = n@Nav { apl, elm, app } } (y:ys)
  | prd n y = Left $ apl c $ app y elm
  | otherwise = addMod c x ys
addMod c _ _ = Left c -- This doesn't happen

{-| What is the difference between removing a Node, and modifying it? What Node
  is being modified. If we are removing a node, this is equivalent to modifying
  the parent node, and removing a child.
-}
modMod :: Core -> Mod a Core -> [a] -> Either Core String
modMod _ _ [] = Right "Could not find node to modify"
modMod c Add { } _ = Left c
modMod c x (y:ys) = let
  n = nav x
  apl' = apl n
  mod' = mod n
  in
    if prd n y then
      Left $ apl' c $ mod' y
    else
      modMod c x ys


mkAddState :: (String, Value) -> Nav State Core
mkAddState x@(f, _) =
  Nav
  { elm = [x]
  , prd = any $ \y -> f == fst y
  , mod = (:) x
  , nxt = (: [])
  , apl = \ c y -> c { state = y ++ state c }
  , root = state
  , app = (++)
  }

mkModState :: ((String, Value) -> (String, Value)) -> (String, Value) -> Nav State Core
mkModState g x@(f, _) =
  let
    p y = f == fst y
  in
    Nav
    { elm = [x]
    , prd = any p
    , mod = map $ \y -> if p y then g y else y
    , nxt = (: [])
    , apl = \core state' -> core { state = state' ++ state core }
    , root = state
    , app = (++)
    }

mkRemState :: (String, Value) -> Nav State Core
mkRemState x@(f, _) =
  let
    p y = f == fst y
  in
  Nav
  { elm = [x]
  , prd = any p
  , mod = filter p
  , nxt = (: [])
  , apl = \ c y -> c { state = y ++ state c }
  , root = state
  , app = (++)
  }


mkAddHtml :: Html -> Nav Html Core
mkAddHtml h =
  let
    prd = mkHtmlPrd h
    parent = getParent prd h
  in
    Nav
    { elm = h
    , prd = mkHtmlPrd parent
    , mod = appendChild h
    , nxt = kids
    , apl = \ c h' -> c { ui = appendChild (ui c) h' }
    , root = ui
    , app = appendChild
    }

mkHtmlPrd :: Html -> Html -> Bool
mkHtmlPrd h = prdId h ?: (prdClass h ?: (h ==))
  where
    prdId :: Html -> Maybe (Html -> Bool)
    prdId h' = do
      id <- getId h'
      Just (hasId id)

    prdClass :: Html -> Maybe (Html -> Bool)
    prdClass h' = do
      cls <- getClass h'
      Just (hasClass cls)

mkRemHtml :: Html -> Nav Html Core
mkRemHtml h = mkModHtml (remKid $ mkHtmlPrd h) h

mkModHtml :: (Html -> Html) -> Html -> Nav Html Core
mkModHtml mod h =
  let
    prd = mkHtmlPrd h
    parent = getParent prd h
  in
    Nav
    { elm = h
    , prd = mkHtmlPrd parent
    , mod
    , nxt = kids
    , apl = \ c h' -> c { ui = appendChild (ui c) h' }
    , root = ui
    , app = appendChild
    }


foldMod :: CoreModification -> CoreModification -> CoreModification
foldMod a b =
  a
    { uiMod = uiMod a ++ uiMod b
    , stateMod = stateMod a ++ stateMod b
    }
