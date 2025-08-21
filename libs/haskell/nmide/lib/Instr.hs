module Instr where

data Instr a 
  = NoOp
  | Add String a
  | Rem String a
  | Then (Instr a) (Instr a)
