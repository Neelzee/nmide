init :: CoreModification
init = emptyCoreModification
  { uiModification =
    [ Button
      [OnClick "BtnClick" 1, Id "Button-Id"]
      [Text "Click 0"]
    ]
  , stateModification =
    [ AddField "counter" 0
    ]
  , eventHandlers = [("BtnClick", eventHandler)]
  }
