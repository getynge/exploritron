#![allow(non_upper_case_globals)]

#[cfg(feature = "client")]
use lazy_static::lazy_static;
#[cfg(feature = "client")]
use libloading::Library;

pub mod v0;

#[cfg(all(feature = "client", target_os = "windows"))]
const DYLIB_PATH: &str = "render.dll";

#[cfg(all(feature = "client", any(target_os = "macos", target_os = "ios")))]
const DYLIB_PATH: &str = "render.dylib";

#[cfg(all(feature = "client", not(any(target_os = "windows", target_os = "macos", target_os = "ios"))))]
const DYLIB_PATH: &str = "render.so";

#[cfg(feature = "client")]
lazy_static!{
    static ref library: Library = unsafe { Library::new(DYLIB_PATH).expect("failed to load library") };
}

/// Render state is a blob that server use to persist state between calls.
/// The engine in no way guarantees that a server will remain in memory indefinitely, therefore state should be stored here.
pub type RenderState = ();

/// ExtensionMetadata is a blob that contains the metadata for any enabled extension
pub type ExtensionMetadata = ();

/// c_array indicates that a pointer refers to an array, not just a single item
#[allow(non_camel_case_types)]
pub type c_array<T> = *mut T;

/// IncomingMetadata is a struct supplying metadata for the target server
/// # Fields
/// * client_version is the version of the core client
/// * supported_versions is a list of server versions the client supports
/// * supported_extensions is a list of extensions supported by the client
/// * enabled_extensions lists which of the supported extensions are enabled
///   note that enabled_extensions\[some x\] will always be equal to supported_extensions\[some y\]
/// * extension_metadata is an array of metadata for each extension
///   note that extension_metadata\[idx\] refers to extension at enabled_extensions\[idx\]
///
/// # Notes
/// This struct may change while on version 0 of the API.
/// Once version 1 is released this struct definition will not change
#[repr(C)]
pub struct IncomingMetadata {
    client_version: *mut i8,
    supported_versions: c_array<*mut i8>,
    supported_versions_length: isize,
    supported_extensions: c_array<*mut i8>,
    supported_extensions_length: isize,
    enabled_extensions: c_array<*mut i8>,
    enabled_extensions_length: isize,
    extension_metadata: c_array<ExtensionMetadata>
}

/// InitResult is a struct consisting of all necessary information for the client to utilize the server
/// # Fields
/// * server_version is the version of the render API the server is using
/// * server_state is the render state the server uses as persistent memory
/// * server_extensions is a list of extensions to the render API protocol the server can respond to,
///   these extensions determine additional functions callable from the client to the server
/// * accepted_extensions is the list of client extensions the server can display,
///   these extensions determine additional information a client can send through v*::state::State
/// * error is a description of the error the occured during initialization, if any
///
/// # Notes
/// * This struct may change while on version 0 of the API.
///   Once version 1 is released this struct definition will not change
/// * If error is non-null that indicates an error occurred
/// * If server_version or server_state is null that means the error is unrecoverable
#[repr(C)]
pub struct InitResult {
    server_version: *mut i8,
    server_state: *mut RenderState,
    server_extensions: c_array<*mut i8>,
    server_extensions_length: isize,
    accepted_extensions: c_array<*mut i8>,
    accepted_extensions_length: isize,
    error: *mut i8
}

/// expr_init initializes the render server
/// the reported server version is the protocol the client will use if the version is supported
///
/// The supplied InitResult contains information from implementing expr_init including any errors
/// that occured
#[cfg(feature = "client")]
pub unsafe fn init(client_metadata: IncomingMetadata) -> InitResult {
    library.get::<unsafe extern fn(IncomingMetadata) -> InitResult>(b"expr_init")
        .expect("failed to load expr_init")(client_metadata)
}

