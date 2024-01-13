#![no_std]
#![no_main]

use aya_bpf::{BpfContext, helpers, macros::tracepoint, programs::TracePointContext};
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

    let pid_tgid = helpers::bpf_get_current_pid_tgid();

    let current_pid = pid_tgid >> 32 & 0xFFFFFFFF;
    let current_tid = pid_tgid & 0xFFFFFFFF;

    let transaction_type = if event.flags & 1 != 0 {
        "(o)"
    } else if event.reply != 0 {
        "(r)"
    } else {
        "( )"
    };

    info!(ctx, "binder: {}:{}\t -> {}:{}\t {}", current_pid, current_tid, event.to_proc, event.to_thread, transaction_type);

    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
