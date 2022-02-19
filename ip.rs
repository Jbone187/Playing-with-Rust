use std::process::Command;

fn main() {
    let data = Command::new("curl")
        .arg("-s")
        .arg("http://whatismyip.akamai.com/")
        .output()
        .expect("Error");

    let ip = "#######";
    let value = String::from_utf8_lossy(&data.stdout);

    if value == ip {
        println!("External_IP: {}", value);
    } else {
        println!("No Data")
    };
}
