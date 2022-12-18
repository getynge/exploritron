use std::collections::HashMap;

pub mod v0;

/// Render state is a blob that renderers use to persist state between calls.
/// The engine in no way guarantees that a renderer will remain in memory indefinitely, therefore state should be stored here.
pub type RenderState = ();

/// ExtensionMetadata is a blob that contains the metadata for any enabled extension
pub type ExtensionMetadata = ();

/// IncomingMetadata is a struct supplying metadata for the target renderer
/// # Fields
/// * client_version is the version of the core client
/// * supported_versions is a list of renderer versions the client supports
/// * supported_extensions is a list of extensions supported by the client
/// * enabled_extensions lists which of the supported extensions are enabled
/// * extension_metadata is a hashmap of each piece of metadata an extension requires
///
/// # Notes
/// This struct may change while on version 0 of the API.
/// Once version 1 is released this struct definition will not change
#[repr(C)]
pub struct IncomingMetadata {
    client_version: *mut i8,
    supported_versions: *mut [*mut i8],
    supported_extensions: *mut [isize],
    enabled_extensions: *mut [*mut i8],
    extension_metadata: HashMap<*mut i8, *mut ExtensionMetadata>
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
    server_extensions: *mut [*mut i8],
    accepted_extensions: *mut [*mut i8],
    error: *mut i8
}

extern {
    /// expr_init initializes the render server
    /// the reported server version is the protocol the client will use if the version is supported
    ///
    /// The supplied InitResult
    pub fn expr_init(client_metadata: IncomingMetadata) -> InitResult;
}
