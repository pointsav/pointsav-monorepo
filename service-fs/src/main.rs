use warp::Filter;
use bytes::Bytes;
use std::fs;

const TOTEBOX_ROOT: &str = "/opt/woodfine/cluster-totebox-personnel-1";
const FS_PORT: u16 = 8095;

#[tokio::main]
async fn main() {
    println!("========================================================");
    println!(" 🗄️ SERVICE-FS: TOTEBOX STORAGE GATEKEEPER ACTIVE");
    println!("========================================================");
    println!("[SYSTEM] Listening for internal write intents on 127.0.0.1:{}", FS_PORT);

    // Ensure physical boundaries exist
    fs::create_dir_all(format!("{}/service-email/maildir/new", TOTEBOX_ROOT)).unwrap();
    fs::create_dir_all(format!("{}/service-email/maildir/cur", TOTEBOX_ROOT)).unwrap();

    let ingress_route = warp::post()
        .and(warp::path!("vault" / "ingress"))
        .and(warp::header::<String>("x-file-name"))
        .and(warp::body::bytes())
        .map(|filename: String, body: Bytes| {
            let safe_name = filename.replace("/", "_").replace("\\", "_");
            let path = format!("{}/service-email/maildir/new/{}", TOTEBOX_ROOT, safe_name);
            
            match fs::write(&path, body) {
                Ok(_) => {
                    println!("[SECURED] Base Asset locked to disk: {}", safe_name);
                    warp::reply::with_status("SECURED", warp::http::StatusCode::OK)
                },
                Err(e) => {
                    eprintln!("[FATAL] Disk write failure: {}", e);
                    warp::reply::with_status("FAULT", warp::http::StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        });

    warp::serve(ingress_route).run(([127, 0, 0, 1], FS_PORT)).await;
}
