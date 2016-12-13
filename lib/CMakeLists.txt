cmake_minimum_required(VERSION 3.4.3)
set(CMAKE_CXX_STANDARD 11)
project(libTableGen CXX)

# if(LLVM_VERSION)
#   find_package(LLVM ${LLVM_VERSION} REQUIRED CONFIG)
# else()
#   find_package(LLVM REQUIRED CONFIG)
# endif()

# message(STATUS "Found LLVM ${LLVM_PACKAGE_VERSION}")
# message(STATUS "Using LLVMConfig.cmake in: ${LLVM_DIR}")

# file(TO_CMAKE_PATH "${CMAKE_SOURCE_DIR}/third-party/llvm-${LLVM_PACKAGE_VERSION}/lib" TABLEGEN_INCLUDE)

# set(TGSD "${TABLEGEN_INCLUDE}/TableGen")
# set(TABLEGEN_SOURCES "${TGSD}/Error.cpp" "${TGSD}/Record.cpp" "${TGSD}/SetTheory.cpp" "${TGSD}/StringMatcher.cpp" "${TGSD}/TGLexer.cpp" "${TGSD}/TGParser.cpp")


# include_directories(${LLVM_INCLUDE_DIRS} ${TABLEGEN_INCLUDE})
# add_definitions(${LLVM_DEFINITIONS})

# # Now build our tools
# add_library(tblgen STATIC ${TABLEGEN_SOURCES} TableGenWrapper.cc)

# # Find the libraries that correspond to the LLVM components
# # that we wish to use
# # llvm_map_components_to_libnames(llvm_libs support core irreader)
# # llvm_map_components_to_libnames(llvm_libs LLVM)

# # # message(FATAL_ERROR ${llvm_libs})
# # # Link against LLVM libraries
# target_link_libraries(tblgen LLVM)


# install(TARGETS tblgen DESTINATION ${CMAKE_INSTALL_PREFIX})


