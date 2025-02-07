data Msg = Msg
  { msg :: String
  , val :: Value
  }

data HTML
  = Div [Attributes] [HTML]
  | Btn [Attributes] [ Html]
  | Text String
  -- ...

data Attributes
  = OnClick Msg
  | Id String
  -- ...
