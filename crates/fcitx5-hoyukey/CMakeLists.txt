cmake_minimum_required(VERSION 3.16)
project(fcitx5-hoyukey VERSION 1.0.0 LANGUAGES CXX C)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(ECM 1.0.0 REQUIRED NO_MODULE)
set(CMAKE_MODULE_PATH ${ECM_FIND_MODULE_DIR} "${CMAKE_CURRENT_SOURCE_DIR}/cmake" ${CMAKE_MODULE_PATH})

find_package(Fcitx5Core 5.0.0 REQUIRED)
find_package(Fcitx5Module REQUIRED COMPONENTS QuickPhrase)

include(GNUInstallDirs)

# Locate or build hoyukey-c static library from Cargo workspace
set(HOYUKEY_C_LIB_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../../target/release")
find_library(HOYUKEY_C_LIB hoyukey_c HINTS ${HOYUKEY_C_LIB_DIR})

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

fcitx5_translate_desktop_file(hoyukey-addon.conf.in hoyukey-addon.conf)
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/hoyukey-addon.conf" DESTINATION "${FCITX_INSTALL_PKGDATADIR}/addon")
install(FILES "hoyukey.conf" DESTINATION "${FCITX_INSTALL_PKGDATADIR}/inputmethod")
install(TARGETS hoyukey DESTINATION "${FCITX_INSTALL_LIBDIR}/fcitx5")
