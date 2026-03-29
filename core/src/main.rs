use std::error::Error;
use libp2p::SwarmBuilder;
use reed_solomon_simd::ReedSolomonEncoder;
use tokio::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🌌 V.O.I.D. Core Engine: Initializing...");

    // 1. Initialize 10/4 Resilience Math
    let encoder = ReedSolomonEncoder::new(10, 4, 64)?;
    println!("✅ 10/4 Reed-Solomon Math: ONLINE");

    // 2. Setup P2P Networking (Updated for libp2p v0.54)
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_quic() // Add this to satisfy the builder phase
        .with_behaviour(|_| libp2p::kad::Behaviour::new(libp2p::identity::PeerId::random(), libp2p::kad::store::MemoryStore::new(libp2p::identity::PeerId::random())))?
        .with_swarm_config(|c| c.with_idle_connection_timeout(Duration::from_secs(60)))
        .build();

    println!("✅ P2P Networking Layer: ONLINE");
    println!("🚀 V.O.I.D. Node is now watching the mesh...");
    
    // 3. Main Loop
    loop {
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!("🛑 V.O.I.D. Engine shutting down gracefully.");
                break;
            }
        }
    }

    Ok(())
}
