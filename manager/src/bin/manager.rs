use env_logger::Env;
use std::env;
use std::time::Duration;

static LIB_FILES: [&str; 3] = ["libyajl.so.2", "libLLVM-12.so.1", "libwasmedge.so.0"];
static VENDOR_BASE: &str = "vendor";
static OCI_BASE: &str = "host_oci";
static LIB_BASE: &str = "host_lib";

fn main() -> Result<(), std::io::Error> {
    if env::args().count() >= 2 {
        println!("Manager 0.1 by Anton Whalley");
        std::process::exit(0);
    }
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let vendor = env::var("VENDOR").unwrap_or_else(|_| "ubuntu_20_04".to_string());
    for file_name in &LIB_FILES {
        manager::copy_to(VENDOR_BASE, LIB_BASE, &vendor, file_name)?
    }
    manager::copy_to(VENDOR_BASE, OCI_BASE, &vendor, "crun")?;

    manager::update_containerd_config("/host_containerd/config.toml")?;

    loop {
        std::thread::sleep(Duration::from_millis(1000));
    }
}
