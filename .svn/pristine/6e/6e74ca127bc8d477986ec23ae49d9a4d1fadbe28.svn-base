module Nmide where

import Graphics.UI.Threepenny.Core
import Control.Monad
import Control.Concurrent.STM
import Core ( Core(..), emptyCoreModification )
import Html ( Html ( Div ) )
import Modules ( modules )
import Module ( Module(..) )
import qualified Event as E
import Prelude hiding ( mod, id )
import Modifier (foldMod, aplMods)
import Render (render)

start :: Int -> IO ()
start port = startGUI defaultConfig
  { jsPort = Just port
  , jsCustomHTML = Just "index.html"
  , jsStatic = Just "static"
  } setup

setup :: Window -> UI ()
setup w = do
  let core = Core { ui = Div [] [], state = [], throwEvent = \_ -> pure () }
  cms <- mapM (\m -> liftIO $ initialize m core) modules
  let cms' = foldl foldMod emptyCoreModification cms
  let (core', reps) = aplMods core cms'
  let _ = print reps
  core'' <- liftIO $ newTVarIO core'
  _ <- return w # set title "nmide"
  window <- liftIO $ newTVarIO w
  _ <- liftIO $ atomically (modifyTVar core'' (\c -> c { throwEvent = mkThrowEvent window core'' }))
  c <- liftIO $ readTVarIO core''
  win <- liftIO $ readTVarIO window
  _ <- getBody win #+ [render c $ ui c]
  pure ()

mkThrowEvent :: TVar Window -> TVar Core -> E.Event -> IO ()
mkThrowEvent win c = thEv
  where
    thEv :: E.Event -> IO ()
    thEv evt = do
      core <- readTVarIO c
      w <- liftIO $ readTVarIO win
      cms <- mapM (\m -> handler m evt core) modules
      let cms' = foldl foldMod emptyCoreModification cms
      let (core', reps) = aplMods core cms'
      _ <- print reps
      let _ = reRender core' w (ui core')
      atomically (modifyTVar c $ const core')


reRender :: Core -> Window -> Html -> UI ()
reRender c w r = do
  ui' <- render c r
  _ <- getBody w # set children [ui']
  pure ()
