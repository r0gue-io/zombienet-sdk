use configuration::NetworkConfigBuilder;
use orchestrator::{AddNodeOpts, Orchestrator};
use provider::NativeProvider;
use support::{fs::local::LocalFileSystem, process::os::OsProcessManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = NetworkConfigBuilder::new()
        .with_relaychain(|r| {
            r.with_chain("rococo-local")
                .with_default_command("polkadot")
                .with_node(|node| node.with_name("alice"))
                .with_node(|node| node.with_name("bob"))
        })
        .with_parachain(|p| {
            p.with_id(100)
                .cumulus_based(true)
                .with_collator(|n| n.with_name("collator").with_command("polkadot-parachain"))
        })
        .build()
        .unwrap();

    let fs = LocalFileSystem;
    let pm = OsProcessManager;
    let provider = NativeProvider::new(fs.clone(), pm);
    let orchestrator = Orchestrator::new(fs, provider);
    let mut network = orchestrator.spawn(config).await?;
    println!("🚀🚀🚀🚀 network deployed");
    // add  a new node
    let opts = AddNodeOpts {
        rpc_port: Some(9444),
        is_validator: true,
        ..Default::default()
    };

    // TODO: add check to ensure if unique
    network.add_node("new1", opts, None).await?;

    // Example of some opertions that you can do
    // with `nodes` (e.g pause, resume, restart)

    // Get a ref to the node
    let node = network.get_node("alice")?;

    let is_10 = node.assert("block_height{status=\"best\"}", 10).await?;
    println!("is_10: {is_10}");

    let role = node.reports("node_roles").await?;
    println!("Role is {role}");

    // pause the node
    // node.pause().await?;
    // println!("node new1 paused!");

    // node.resume().await?;
    // println!("node new1 resumed!");

    let col_opts = AddNodeOpts {
        command: Some("polkadot-parachain".try_into()?),
        ..Default::default()
    };
    network.add_node("new-col-1", col_opts, Some(100)).await?;
    println!("new collator deployed!");

    // For now let just loop....
    #[allow(clippy::empty_loop)]
    loop {}

    // Ok(())
}
