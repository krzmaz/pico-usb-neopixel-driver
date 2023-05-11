cmake_minimum_required(VERSION 3.19)

set(PICO_BOARD pico_w)
set(PICO_DEFAULT_UART_BAUD_RATE 921600)
add_compile_definitions("PICO_DEFAULT_UART_BAUD_RATE=921600")
include(cmake/utils.cmake)

get_pico_sdk_import_cmake()
# initialize pico-sdk from GIT
# (note this can come from environment, CMake cache etc)
set(PICO_SDK_FETCH_FROM_GIT on)
# pico_sdk_import.cmake is a single file copied from this SDK
# note: this must happen before project()
include(cmake/pico_sdk_import.cmake)

if(DEBUG_LOGS)
        add_compile_definitions(DEBUG_LOGS)
endif(DEBUG_LOGS)



project(pico_usb_neopixel_driver C CXX ASM)
set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)

add_compile_options(
        -Wall
        -Werror
        )


# initialize the Raspberry Pi Pico SDK
pico_sdk_init()


add_subdirectory(src)
