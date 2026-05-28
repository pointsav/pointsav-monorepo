#
# Minimal seL4 rootserver — Phase 1C.c hello-world
#
cmake_minimum_required(VERSION 3.16.0)

project(hello-rootserver C)

# seL4 rootserver: must be named according to seL4 build system conventions.
# The rootserver is the initial user-space thread that seL4 starts.
add_executable(hello-rootserver main.c)

target_link_libraries(
    hello-rootserver
    sel4
    muslc
    sel4muslcsys
)

# Install to binary directory so the elfloader can find it.
include(rootserver)
DeclareRootserver(hello-rootserver)
