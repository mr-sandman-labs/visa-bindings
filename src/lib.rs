#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
include!("./bindings_x86_64_unknown_linux_gnu.rs");
