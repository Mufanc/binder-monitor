use aya::programs::TracePoint;
use aya::{include_bytes_aligned, Bpf};
use aya_log::BpfLogger;
use log::{info, warn, debug};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    // Bump the memlock rlimit. This is needed for older kernels that don't use the
    // new memcg based accounting, see https://lwn.net/Articles/837122/
    let rlim = libc::rlimit { rlim_cur: libc::RLIM_INFINITY, rlim_max: libc::RLIM_INFINITY, };
    let code = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };

    if code != 0 {
        debug!("remove limit on locked memory failed with code: {}", code);
    }

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!(concat!(
        env!("CARGO_RUSTC_CURRENT_DIR"),
        "/target/bpfel-unknown-none/debug/binder-monitor"
    )))?;

    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!(concat!(
        env!("CARGO_RUSTC_CURRENT_DIR"),
        "/target/bpfel-unknown-none/release/binder-monitor"
    )))?;

    if let Err(err) = BpfLogger::init(&mut bpf) {
        // This can happen if you remove all log statements from your eBPF program.
        warn!("failed to initialize eBPF logger: {}", err);
    }

    let program: &mut TracePoint = bpf.program_mut("binder_monitor").unwrap().try_into()?;
    program.load()?;
    let id = program.attach("binder", "binder_transaction")?;

    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;

    info!("Exiting...");
    program.detach(id)?;

    Ok(())
}
