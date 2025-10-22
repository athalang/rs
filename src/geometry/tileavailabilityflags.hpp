#pragma once
#include <CesiumGeometry/TileAvailabilityFlags.h>

using TileAvailabilityFlags = CesiumGeometry::TileAvailabilityFlags;
static_assert(sizeof(TileAvailabilityFlags) == 4, "FFI assumes 32 bit int");