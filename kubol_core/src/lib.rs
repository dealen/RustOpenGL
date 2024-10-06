pub mod logger;
pub mod custom_errors;
pub mod graphics {
    pub mod window;
    pub mod gl_wrappers;
}
// every file (module) needs to be declared here to be accessible from outside

pub fn check_if_working() -> String {
    "Hello, world! from kubol_core lib.".to_string()
}