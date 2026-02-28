function(GenerateElfImage)
    # If the standard KernelTarget isn't set, we'll look for the default 'kernel.elf'
    if(NOT DEFINED KernelTarget)
        if(TARGET kernel.elf)
            set(KernelTarget kernel.elf CACHE INTERNAL "")
        else()
            message(FATAL_ERROR "Forge Error: Could not find 'kernel.elf' target. Check KERNEL_PATH.")
        endif()
    endif()

    add_custom_command(
        OUTPUT "${CMAKE_BINARY_DIR}/images/pointsav-kernel.bin"
        COMMAND ${CMAKE_COMMAND} -E make_directory "${CMAKE_BINARY_DIR}/images"
        COMMAND ${CMAKE_COMMAND} -E echo "Fusing Kernel and RootTask into pointsav-kernel.bin..."
        # In a real build, this would call the actual elfloader tool
        COMMAND ${CMAKE_COMMAND} -E copy $<TARGET_FILE:${KernelTarget}> "${CMAKE_BINARY_DIR}/images/pointsav-kernel.bin"
        DEPENDS ${KernelTarget} pointsav_root_task
        COMMENT "Forging the Sovereign Substrate image"
    )
    add_custom_target(generate_image ALL DEPENDS "${CMAKE_BINARY_DIR}/images/pointsav-kernel.bin")
    message(STATUS "Forge: GenerateElfImage command initialized.")
endfunction()

function(DeclareRootserver target)
    set_property(TARGET ${target} PROPERTY ROOTSERVER TRUE)
    message(STATUS "Forge: Rootserver [${target}] declared.")
endfunction()
