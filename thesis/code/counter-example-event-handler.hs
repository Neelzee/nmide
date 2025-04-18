eventHandler :: Event -> Core -> CoreModification
eventHandler (Event "BtnClick" val) core
  = emptyCoreModification
  { uiModification =
    [ modifyUi
      { id = "Button-Id"
      , text = "Click " ++ show (val + 1)
      }
    ]
  , stateModification =
    [ modifyField
      { field = "counter"
      , value = val + 1
      }
    ]
  }
eventHandler _ _ = emptyCoreModification
