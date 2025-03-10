module Mod where

data Mod a
  -- Adds a Node
  = Add
  { node :: a
  , pred :: a -> Bool
  }
  -- Removes a Node
  | Rem
  { pred :: a -> Bool
  }
  -- Modifies a Node
  | Mod
  { pred :: a -> Bool
  , mod :: a -> a
  }
