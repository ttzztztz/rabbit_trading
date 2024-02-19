use std::{
    collections::LinkedList,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use tokio::sync::{broadcast::Receiver, RwLock, RwLockReadGuard};

use crate::{
    model::{
        common::{error::Error, types::ConfigMap},
        trading::event::RabbitTradingEvent,
    },
    pod::event::listener::common::listener::EventListenerTrait,
};

pub struct LogContainerEventListener {
    config_map: ConfigMap,
    data: Arc<RwLock<LinkedList<RabbitTradingEvent>>>,
    stopped_indicator: Arc<AtomicBool>,
}

impl LogContainerEventListener {
    async fn async_log_task(
        mut receiver: Receiver<RabbitTradingEvent>,
        data_container: Arc<RwLock<LinkedList<RabbitTradingEvent>>>,
        stopped_indicator: Arc<AtomicBool>,
    ) {
        loop {
            if stopped_indicator.load(Ordering::Relaxed) {
                return;
            }

            match receiver.recv().await {
                Ok(event) => {
                    let mut write_guard = data_container.write().await;
                    write_guard.push_back(event);
                }
                Err(error) => {
                    log::error!("Error when polling data from event bus, {}", error);
                    return;
                }
            }
        }
    }

    pub async fn inspect_log(&self) -> RwLockReadGuard<'_, LinkedList<RabbitTradingEvent>> {
        self.data.read().await
    }
}

impl EventListenerTrait for LogContainerEventListener {
    fn new(config_map: ConfigMap) -> Self {
        LogContainerEventListener {
            config_map,
            data: Arc::new(RwLock::new(LinkedList::new())),
            stopped_indicator: Arc::new(AtomicBool::new(false)),
        }
    }

    fn get_identifier() -> String {
        const IDENTIFIER: &'static str = "LogContainerEventListener";
        IDENTIFIER.to_owned()
    }

    fn start(&self, receiver: Receiver<RabbitTradingEvent>) {
        tokio::task::spawn(Self::async_log_task(
            receiver,
            self.data.clone(),
            self.stopped_indicator.clone(),
        ));
    }

    fn stop(&self) -> Result<(), Error> {
        self.stopped_indicator.store(true, Ordering::Relaxed);
        Result::Ok(())
    }
}

impl Clone for LogContainerEventListener {
    fn clone(&self) -> Self {
        Self {
            config_map: self.config_map.clone(),
            // shallow clone the Arc pointer
            data: self.data.clone(),
            stopped_indicator: self.stopped_indicator.clone(),
        }
    }
}
