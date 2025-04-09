module Mod where

import Prelude hiding (mod)

data Nav a b
  = Nav
  { elm :: a           -- ^ The node to be added/removed
  , prd :: a -> Bool   -- ^ When `mod` should be applied
  , mod :: a -> a      -- ^ Modifies a node
  , nxt :: a -> [a]    -- ^ Get the child nodes from a node
  , apl :: b -> a -> b -- ^ Add the node to the tree
  , root :: b -> a     -- ^ Get the root node from a tree
  , app :: a -> a -> a -- ^ Append one node to another
  }

data Mod a b
  = Add { nav :: Nav a b }
  | Rem { nav :: Nav a b }
  | Mod { nav :: Nav a b }
