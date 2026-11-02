cmake_minimum_required(VERSION 3.16)
project(fcitx5-hoyukey VERSION 1.0.0 LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

include(GNUInstallDirs)

find_package(ECM 1.0.0 QUIET NO_MODULE)
if(ECM_FOUND)
    set(CMAKE_MODULE_PATH ${ECM_FIND_MODULE_DIR} ${CMAKE_MODULE_PATH})
endif()

find_package(Fcitx5Core 5.0.0 REQUIRED)

# Locate hoyukey-c static library from Cargo workspace
find_library(HOYUKEY_C_LIB 
    NAMES libhoyukey_c.a hoyukey_c
    HINTS "${CMAKE_CURRENT_SOURCE_DIR}/../../target/release"
    REQUIRED
)

include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/src
    ${CMAKE_CURRENT_SOURCE_DIR}/../hoyukey-c/include
)

add_library(hoyukey MODULE
    src/hoyukey_engine.cpp
)

target_link_libraries(hoyukey PRIVATE
    Fcitx5::Core
    ${HOYUKEY_C_LIB}
    pthread
    dl
    m
)

set_target_properties(hoyukey PROPERTIES PREFIX "")

if(NOT DEFINED FCITX_INSTALL_PKGDATADIR)
    set(FCITX_INSTALL_PKGDATADIR "${CMAKE_INSTALL_DATADIR}/fcitx5")
endif()
if(NOT DEFINED FCITX_INSTALL_LIBDIR)
    set(FCITX_INSTALL_LIBDIR "${CMAKE_INSTALL_LIBDIR}")
endif()

install(FILES "hoyukey-addon.conf" DESTINATION "${FCITX_INSTALL_PKGDATADIR}/addon")
install(FILES "hoyukey.conf" DESTINATION "${FCITX_INSTALL_PKGDATADIR}/inputmethod")
install(TARGETS hoyukey DESTINATION "${FCITX_INSTALL_LIBDIR}/fcitx5")
