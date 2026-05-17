/// tool-wallet — Polygon USDC payment watcher + receipt writer
///
/// Single-tenant (PointSav vendor). Watches for inbound USDC transfers to
/// POLYGON_WALLET_ADDRESS on Polygon PoS. On confirmation, writes a signed
/// receipt to service-fs (WORM ledger). app-privategit-marketplace checks
/// receipts to issue license keys and signed binary download URLs.
///
/// service-wallet (Doctrine claim #53) is the multi-tenant Ring 2 service
/// for customer-side Reverse-Flow revenue. tool-wallet is distinct: it is
/// a vendor-side utility for PointSav receiving inbound license payments.
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

// USDC contract on Polygon PoS (native USDC, not bridged)
const USDC_CONTRACT: &str = "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359";
// ERC-20 Transfer event topic
const TRANSFER_TOPIC: &str = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";

#[derive(Parser)]
#[command(name = "tool-wallet", about = "Polygon USDC payment watcher and receipt writer")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Watch for incoming USDC payments and write receipts to service-fs
    Watch(WatchArgs),
    /// Check whether a given tx_hash is a confirmed payment for a product
    Check(CheckArgs),
    /// Generate a fresh HD-derived payment address for a pending order
    Address(AddressArgs),
}

#[derive(Parser)]
struct WatchArgs {
    #[arg(env = "POLYGON_RPC_URL", long, help = "Polygon JSON-RPC endpoint")]
    rpc_url: String,
    #[arg(env = "POLYGON_WALLET_ADDRESS", long, help = "Receiving wallet address")]
    wallet_address: String,
    #[arg(env = "FS_ENDPOINT", long, default_value = "http://127.0.0.1:8020")]
    fs_endpoint: String,
    #[arg(env = "FS_MODULE_ID", long, default_value = "software")]
    fs_module_id: String,
    #[arg(long, default_value = "30", help = "Poll interval in seconds")]
    poll_secs: u64,
}

#[derive(Parser)]
struct CheckArgs {
    #[arg(help = "Transaction hash to verify")]
    tx_hash: String,
    #[arg(env = "POLYGON_RPC_URL", long)]
    rpc_url: String,
    #[arg(env = "POLYGON_WALLET_ADDRESS", long)]
    wallet_address: String,
}

#[derive(Parser)]
struct AddressArgs {
    #[arg(help = "Order ID — used to derive a unique per-order address")]
    order_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct LicenseReceipt {
    product_id: String,
    version: String,
    customer_ref: String,
    price_usdc: u64,     // 6-decimal USDC units
    tx_hash: String,
    chain: String,
    confirmed_at: String,
    block_number: u64,
    license_key: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    let cli = Cli::parse();

    match cli.command {
        Command::Watch(args) => watch(args).await,
        Command::Check(args) => check(args).await,
        Command::Address(args) => address(args).await,
    }
}

async fn watch(args: WatchArgs) -> Result<()> {
    tracing::info!(
        wallet = %args.wallet_address,
        poll_secs = args.poll_secs,
        "tool-wallet: watching for USDC transfers on Polygon PoS"
    );
    // Task Claude: implement polling loop using eth_getLogs against USDC_CONTRACT
    // Filter: topics[0] = TRANSFER_TOPIC, topics[2] = wallet_address (padded to 32 bytes)
    // On confirmed transfer: call confirm_and_write_receipt()
    // Poll interval: args.poll_secs seconds between eth_blockNumber calls
    tracing::warn!("watch loop not yet implemented — Task Claude to implement");
    Ok(())
}

async fn check(args: CheckArgs) -> Result<()> {
    // Task Claude: eth_getTransactionReceipt for args.tx_hash
    // Verify: to == USDC_CONTRACT, log topics match Transfer to wallet_address
    // Print confirmation status + USDC amount
    tracing::info!(tx_hash = %args.tx_hash, "checking transaction");
    tracing::warn!("check not yet implemented — Task Claude to implement");
    Ok(())
}

async fn address(args: AddressArgs) -> Result<()> {
    // Task Claude: HD-derive a per-order address from the operator-provisioned seed
    // Seed lives at $WALLET_SEED_PATH — never in code, never in git, never AI-visible
    // Use BIP-32 path m/44'/60'/0'/0/<order_index>
    tracing::info!(order_id = %args.order_id, "generating payment address");
    tracing::warn!("address derivation not yet implemented — Task Claude to implement");
    Ok(())
}

async fn _confirm_and_write_receipt(
    receipt: LicenseReceipt,
    fs_endpoint: &str,
    module_id: &str,
) -> Result<()> {
    // Task Claude: POST to service-fs /v1/append with:
    // - X-Foundry-Module-ID: module_id
    // - Body: serde_json::to_vec(&receipt)?
    // Path written: vault/source/license-receipts/<YYYY>/<MM>/<tx_hash>.json
    // The service-fs WORM ledger provides immutability + Sigstore Rekor anchoring.
    // service-bookkeeper in project-bookkeeping reads this path to emit journal entries.
    let _url = format!("{}/v1/append", fs_endpoint);
    let _payload = serde_json::to_vec(&receipt).context("serialize receipt")?;
    tracing::info!(tx_hash = %receipt.tx_hash, "receipt ready to write");
    Ok(())
}
