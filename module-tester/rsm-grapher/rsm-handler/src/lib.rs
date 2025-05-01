use anyhow::{Context, Result, anyhow};
use core_module_lib::Module;
use core_std_lib::event::Event;
use futures::FutureExt;
use rsm_invoker::{CONSUMER, Core, Dependency, MODULE, SUITE, THROWN_EVENTS};
use std::time::Duration;
use tokio::{
    sync::{mpsc, oneshot},
    time::sleep,
};

pub async fn handle(
    module: Box<dyn Module>,
    initial_events: Vec<Event>,
    dur: Duration,
) -> Result<Dependency> {
    let mut suite = SUITE.write().await;
    suite.initialize(module).await;
    let (sender, mut recv) = mpsc::channel::<bool>(100);
    tokio::spawn(async move {
        for evt in initial_events {
            let result = tokio::task::spawn(async move {
                let module = MODULE.read().await;
                let mod_handler = module.handler(evt, Box::new(Core));
                let mod_timeout = async { sleep(dur).await };
                futures::select_biased! {
                    _ = mod_handler.fuse() => Result::<bool>::Ok(true),
                    _ = mod_timeout.fuse() => Result::<bool>::Ok(false),
                }
            })
            .await;

            let success = match result {
                Ok(Ok(success)) => success,
                Ok(Err(e)) => {
                    eprintln!("Handler returned an error: {:?}", e);
                    false
                }
                Err(e) => {
                    eprintln!("Handler panicked: {:?}", e);
                    false
                }
            };

            sender.send(success).await.expect("Channel should be open");
        }
    });

    let (s, r) = oneshot::channel::<Vec<Dependency>>();
    tokio::spawn(async move {
        let mut deps = Vec::new();
        while let Some(success) = recv.recv().await {
            if success {
                sleep(dur).await;
            }
            let providing = THROWN_EVENTS.read().await.clone();
            let consuming = CONSUMER.read().await.clone();
            deps.push(Dependency {
                providing,
                consuming,
                success,
            });
        }
        s.send(deps).expect("Channel should be open");
    });

    match r.await {
        Ok(deps) => deps
            .into_iter()
            .reduce(|a, b| fold_deps(a, b))
            .ok_or(anyhow!("No dependencies found")),
        Err(err) => Err(anyhow!(err)),
    }
}

pub fn fold_deps(a: Dependency, b: Dependency) -> Dependency {
    let mut xs = a.consuming;
    let mut ys = b.consuming;
    xs.append(&mut ys);
    xs.sort();
    xs.dedup();
    let mut zs = a.providing;
    let mut ws = b.providing;
    zs.append(&mut ws);
    zs.sort();
    zs.dedup();
    Dependency {
        providing: zs,
        consuming: xs,
        success: a.success && b.success,
    }
}
