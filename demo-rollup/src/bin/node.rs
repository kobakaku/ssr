use anyhow::Context;
use tracing::info;

#[cfg(feature = "celestia")]
use demo_rollup::celestia_rollup::CelestiaRollup;
#[cfg(feature = "mock")]
use demo_rollup::mock_rollup::MockRollup;

use demo_rollup::initialize_logging;
use sov_modules_rollup_blueprint::{Rollup, RollupBlueprint};
use sov_stf_runner::{from_toml_path, RollupConfig};

#[cfg(feature = "mock")]
const CONFIG_PATH: &'static str = "./mock_rollup_config.toml";
#[cfg(feature = "celestia")]
const CONFIG_PATH: &'static str = "./celestia_rollup_config.toml";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    initialize_logging();

    let rollup = new_rollup(CONFIG_PATH).await?;

    info!("Starting Mock Rollup with config: {}", CONFIG_PATH);

    rollup.run().await
}

#[cfg(feature = "mock")]
async fn new_rollup(rollup_config_path: &str) -> anyhow::Result<Rollup<MockRollup>> {
    use sov_mock::MockDaConfig;

    let rollup_config: RollupConfig<MockDaConfig> =
        from_toml_path(rollup_config_path).context("Failed to read rollup configuration")?;
    let rollup = MockRollup {};
    rollup.create_new_rollup(rollup_config).await
}

#[cfg(feature = "celestia")]
async fn new_rollup(rollup_config_path: &str) -> anyhow::Result<Rollup<CelestiaRollup>> {
    use sov_celestia::CelestiaDaConfig;

    let rollup_config: RollupConfig<CelestiaDaConfig> =
        from_toml_path(rollup_config_path).context("Failed to read rollup configuration")?;
    let rollup = CelestiaRollup {};
    rollup.create_new_rollup(rollup_config).await
}
