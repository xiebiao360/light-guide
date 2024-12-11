mod cert;

pub use cert::generate_self_signed_file;

/// 获取局域网IP地址
pub fn get_local_ip() -> Option<String> {
    let interfaces = get_if_addrs::get_if_addrs().unwrap();
    for iface in interfaces {
        if iface.is_loopback() {
            continue;
        }
        if iface.ip().is_ipv4() {
            return Some(iface.ip().to_string());
        }
    }
    None
}
