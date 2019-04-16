// Copyright (c) Microsoft. All rights reserved.

#![deny(unused_extern_crates, warnings)]
#![deny(clippy::all, clippy::pedantic)]

use std::env;
use std::fs::File;
use std::io::Write;
use tempfile::TempDir;

use edgelet_core::{Certificate, GetDeviceIdentityCertificate, KeyBytes, PrivateKey, Signature};
use edgelet_hsm::X509;

const HOMEDIR_KEY: &str = "IOTEDGE_HOMEDIR";
const DEVICE_IDENTITY_CERT_KEY: &str = "IOTEDGE_DEVICE_IDENTITY_CERT";
const DEVICE_IDENTITY_PK_KEY: &str = "IOTEDGE_DEVICE_IDENTITY_PK";
const DEVICE_IDENTITY_CERT: &str = "-----BEGIN CERTIFICATE-----\nMIICpDCCAYwCCQCgAJQdOd6dNzANBgkqhkiG9w0BAQsFADAUMRIwEAYDVQQDDAlsb2NhbGhvc3QwHhcNMTcwMTIwMTkyNTMzWhcNMjcwMTE4MTkyNTMzWjAUMRIwEAYDVQQDDAlsb2NhbGhvc3QwggEiMA0GCSqGSIb3DQEBAQUAA4IBDwAwggEKAoIBAQDlJ3fRNWm05BRAhgUY7cpzaxHZIORomZaOp2Uua5yv+psdkpv35ExLhKGrUIK1AJLZylnue0ohZfKPFTnoxMHOecnaaXZ9RA25M7XGQvw85ePlGOZKKf3zXw3Ds58GFY6Sr1SqtDopcDuMmDSg/afYVvGHDjb2Fc4hZFip350AADcmjH5SfWuxgptCY2Jl6ImJoOpxt+imWsJCJEmwZaXw+eZBb87e/9PH4DMXjIUFZebShowAfTh/sinfwRkaLVQ7uJI82Ka/icm6Hmr56j7U81gDaF0DhC03ds5lhN7nMp5aqaKeEJiSGdiyyHAescfxLO/SMunNc/eG7iAirY7BAgMBAAEwDQYJKoZIhvcNAQELBQADggEBACU7TRogb8sEbv+SGzxKSgWKKbw+FNgC4Zi6Fz59t+4jORZkoZ8W87NM946wvkIpxbLKuc4F+7nTGHHksyHIiGC3qPpi4vWpqVeNAP+kfQptFoWEOzxD7jQTWIcqYhvssKZGwDk06c/WtvVnhZOZW+zzJKXA7mbwJrfp8VekOnN5zPwrOCumDiRX7BnEtMjqFDgdMgs9ohR5aFsI7tsqp+dToLKaZqBLTvYwCgCJCxdg3QvMhVD8OxcEIFJtDEwm3h9WFFO3ocabCmcMDyXUL354yaZ7RphCBLd06XXdaUU/eV6fOjY6T5ka4ZRJcYDJtjxSG04XPtxswQfrPGGoFhk=\n-----END CERTIFICATE-----\n";
const DEVICE_IDENTITY_PK: &str = "ABCD";

#[test]
fn x509_get_identity_cert_success() {
    // arrange
    let home_dir = TempDir::new().unwrap();
    env::set_var(HOMEDIR_KEY, &home_dir.path());
    println!("IOTEDGE_HOMEDIR set to {:#?}", home_dir.path());

    let file_path = home_dir.path().join("temp_cert.pem");
    let mut cert_file = File::create(file_path.clone()).unwrap();
    write!(cert_file, "{}", DEVICE_IDENTITY_CERT).unwrap();
    env::set_var(DEVICE_IDENTITY_CERT_KEY, file_path);

    let file_path = home_dir.path().join("temp_key.pem");
    let mut key_file = File::create(file_path.clone()).unwrap();
    write!(key_file, "{}", DEVICE_IDENTITY_PK).unwrap();
    env::set_var(DEVICE_IDENTITY_PK_KEY, file_path);

    let x509 = X509::new().unwrap();

    let cert_info = x509.get().unwrap();

    assert!(cert_info.get_valid_to().is_ok());

    // assert that the configured test cert via env variable is received
    let buffer = cert_info.pem().unwrap();
    assert_eq!(DEVICE_IDENTITY_CERT, buffer);

    let pk = match cert_info.get_private_key().unwrap() {
        Some(pk) => pk,
        None => panic!("Expected to find a key"),
    };

    // assert that the configured test key via env variable is received
    match pk {
        PrivateKey::Ref(_) => panic!("did not expect reference private key"),
        PrivateKey::Key(KeyBytes::Pem(k)) => assert_eq!(DEVICE_IDENTITY_PK.as_bytes(), k.as_bytes()),
    }

    // cleanup
    drop(cert_file);
    drop(key_file);
    home_dir.close().unwrap();
}
