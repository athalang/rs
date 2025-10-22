#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    enum CullingResult {
        Outside = -1,
        Intersecting = 0,
        Inside = 1
    }

    unsafe extern "C++" {
        include!("cesium-native-rs/src/geometry/cullingresult.hpp");
        type CullingResult;
    }
}
