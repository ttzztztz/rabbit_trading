use std::{
    collections::HashMap,
    sync::{atomic::AtomicBool, Arc},
};

use crate::broker::{
    common::broker::EmptyBrokerInterceptorFactory, initializer::get_broker_instance,
};

#[test]
fn test_get_broker_instance() {
    const LONGBRIDGE_IDENTIFIER: &'static str = "longbridge";
    const YAHOO_FINANCE_IDENTIFIER: &'static str = "yahoo_finance";

    assert_eq!(
        cfg!(feature = "broker__longbridge"),
        get_broker_instance(
            LONGBRIDGE_IDENTIFIER.to_owned(),
            Box::new(EmptyBrokerInterceptorFactory::new()),
            HashMap::new(),
            Arc::new(AtomicBool::new(false)),
        )
        .is_ok()
    );
    assert_eq!(
        cfg!(feature = "broker__yahoo_finance"),
        get_broker_instance(
            YAHOO_FINANCE_IDENTIFIER.to_owned(),
            Box::new(EmptyBrokerInterceptorFactory::new()),
            HashMap::new(),
            Arc::new(AtomicBool::new(false)),
        )
        .is_ok()
    );
}
