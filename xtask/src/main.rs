use std::process::Command;
use std::fs;
use std::path::Path;

fn main() {
    println!("============================================================");
    println!("[*] PointSav Digital Systems: Tier-6 Build Orchestrator");
    println!("============================================================");

    // Phase 1: Compile Sovereign Payloads
    build_library_payload("system-security");
    build_baremetal_payload("service-fs");
    println!("[+] Phase 1 Complete: Core & Unikernel secured.");
    println!("------------------------------------------------------------");

    // Phase 2: Assembly & Deployment Routing (REALIGNED TO PERSONNEL-1)
    let deploy_dir = "/home/mathew/deployments/woodfine-fleet-deployment/cluster-totebox-personnel-1/os-totebox-release";
    assemble_and_route(deploy_dir);

    println!("============================================================");
    println!("[+] PIPELINE COMPLETE. Ready for GCP push.");
    println!("============================================================");
}

fn build_library_payload(crate_name: &str) {
    println!("[*] Invoking cross-compiler for Root-Task: {} (staticlib)", crate_name);
    let status = Command::new("cargo")
        .args(["build", "--release", "--package", crate_name, "--target", "x86_64-unknown-none"])
        .status()
        .expect("[-] FATAL: Failed to invoke cargo compiler.");

    if !status.success() {
        eprintln!("[-] FATAL: Compilation of {} failed.", crate_name);
        std::process::exit(1);
    }
}

fn build_baremetal_payload(crate_name: &str) {
    println!("[*] Invoking cross-compiler for Unikernel: {} (ELF)", crate_name);
    let status = Command::new("cargo")
        .args(["build", "--release", "--package", crate_name, "--target", "x86_64-unknown-none"])
        .status()
        .expect("[-] FATAL: Failed to invoke cargo compiler.");

    if !status.success() {
        eprintln!("[-] FATAL: Compilation of {} failed.", crate_name);
        std::process::exit(1);
    }
}

fn assemble_and_route(deploy_dir: &str) {
    println!("[*] Initializing Phase 2: Microkernel Assembly & Routing");
    
    fs::create_dir_all(deploy_dir).expect("[-] FATAL: Could not access Customer deployment zone.");

    let kernel_src = "vendor-sel4-kernel/build/kernel.elf";
    let root_task_src = "target/x86_64-unknown-none/release/libsystem_security.a";
    let unikernel_src = "target/x86_64-unknown-none/release/service-fs";

    if !Path::new(kernel_src).exists() {
        eprintln!("[-] FATAL: C-Kernel (kernel.elf) missing. Run Ninja build first.");
        std::process::exit(1);
    }

    println!("[*] Packaging artifacts into release boundary...");
    
    fs::copy(kernel_src, format!("{}/kernel.elf", deploy_dir)).expect("[-] FATAL: Kernel routing failed.");
    fs::copy(root_task_src, format!("{}/libsystem_security.a", deploy_dir)).expect("[-] FATAL: Root-Task routing failed.");
    fs::copy(unikernel_src, format!("{}/service-fs.elf", deploy_dir)).expect("[-] FATAL: Unikernel routing failed.");

    println!("[+] SUCCESS: Artifacts verified and routed to Customer Deployment Zone.");
    println!("    -> Location: {}", deploy_dir);
}
