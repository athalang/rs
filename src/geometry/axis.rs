#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    enum Axis {
        X,
        Y,
        Z
    }

    unsafe extern "C++" {
        include!("cesium-native-rs/src/geometry/axis.hpp");
        type Axis;
    }
}