use std::net::UdpSocket;
use std::time::Duration;
use std::io;

fn main() -> io::Result<()> {
    let laptop_ip = "10.0.0.101:5000";
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;

    println!("🏛️ PointSav Command Authority: Connecting to Laptop B...");

    loop {
        // Send Telemetry Request
        socket.send_to(b"GET_VITALS", laptop_ip)?;

        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf) {
            Ok((amt, _)) => {
                let response = String::from_utf8_lossy(&buf[..amt]);
                print!("\r📡 Status: {}    ", response);
                use std::io::Write;
                io::stdout().flush().unwrap();
            }
            Err(_) => println!("\r⚠️ Timeout: Substrate link interrupted. Check Lid Status."),
        }
        
        std::thread::sleep(Duration::from_millis(1000));
    }
}
