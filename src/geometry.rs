#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("geometry.h");
    }
}