init :: State
init = [("counter", VInt 0)]

update :: Msg -> State -> State
update (Msg { msg: "counter", val: VInt i }) model =
  case lookup "counter" model of
    Just (VInt j) ->
      insert "counter" (VInt (j + i)) model
    Nothing -> insert "counter" (VInt 0) model
update _ m = m

view :: State -> HTML
view model = Div [] [Text "Hello, World!"
  , Btn [OnClick $ Msg { msg: "counter", val: VInt 1 }] []
  , Text $ putStrLn $ lookup "counter" model
