use std::{mem::MaybeUninit, path::PathBuf};

use sgx_types::{metadata::metadata_t, *};
use sgx_urts::SgxEnclave;

pub fn create_enclave(path: impl Into<PathBuf>) -> SgxResult<SgxEnclave> {
    let mut launch_token: sgx_launch_token_t = [0; 1024];
    let mut launch_token_updated: i32 = 0;
    // call sgx_create_enclave to initialize an enclave instance
    // Debug Support: set 2nd parameter to 1
    let debug = 1;
    let mut misc_attr = sgx_misc_attribute_t {
        secs_attr: sgx_attributes_t { flags: 0, xfrm: 0 },
        misc_select: 0,
    };
    SgxEnclave::create(
        path.into(),
        debug,
        &mut launch_token,
        &mut launch_token_updated,
        &mut misc_attr,
    )
}

pub fn sgx_get_metadata(path: impl Into<PathBuf>) -> SgxResult<metadata_t> {
    let path = path.into();
    let enclave_path = std::ffi::CString::new(path.as_os_str().to_str().unwrap()).unwrap();
    let metadata = unsafe {
        let mut metadata: sgx_types::metadata::metadata_t = MaybeUninit::zeroed().assume_init();
        sgx_types::sgx_get_metadata(enclave_path.as_ptr(), &mut metadata);
        metadata
    };
    Ok(metadata)
}
