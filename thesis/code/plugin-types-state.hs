data Value
  = VInt Int
  | VStr String
  | VBool Bool
  | VFloat Float
  | VLst [Value]
  | VObj [(String, Value)]

newtype State = [(String, Value)]
