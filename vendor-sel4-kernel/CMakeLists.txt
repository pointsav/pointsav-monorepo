cmake_minimum_required(VERSION 3.18)

# 1. HARDWARE IDENTITY (Target: Laptop B)
set(KernelPlatform "pc99" CACHE STRING "")
set(KernelSel4Arch "x86_64" CACHE STRING "")

project(PointSav-Infrastructure C ASM)

# 2. MASTER INITIALIZATION
set(BASE_DIR "/home/mathew/Foundry/factory-pointsav/pointsav-monorepo/vendor-sel4-kernel")
set(CMAKE_TOOL_PATH "${BASE_DIR}/tools/cmake-tool")
list(APPEND CMAKE_MODULE_PATH "${CMAKE_TOOL_PATH}/helpers")

# This script automatically discovers 'kernel' and 'projects'
include(${CMAKE_TOOL_PATH}/all.cmake)

# 3. EXPLICIT HELPER LOADING
# Pointing to our surgically provisioned forge logic
include("${BASE_DIR}/projects/elfloader-tool/elfloader-tool.cmake")

# 4. SOVEREIGN HANDSHAKE
# Fusing the 'os-infrastructure' binary from your Rust workspace
set(RootTaskBinary "${BASE_DIR}/../target/x86_64-unknown-none/release/system-substrate")
add_custom_target(pointsav_root_task DEPENDS ${RootTaskBinary})
set_property(TARGET pointsav_root_task PROPERTY ROOTSERVER_BINARY ${RootTaskBinary})

# 5. FUSE & FORGE
DeclareRootserver(pointsav_root_task)
GenerateElfImage()
