module Value where

data Value
  = VInt Int
  | VFloat Double
  | VBool Bool
  | VStr String
  | VLst [Value]
  | VObj [(String, Value)]
  deriving (Show, Eq)

type State = [(String, Value)]

kids :: Value -> [(String, Value)]
kids (VObj k) = k
kids _ = []

flatten :: State -> State
flatten [] = []
flatten ((p, VObj x):xs) = flatten' p x ++ flatten xs
  where
    flatten' :: String -> [(String, Value)] -> [(String, Value)]
    flatten' _ [] = []
    flatten' parent ((f, VObj k):ys) = flatten' (parent ++ "." ++ f) k ++ flatten' parent ys
    flatten' parent ((f, y):ys) = (parent ++ "." ++ f, y) : flatten' parent ys
flatten (x:xs) = x : flatten xs

access :: String -> State -> Maybe Value
access k s = lookup k $ flatten s
