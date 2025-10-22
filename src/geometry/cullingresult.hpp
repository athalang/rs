#pragma once
#include <CesiumGeometry/CullingResult.h>

using CullingResult = CesiumGeometry::CullingResult;
static_assert(sizeof(CullingResult) == 4, "FFI assumes 32 bit int");
