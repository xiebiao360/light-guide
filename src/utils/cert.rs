use anyhow::Result;
use rcgen::CertifiedKey;
use std::{fs::File, io::Write, path::Path};
use tokio::fs;

use crate::utils::get_local_ip;

/// Generate self-signed certificate
pub async fn generate_self_signed_file(cert_path: &str) -> Result<()> {
    fs::create_dir_all(cert_path).await?;
    let cert_file = Path::new(cert_path).join("registry.crt");
    let key_file = Path::new(cert_path).join("registry.key");
    if cert_file.exists() && key_file.exists() {
        return Ok(());
    }

    let subject_alt_names = vec![
        "localhost".to_string(),
        "127.0.0.1".to_string(),
        get_local_ip().unwrap_or_default(),
    ];
    let CertifiedKey { cert, key_pair } = rcgen::generate_simple_self_signed(subject_alt_names)?;

    println!("Generate self-signed certificate successfully!");
    // 保存证书
    let mut cert_file = File::create(cert_file)?;
    cert_file.write_all(cert.pem().as_bytes())?;

    println!("Save certificate successfully!");

    // 保存私钥
    let mut key_file = File::create(key_file)?;
    key_file.write_all(key_pair.serialize_pem().as_bytes())?;

    println!("Save private key successfully!");

    Ok(())
}
