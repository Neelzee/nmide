counterEvent :: Event
counterEvent = Event
  { moduleName = "CounterModule"
  , eventName = "Counter"
  , arguments = Just $ VInt 1
  }

init :: Core -> CoreModification
init core = emptyCoreModification
  { uiModification =
    [ AddUI $ Btn
      [Id "CounterBtn", OnClick counterEvent]
      [Text "Click"]
    ]
  , stateModification = [AddField "Counter" (ValInt 0)]
  , eventHandler = [("Counter", evtHdl)]
  }
