#[cxx::bridge]
mod ffi {
    #[repr(u32)]
    enum TileAvailabilityFlags {
        TILE_AVAILABLE = 1,
        CONTENT_AVAILABLE = 2,
        SUBTREE_AVAILABLE = 4,
        SUBTREE_LOADED = 8,
        REACHABLE = 16
    }

    unsafe extern "C++" {
        include!("cesium-native-rs/src/geometry/tileavailabilityflags.hpp");
        type TileAvailabilityFlags;
    }
}