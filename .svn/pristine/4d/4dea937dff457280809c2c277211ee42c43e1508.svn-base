async fn init(&self, core: Box<dyn Core>) {
    let mod = CoreModification::default()
        .set_state(StateBuilder::new().add("count", 0));
    core.get_sender().await.send(mod).await.unwrap();
}
