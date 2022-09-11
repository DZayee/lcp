#[cfg(feature = "sgx")]
use crate::sgx_reexport_prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum LightClientHandlerError {
    #[error("ICS02Error: {0}")]
    ICS02Error(ibc::core::ics02_client::error::Error),
    #[error("LightClientError")]
    LightClientError(#[from] light_client::LightClientError),
    #[error("CommitmentError")]
    CommitmentError(#[from] commitments::CommitmentError),
    #[error("ICS24ValidationError: {0}")]
    ICS24ValidationError(ibc::core::ics24_host::error::ValidationError),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}
