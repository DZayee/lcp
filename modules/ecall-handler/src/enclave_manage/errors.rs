#[cfg(feature = "sgx")]
use crate::sgx_reexport_prelude::*;
use sgx_types::sgx_status_t;
use std::string::String;

#[derive(thiserror::Error, Debug)]
pub enum EnclaveManageError {
    #[error("SGXError: {0}")]
    SGXError(sgx_status_t),
    #[error("EnclaveKeyNotFound")]
    EnclaveKeyNotFound,
    #[error("CryptoError")]
    CryptoError(#[from] crypto::CryptoError),
    #[error("AttestationReportError")]
    AttestationReportError(#[from] attestation_report::AttestationReportError),
    #[error("RemoteAttestationError")]
    RemoteAttestationError(#[from] enclave_remote_attestation::errors::RemoteAttestationError),
    #[error("TimeError")]
    TimeError(#[from] lcp_types::TimeError),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}