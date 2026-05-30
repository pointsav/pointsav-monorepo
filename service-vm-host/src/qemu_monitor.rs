use std::{
    io::{BufRead, BufReader, Write},
    os::unix::net::UnixStream,
    time::Duration,
};
use system_vm_fleet_types::{VmRecord, VmState};

/// Poll all running QEMU processes via their UNIX monitor sockets.
///
/// Scans `VM_DISK_DIR` (default: `/var/lib/vm-fleet`) for `*.monitor.sock` files.
/// For each socket: connects, exchanges QMP handshake, queries running state.
/// Sockets that don't respond within 500 ms are skipped silently.
pub fn poll_running_vms() -> Vec<VmRecord> {
    let disk_dir = std::env::var("VM_DISK_DIR")
        .unwrap_or_else(|_| "/var/lib/vm-fleet".to_string());

    let read_dir = match std::fs::read_dir(&disk_dir) {
        Ok(rd) => rd,
        Err(_) => return vec![],
    };

    read_dir
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            let name = path.file_name()?.to_str()?.to_owned();
            let vm_id = name.strip_suffix(".monitor.sock")?.to_owned();
            query_qmp_socket(&path, &vm_id)
        })
        .collect()
}

fn query_qmp_socket(path: &std::path::Path, vm_id: &str) -> Option<VmRecord> {
    let timeout = Duration::from_millis(500);
    let stream = UnixStream::connect(path).ok()?;
    stream.set_read_timeout(Some(timeout)).ok()?;
    stream.set_write_timeout(Some(timeout)).ok()?;

    let mut writer = stream.try_clone().ok()?;
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    // Read QMP greeting banner.
    reader.read_line(&mut line).ok()?;
    line.clear();

    // Negotiate capabilities.
    writer.write_all(b"{\"execute\":\"qmp_capabilities\"}\n").ok()?;
    reader.read_line(&mut line).ok()?;
    line.clear();

    // Query running state.
    writer.write_all(b"{\"execute\":\"query-status\"}\n").ok()?;
    reader.read_line(&mut line).ok()?;

    // QEMU reports {"return":{"running":true,...}} when the VM is running.
    if !line.contains("\"running\":true") {
        return None;
    }

    Some(VmRecord {
        vm_id: vm_id.to_string(),
        vm_type: "unknown".to_string(),
        state: VmState::Running,
        ram_alloc_mb: 0,
        vcpu_count: 0,
        started_at: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_or_absent_disk_dir_returns_empty() {
        std::env::set_var("VM_DISK_DIR", "/tmp/nonexistent-vm-monitor-test-dir");
        let vms = poll_running_vms();
        assert!(vms.is_empty());
    }

    #[test]
    fn dir_with_no_sockets_returns_empty() {
        let dir = "/tmp/vm-monitor-test-no-socks";
        std::fs::create_dir_all(dir).unwrap();
        std::env::set_var("VM_DISK_DIR", dir);
        let vms = poll_running_vms();
        assert!(vms.is_empty());
        let _ = std::fs::remove_dir(dir);
    }
}
