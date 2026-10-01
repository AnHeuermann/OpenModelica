// Auto-generated lib file
#![recursion_limit = "1024"]
#[cfg(not(any(target_arch = "wasm32", not(feature = "curl"))))]
pub mod Curl;
#[cfg(any(target_arch = "wasm32", not(feature = "curl")))]
#[path = "Curl_wasm.rs"]
pub mod Curl;
pub mod DynLoad;
pub mod DynLoadExt;
pub mod GlobalScriptUtil;
pub mod PackageManagement;
pub mod SimulationResults;
pub mod UnitParserExt;
pub mod Unzip;
