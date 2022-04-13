#[cfg(feature = "sgx")]
use crate::sgx_reexport_prelude::*;
use commitments::StateID;
use core::convert::Infallible;
use ibc::core::{
    ics02_client::{client_consensus::AnyConsensusState, client_type::ClientType},
    ics23_commitment::commitment::CommitmentRoot,
};

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct ConsensusState {
    pub state_id: StateID,
    pub timestamp: u64,
}

impl ibc::core::ics02_client::client_consensus::ConsensusState for ConsensusState {
    type Error = Infallible;

    fn client_type(&self) -> ClientType {
        todo!()
    }

    fn root(&self) -> &CommitmentRoot {
        panic!("not supported")
    }

    fn wrap_any(self) -> AnyConsensusState {
        panic!("not supported")
    }
}