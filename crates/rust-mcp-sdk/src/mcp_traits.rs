#[cfg(feature = "client")]
pub mod mcp_client;
#[cfg(any(feature = "server", feature = "client"))]
pub mod mcp_handler;
#[cfg(feature = "server")]
pub mod mcp_server;
