cmake_minimum_required(VERSION 3.17.2)
project(vulkan_profiles_ash)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

file(TO_CMAKE_PATH "$ENV{VULKAN_SDK}" VULKAN_SDK)
file(TO_CMAKE_PATH "$ENV{VULKAN_PROFILES_PATH}" VULKAN_PROFILES_PATH)

add_library(${PROJECT_NAME} STATIC ${VULKAN_PROFILES_PATH}/vulkan_profiles.cpp)

target_include_directories(${PROJECT_NAME} PRIVATE ${VULKAN_PROFILES_PATH})
target_include_directories(${PROJECT_NAME} PRIVATE ${VULKAN_SDK}/Include)

# Force non-debug msvcrt, see https://github.com/rust-lang/rust/issues/39016
if (MSVC)
    set_property(TARGET ${PROJECT_NAME} PROPERTY MSVC_RUNTIME_LIBRARY "MultiThreaded")
endif()

install(TARGETS ${PROJECT_NAME} DESTINATION .)
