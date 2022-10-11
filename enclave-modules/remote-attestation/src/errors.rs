use crate::prelude::*;
use flex_error::*;
use lcp_types::Time;
use sgx_types::sgx_status_t;

define_error! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    Error {
        TooOldReportTimestamp {
            now: Time,
            timestamp: Time
        }
        |e| {
            format_args!("TooOldReportTimestamp: the timestamp of the report is too old: now={:?} attestation_time={:?}", e.now, e.timestamp)
        },

        AttestationReport
        [attestation_report::Error]
        |_| { "AttestationReport error" },

        UnexpectedReport {
            descr: String
        }
        |e| {
            format_args!("UnexpectedReport error: {}", e.descr)
        },

        UnexpectedQuote {
            descr: String
        }
        |e| {
            format_args!("UnexpectedQuoteError: {}", e.descr)
        },

        SgxError {
            status: sgx_status_t,
            descr: String
        }
        |e| {
            format_args!("SGXError: status={:?} descr={}", e.status, e.descr)
        },

        Time
        [lcp_types::TimeError]
        |_| { "Time error" },

        HostApi
        [host_api::Error]
        |_| { "HostApi error" }
    }
}
