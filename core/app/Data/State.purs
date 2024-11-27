module State (stateHandler) where

import Prelude
import Data.List (List(..), foldl, groupBy, elem, sortBy, concatMap, null)
import Data.List.Types
import Data.Either (Either(..))
import Data.Maybe
import Data.List.NonEmpty as NE
import Data.Tuple (Tuple(..), fst, snd)
import TMap (TMap)

type Collision = Tuple (List String) TMap

stateHandler :: List (Tuple String TMap) -> Either TMap (List Collision)
stateHandler xs = do
  let ys = groupByEqFields
  let model = concatMap snd $ map NE.head ys
  let collisions = map NE.tail ys
  if (null collisions) then
    Left model
  else
    Right $ map getCollision collisions
  where
  getCollision :: List (Tuple String TMap) -> Collision
  getCollision ys = Tuple (map fst ys) (foldl combine Nil $ map snd ys)

  groupByEqFields :: List (NonEmptyList (Tuple String TMap))
  groupByEqFields = groupBy tmapFieldEq sortXs

  tmapFieldEq :: Tuple String TMap -> Tuple String TMap -> Boolean
  tmapFieldEq (Tuple _ ys) (Tuple _ zs) = tmapFieldEq' ys (map fst zs)

  tmapFieldEq' :: TMap -> List String -> Boolean
  tmapFieldEq' Nil _ = false
  tmapFieldEq' ((Tuple y _) : ys) zs
    | elem y zs = true
    | otherwise = tmapFieldEq' ys zs

  sortXs :: List (Tuple String TMap)
  sortXs = sortBy (comparing fst) xs

  combine :: forall a. List a -> List a -> List a
  combine Nil zs = zs
  combine (y : ys) zs = combine ys (y : zs)

