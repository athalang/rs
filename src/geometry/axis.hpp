#pragma once
#include <CesiumGeometry/Axis.h>

using Axis = CesiumGeometry::Axis;
static_assert(sizeof(Axis) == 4, "FFI assumes 32 bit int");