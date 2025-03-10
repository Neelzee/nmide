{-# LANGUAGE NamedFieldPuns #-}

module Nmide where

import qualified Graphics.UI.Threepenny as UI
import Graphics.UI.Threepenny.Core
import Mod ( Mod(..) )
import Core ( Core(..), CoreModification(..) )
import Value (Value)
import Html (Html (..))
import qualified Event as E

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
    throwEvent :: E.Event -> IO ()
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
    applyUIMod c' _ = c'

    applyStateMod :: Core -> [Mod Value] -> Core
    applyStateMod c' _ = c'
