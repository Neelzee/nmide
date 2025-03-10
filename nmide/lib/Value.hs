module Value where

data Value
  = VInt Int
  | VFloat Double
  | VBool Bool
  | VStr String
  | VLst [Value]
  | VObj [(String, Value)]
