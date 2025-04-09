evtHdl :: EventHandler
evtHdl evt c = case (eventName evt, arguments evt) of
  ("Counter", (Just i)) -> emptyCoreModification
    { stateModification =
      [UpdateField "Counter" (\x -> x + i)]
    }
  _ -> emptyCoreModification
