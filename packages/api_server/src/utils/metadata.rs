use rabbit_trading_core::{model::config::pod::PodConfig, utils::time::get_now_unix_timestamp};
use std::sync::Arc;

use super::id_generator::common_trait::IdGeneratorTrait;
use crate::model::pod::metadata::PodMetadata;

pub fn generate_pod_metadata(
    id_generator: Arc<Box<dyn IdGeneratorTrait>>,
    config: PodConfig,
) -> PodMetadata {
    let pod_id = id_generator.generate();
    let created_at = get_now_unix_timestamp();

    PodMetadata {
        id: pod_id,
        created_at,
        config,
    }
}
