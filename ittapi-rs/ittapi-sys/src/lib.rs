#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#![allow(unused)]

// The ITT bindings are OS-specific: they contain OS-specific constants (e.g. `ITT_OS` and
// `ITT_PLATFORM`) and some of the Windows structure sizes are different. Because of this, we
// generate bindings separately for each OS. TODO handle unsupported OSes gracefully here.
#[cfg(target_os = "linux")]
include!("linux/ittnotify_bindings.rs");
#[cfg(target_os = "macos")]
include!("macos/ittnotify_bindings.rs");
#[cfg(target_os = "windows")]
include!("windows/ittnotify_bindings.rs");

// The JIT profiling bindings are almost OS-agnostic, but slight differences with `c_uint` vs
// `c_int`, e.g., force us to use separate bindings.
#[cfg(target_os = "linux")]
include!("linux/jitprofiling_bindings.rs");
#[cfg(target_os = "macos")]
include!("macos/jitprofiling_bindings.rs");
#[cfg(target_os = "windows")]
include!("windows/jitprofiling_bindings.rs");