module Main where

import Nmide ( start )
import System.Environment ( getArgs )
import System.IO

main :: IO ()
main = do
  hSetBuffering stdout LineBuffering
  [port] <- getArgs
  start (read port)
