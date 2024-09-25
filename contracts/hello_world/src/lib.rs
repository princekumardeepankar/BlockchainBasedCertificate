#![allow(non_snake_case)]
#![no_std]

use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Map, String};

// Struct to store certificate details
#[contracttype]
#[derive(Clone)]
pub struct Certificate {
    pub cert_id: u64,         // Unique certificate ID
    pub recipient: String,    // Recipient's name
    pub issuer: String,       // Issuer's name (e.g., educational institution or employer)
    pub course: String,       // Course or title of the certificate
    pub issue_date: u64,      // Timestamp of when the certificate was issued
    pub valid: bool,          // Is the certificate valid?
}

// Struct to keep track of the certificate issuance status
#[contracttype]
#[derive(Clone)]
pub struct CertificateStatus {
    pub total_issued: u64,  // Total certificates issued
    pub total_revoked: u64, // Total certificates revoked
}

// Constants for tracking status
const COUNT_CERT: &str = "C_CERT";
const CERT_STATUS: &str = "CERT_STATUS";

#[contract]
pub struct CertificateContract;

#[contractimpl]
impl CertificateContract {

    // Function to issue a new certificate
    pub fn issue_certificate(
        env: Env,
        recipient: String,
        issuer: String,
        course: String
    ) -> u64 {
        let mut cert_count: u64 = env.storage().instance().get(&COUNT_CERT).unwrap_or(0);
        cert_count += 1;

        let issue_date = env.ledger().timestamp(); // Get the current timestamp

        // Create the certificate
        let cert = Certificate {
            cert_id: cert_count.clone(),
            recipient: recipient.clone(),
            issuer: issuer.clone(),
            course: course.clone(),
            issue_date: issue_date,
            valid: true, // Initially valid
        };

        // Store the certificate in a Map with u64 as the key
        let mut certs_map: Map<u64, Certificate> = env.storage().instance().get(&"CERT_MAP").unwrap_or(Map::new(&env));
        certs_map.set(cert_count, cert);
        env.storage().instance().set(&"CERT_MAP", &certs_map);

        // Update the certificate status
        let mut cert_status = Self::view_cert_status(env.clone());
        cert_status.total_issued += 1;
        env.storage().instance().set(&CERT_STATUS, &cert_status);

        // Update the certificate count
        env.storage().instance().set(&COUNT_CERT, &cert_count);

        log!(&env, "Certificate ID: {} issued to {}", cert_count, recipient);
        cert_count
    }

    // Function to revoke a certificate
    pub fn revoke_certificate(env: Env, cert_id: u64) {
        let mut certs_map: Map<u64, Certificate> = env.storage().instance().get(&"CERT_MAP").unwrap_or(Map::new(&env));
        let mut cert = certs_map.get(cert_id).unwrap_or_else(|| panic!("Certificate ID not found"));
        
        if cert.valid == false {
            log!(&env, "Certificate ID: {} is already revoked!", cert_id);
            panic!("Certificate already revoked!");
        }

        cert.valid = false; // Revoke the certificate

        // Update the certificate in the map
        certs_map.set(cert_id, cert);
        env.storage().instance().set(&"CERT_MAP", &certs_map);

        // Update the certificate status
        let mut cert_status = Self::view_cert_status(env.clone());
        cert_status.total_revoked += 1;
        env.storage().instance().set(&CERT_STATUS, &cert_status);

        log!(&env, "Certificate ID: {} has been revoked", cert_id);
    }

    // Function to verify if a certificate is valid
    pub fn verify_certificate(env: Env, cert_id: u64) -> bool {
        let certs_map: Map<u64, Certificate> = env.storage().instance().get(&"CERT_MAP").unwrap_or(Map::new(&env));
        let cert = certs_map.get(cert_id).unwrap_or_else(|| panic!("Certificate ID not found"));
        
        if cert.valid {
            log!(&env, "Certificate ID: {} is valid", cert_id);
            true
        } else {
            log!(&env, "Certificate ID: {} is not valid", cert_id);
            false
        }
    }

    // View function to get the details of a certificate by its ID
    pub fn view_certificate(env: Env, cert_id: u64) -> Certificate {
        let certs_map: Map<u64, Certificate> = env.storage().instance().get(&"CERT_MAP").unwrap_or(Map::new(&env));
        certs_map.get(cert_id).unwrap_or_else(|| panic!("Certificate ID not found"))
    }

    // View function to get the status of certificate issuance
    pub fn view_cert_status(env: Env) -> CertificateStatus {
        env.storage().instance().get(&CERT_STATUS).unwrap_or(CertificateStatus {
            total_issued: 0,
            total_revoked: 0,
        })
    }
}
