#define GLFW_INCLUDE_NONE // don't include any OpenGL header
#include "glfw/include/GLFW/glfw3.h"

#if defined(_WIN32)
#define GLFW_EXPOSE_NATIVE_WIN32
#elif defined(__APPLE__)
#define GLFW_EXPOSE_NATIVE_COCOA
#else
#define GLFW_EXPOSE_NATIVE_X11
#if defined(GLFW_BINDGEN_FEATURE_WAYLAND)
#define GLFW_EXPOSE_NATIVE_WAYLAND
#endif
#endif
#include "glfw/include/GLFW/glfw3native.h"
