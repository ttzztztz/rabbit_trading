use super::event::event_bus::EventBus;
use crate::{
    broker::{common::broker::BrokerTrait, initializer::get_broker_instance},
    model::{
        common::error::Error,
        config::{pod::PodConfig, strategy::StrategyConfig},
    },
    pod::interceptor::factory::PodBrokerInterceptorCollectionFactory,
    strategy::common::strategy::StrategyContext,
};

pub struct Pod {
    pod_config: PodConfig,
    event_bus: EventBus,
}

impl Pod {
    pub fn new(pod_config: PodConfig) -> Self {
        let pod_id = pod_config.pod_id.clone();
        Pod {
            pod_config,
            event_bus: EventBus::new(pod_id),
        }
    }

    fn initialize_broker_list(&self) -> Result<Vec<Box<dyn BrokerTrait>>, Error> {
        const ILLEGAL_BROKER_ID: &'static str = "ILLEGAL_BROKER_ID";

        let broker_list: Vec<Box<dyn BrokerTrait>> = self
            .pod_config
            .broker_list
            .iter()
            .filter_map(|broker_config| {
                get_broker_instance(
                    Box::new(PodBrokerInterceptorCollectionFactory::new(
                        self.event_bus.clone(),
                    )),
                    broker_config.identifier.clone(),
                )
            })
            .collect();

        if broker_list.len() != self.pod_config.broker_list.len() {
            let broker_id_list: Vec<String> = self
                .pod_config
                .broker_list
                .iter()
                .map(|broker_config| broker_config.identifier.clone())
                .collect();

            return Result::Err(Error {
                code: ILLEGAL_BROKER_ID.to_owned(),
                message: broker_id_list.join(","),
            });
        }

        Result::Ok(broker_list)
    }

    fn initialize(&self) -> Result<(), Error> {
        let broker_list = self.initialize_broker_list()?;

        let strategy_context = StrategyContext::<String> {
            broker_list,
            persistent_kv_store: todo!(),
        };
    }

    pub fn start(&self) {
        self.initialize();
    }

    pub fn stop(&self) {
        todo!() // gracefully exit
    }
}
