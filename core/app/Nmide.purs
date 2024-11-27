module Nmide where

import Data.Argonaut (Json)
import Data.Either (Either)
import Data.Unit (Unit)
import Effect (Effect)
import State (TValue)

foreign import app ∷ Unit -> Effect Unit

foreign import debug ∷ TValue -> Effect Unit

foreign import debugJson ∷ Json -> Effect Unit

foreign import debugEither ∷ forall a b. Either a b -> Effect Unit
