//! Query user-specific fee rates on Hyperliquid.
//!
//! This example demonstrates how to retrieve a user's effective maker/taker rates
//! and active referral discount.
//!
//! # Usage
//!
//! ```bash
//! cargo run --example user_fees -- <USER_ADDRESS>
//! ```

use clap::Parser;
use hypersdk::{Address, hypercore};

#[derive(Parser)]
struct Args {
    /// User address to query fee rates for
    user: Address,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = hypercore::mainnet();

    let fees = client.user_fees(args.user).await?;

    println!("User: {:?}", args.user);
    println!("Daily volume: {}", fees.daily_user_vlm);
    println!("Perp maker rate: {}", fees.user_add_rate);
    println!("Perp taker rate: {}", fees.user_cross_rate);
    if let Some(rate) = fees.user_spot_add_rate {
        println!("Spot maker rate: {}", rate);
    }
    if let Some(rate) = fees.user_spot_cross_rate {
        println!("Spot taker rate: {}", rate);
    }
    println!("Referral discount: {}", fees.active_referral_discount);

    Ok(())
}
