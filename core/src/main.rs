use std::error::Error;
use libp2p::SwarmBuilder;
use reed_solomon_simd::ReedSolomonEncoder;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🌌 V.O.I.D. Core Engine: Initializing...");

    // 1. Initialize 10/4 Resilience Math
    let _encoder = ReedSolomonEncoder::new(10, 4, 64)?;
    println!("✅ 10/4 Reed-Solomon Math: ONLINE");

    // 2. Setup P2P Networking (Pure TCP for 100% Cross-Platform Compilation)
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|_| libp2p::swarm::dummy::Behaviour)? 
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    let peer_id = *swarm.local_peer_id();
    println!("✅ P2P Networking Layer: ONLINE (ID: {})", peer_id);
    println!("🚀 V.O.I.D. Node is now watching the mesh...");
    
    // 3. Graceful Shutdown
    tokio::signal::ctrl_c().await?;
    println!("🛑 V.O.I.D. Engine shutting down gracefully.");

    Ok(())
}
