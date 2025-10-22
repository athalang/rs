fn main() {
    cxx_build::bridges([
        "src/geometry/axis.rs",
        "src/geometry/cullingresult.rs",
        "src/geometry/tileavailabilityflags.rs",
    ])
    .files([
        "src/geometry/axis.hpp",
        "src/geometry/cullingresult.hpp",
        "src/geometry/tileavailabilityflags.hpp",
    ])
    .std("c++20")
    .compile("cesium-native-rs");
}