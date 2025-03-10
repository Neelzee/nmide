{-# LANGUAGE NamedFieldPuns #-}


module Nmide where

import qualified Graphics.UI.Threepenny as UI
import Graphics.UI.Threepenny.Core
import Module (Core (..), CoreModification (..), Html (..), Mod (..), appendChild)
import qualified Module as M

start :: Int -> IO ()
start port = do
  startGUI defaultConfig
    { jsPort = Just port
    , jsCustomHTML = Just "index.html"
    , jsStatic = Just "static"
    } setup

setup :: Window -> UI ()
setup window = do
  _ <- return window # set title "nmide"
  _ <- getBody window #+ [UI.p # set text "Hello, World!"]
  pure ()

(?:) :: Maybe a -> a -> a
(?:) (Just a) _ = a
(?:) _ a = a

{- NOTE: I have window in the core, because I _feel_ like I need to have access
 - to it, to be able to re-render.
 - -}
startCore :: Window -> Core
startCore _w =
  Core
    { state = []
    , ui = Div [] []
    , throwEvent
    , handlers = []
    , events = []
    }
  where
    throwEvent :: M.Event -> IO ()
    throwEvent _ = putStrLn "Event thrown"

foldMod :: CoreModification -> CoreModification -> CoreModification
foldMod a b =
  a
    { uiMod = uiMod a ++ uiMod b
    , stateMod = stateMod a ++ stateMod b
    , eventHandlers = eventHandlers a ++ eventHandlers b
    }

applyMod :: Core -> CoreModification -> Core
applyMod c cm = c
  { ui = ui (applyUIMod c (uiMod cm))
  , state = state (applyStateMod c (stateMod cm))
  }
  where
    applyUIMod :: Core -> [Mod Html] -> Core
    applyUIMod c' mods = case mods of
      ((Add x) : xs) -> applyUIMod c' {ui = appendChild (ui c') x} xs
      _ -> c'

    applyStateMod :: Core -> [Mod M.Value] -> Core
    applyStateMod c' mods = case mods of
      -- TODO: Correct this to ensure state works
      ((Add x) : xs) -> applyStateMod c' { state = ("", x) : state c' } xs
      _ -> c'
