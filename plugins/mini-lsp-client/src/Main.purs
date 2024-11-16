module Main (main) where

import Prelude

import Data.Maybe (Maybe(..))
import Data.Nullable (toNullable)
import Effect (Effect)
import Effect.InstallPlugin (installPlugin)
import Plugins (THtml(..), TMap, TMsg)

main :: Effect Unit
main = installPlugin "PureScriptPlugin" ({ init, update, view })

init :: Unit -> TMap
init _ = []

update :: TMsg -> TMap -> TMap
update _ _ = []

view :: TMap -> THtml
view _ = THtml 
  { kind: "P"
  , attrs: [] :: Array Unit
  , kids: [] :: Array THtml
  , text: (toNullable $ Just "Hello, World!")
  }