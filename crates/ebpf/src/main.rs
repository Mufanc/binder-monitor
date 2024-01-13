#![no_std]
#![no_main]

use aya_bpf::{BpfContext, macros::tracepoint, programs::TracePointContext};
use aya_log_ebpf::info;

#[repr(C)]
struct BinderTransactionEvent {
    debug_id: i32,
    target_node: i32,
    to_proc: i32,
    to_thread: i32,
    reply: i32,
    code: u32,
    flags: u32
}

#[tracepoint]
pub fn binder_monitor(ctx: TracePointContext) -> u32 {
    run_catching(&ctx).unwrap_or_else(|code| {
        info!(&ctx, "binder-monitor failed with code: {}", code);
        code
    })
}

fn run_catching(ctx: &TracePointContext) -> Result<u32, u32> {
    let event: &BinderTransactionEvent = unsafe { &*(ctx.as_ptr().add(8) as *const _) };

    info!(ctx, "debug_id={}", event.debug_id);

    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
