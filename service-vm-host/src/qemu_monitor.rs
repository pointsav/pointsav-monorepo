use system_vm_fleet_types::VmRecord;

/// Poll all running QEMU processes via their UNIX monitor sockets and return VmRecords.
///
/// Phase 1: returns an empty Vec until QEMU monitor socket conventions are agreed.
/// Phase 2: scan /run/vm-*/monitor.sock and query each socket for VM metadata.
pub fn poll_running_vms() -> Vec<VmRecord> {
    // Phase 1 stub — enumerate monitor sockets when naming convention is settled.
    // The fleet controller builds its VM inventory from create/destroy API calls
    // combined with heartbeat VM lists; the absence of a list here is safe.
    vec![]
}
