module State (stateHandler) where

import Data.Maybe (Maybe(..))
import Prelude

import Data.Array (concatMap, elem, foldl, groupBy, null, sortBy, uncons, (:))
import Data.Array.NonEmpty (NonEmptyArray)
import Data.Array.NonEmpty as NE
import Data.Either (Either(..))
import Data.Tuple (Tuple(..), fst, snd)
import TMap (TMap)

type Collision = Tuple (Array String) TMap

stateHandler :: Array (Tuple String TMap) -> Either TMap (Array Collision)
stateHandler xs = do
  let ys = groupByEqFields
  let model = concatMap snd $ map NE.head ys
  let collisions = map NE.tail ys
  if (null collisions) then
    Left model
  else
    Right $ map getCollision collisions
  where
  getCollision :: Array (Tuple String TMap) -> Collision
  getCollision ys = Tuple (map fst ys) (foldl combine [] $ map snd ys)

  groupByEqFields :: Array (NonEmptyArray (Tuple String TMap))
  groupByEqFields = groupBy tmapFieldEq sortXs

  tmapFieldEq :: Tuple String TMap -> Tuple String TMap -> Boolean
  tmapFieldEq (Tuple _ ys) (Tuple _ zs) = tmapFieldEq' ys (map fst zs)

  tmapFieldEq' :: TMap -> Array String -> Boolean
  tmapFieldEq' ys zs = case uncons ys of
    Just { head: (Tuple y _), tail: ys' } ->
      if elem y zs then
        true
      else
        tmapFieldEq' ys' zs
    Nothing -> false

  sortXs :: Array (Tuple String TMap)
  sortXs = sortBy (comparing fst) xs

  combine :: forall a. Array a -> Array a -> Array a
  combine ys zs = case uncons ys of
    Just { head: y, tail: ys' } -> combine ys' (y : zs)
    Nothing -> zs

