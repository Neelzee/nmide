  stateHandler :: [(String, TMap)] -> [Either [(String, TMap)] ([(String, TMap)], String)]
  stateHandler xs = map partitionStateCollision (groupBy stateCollision xs)
  where
  -- Returns true if they have the same fields
  stateCollision :: (String, TMap) -> (String, TMap) -> Bool
  stateCollision [] _ = false
  stateCollision ((a, _):xs) ys
  | a `elem` (map fst ys) = true
  | otherwise = stateCollision xs ys
  {- Returns the collision-field
  Is only called in the context where there is a collision.
  -}
  getCollisionField :: [(String, TMap)] -> String
  getCollisionField [] = "" -- Will never happen
  getCollisionField ((a, _):ys)
  | a `elem` (map fst xs) = a
  | otherwise = getCollisionField ys
  partitionStateCollision :: [[(String, TMap)]] -> [Either [(String, TMap)] ([(String, TMap)], String)]
  partitionStateCollision [] = []
  partitionStateCollision ([ys]:xs) = Left ys : partitionStateCollision xs
  partitionStateCollision (ys:xs) =  Right (ys, getCollisionField ys) : partitionStateCollision xs
