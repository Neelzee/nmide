async fn handler(&self, event: Event, core: Box<dyn Core>) {
    match event.event_name() {
        "get_magnolia_graph" => {
            let path = event.args();
            let key = format!("graph:{path}");
            match core.state().get(&key) {
                Some(existing_graph) => {
                    core.throw_event(Event::new("graph", existing_graph));
                }
                None => {
                    let graph = get_graph(&path);
                    core.throw_event(Event::new("graph", graph));
                    let mods = CoreModification::state(
                        StateBuilder::add(key, graph)
                    );
                    core.send_modification(mods);
                }
            }
        }
        _ => (),
    }
}