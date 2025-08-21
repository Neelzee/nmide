-- A Module, and its corresponding State
newtype ModuleState = (String, State)
{- Field the collision occurred on, list of modules, and the state with the
  collision -}
newtype CollisionReport = (State, [(String, [ModuleState])])
  -- Combines frontend and backend states, removing all collisions
stateUpdateHandler :: [ModuleState] -> [ModuleState] -> State
stateUpdateHandler fs bs = first $ map foldPartition (group (fs ++ bs))

foldPartition :: CollisionReport -> [ModuleState] -> CollisionReport
foldPartition acc cur = bimap (map snd (head cur) :) (tail cur :) acc
