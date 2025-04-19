async fn init(&self, core: Box<dyn Core>) {
    let mod = CoreModification::default()
        .set_state(UIBuilder::new().add(None, None, Html::Div()));
    core.get_sender().await.send(mod).await.unwrap();
}
