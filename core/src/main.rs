use std::error::Error;
use libp2p::{futures::StreamExt, SwarmBuilder};
use reed_solomon_simd::ReedSolomonEncoder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🌌 V.O.I.D. Core Engine: Initializing...");

    // 1. Initialize 10/4 Resilience Math
    // 10 Original Shards + 4 Recovery Shards
    let encoder = ReedSolomonEncoder::new(10, 4, 64)?;
    println!("✅ 10/4 Reed-Solomon Math: ONLINE");

    // 2. Setup P2P Networking
    let mut swarm = SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )?
        .build();

    println!("✅ P2P Networking Layer: ONLINE");

    // 3. Main Loop (Waiting for Shards)
    println!("🚀 V.O.I.D. Node is now watching the mesh...");
    
    // This keeps the engine running forever
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
