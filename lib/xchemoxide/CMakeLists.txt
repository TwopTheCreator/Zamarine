cmake_minimum_required(VERSION 3.10)
project(xchemoxide VERSION 1.0.0 LANGUAGES C)

# Set C standard
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

# Platform-specific settings
if(WIN32)
    add_definitions(-DXCHEMOXIDE_PLATFORM_WINDOWS)
    set(PLATFORM_LIBS opengl32 gdi32 user32)
elseif(APPLE)
    add_definitions(-DXCHEMOXIDE_PLATFORM_MACOS)
    find_library(COCOA_LIBRARY Cocoa)
    find_library(OPENGL_LIBRARY OpenGL)
    set(PLATFORM_LIBS ${COCOA_LIBRARY} ${OPENGL_LIBRARY})
else()
    add_definitions(-DXCHEMOXIDE_PLATFORM_LINUX)
    find_package(OpenGL REQUIRED)
    find_package(X11 REQUIRED)
    set(PLATFORM_LIBS ${OPENGL_LIBRARIES} X11 Xrandr Xi Xcursor Xinerama Xxf86vm)
endif()

# Library target
add_library(xchemoxide
    src/xchemoxide.c
)

target_include_directories(xchemoxide
    PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
        $<INSTALL_INTERFACE:include>
)

target_link_libraries(xchemoxide PRIVATE ${PLATFORM_LIBS})

# Example application
add_executable(example examples/example.c)
target_link_libraries(example PRIVATE xchemoxide)

# Installation
include(GNUInstallDirs)
install(TARGETS xchemoxide
    EXPORT xchemoxide-targets
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)

install(DIRECTORY include/ DESTINATION ${CMAKE_INSTALL_INCLUDEDIR})

# Package config
include(CMakePackageConfigHelpers)
write_basic_package_version_file(
    ${CMAKE_CURRENT_BINARY_DIR}/xchemoxide-config-version.cmake
    VERSION ${PACKAGE_VERSION}
    COMPATIBILITY SameMajorVersion
)

install(FILES
    ${CMAKE_CURRENT_BINARY_DIR}/xchemoxide-config-version.cmake
    DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/xchemoxide
)

export(EXPORT xchemoxide-targets
    FILE ${CMAKE_CURRENT_BINARY_DIR}/xchemoxide-targets.cmake
    NAMESPACE xchemoxide::
)

install(EXPORT xchemoxide-targets
    FILE xchemoxide-targets.cmake
    NAMESPACE xchemoxide::
    DESTINATION ${CMAKE_INSTALL_LIBDIR}/cmake/xchemoxide
)
