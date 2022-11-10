use gl33::gl_enumerations::*;
use glfw_bindgen as glfw;
use std::{
    ffi::{c_char, c_int, c_void, CStr},
    ptr,
    time::Instant,
};

unsafe extern "C" fn glfw_error_callback(error_code: c_int, description: *const c_char) {
    let description = CStr::from_ptr(description).to_string_lossy();
    eprintln!("GLFW error ({error_code}): {description}");
}

fn gl_get_proc_adress(symbol: *const u8) -> *const c_void {
    unsafe {
        glfw::glfwGetProcAddress(symbol as _)
            .map(|proc| proc as *const _)
            .unwrap_or(ptr::null())
    }
}

fn main() {
    unsafe {
        'glfw: {
            glfw::glfwSetErrorCallback(Some(glfw_error_callback));
            if glfw::glfwInit() == glfw::GLFW_FALSE as _ {
                eprintln!("glfwInit failed");
                return;
            }

            glfw::glfwWindowHint(glfw::GLFW_CONTEXT_VERSION_MAJOR as _, 3);
            glfw::glfwWindowHint(glfw::GLFW_CONTEXT_VERSION_MINOR as _, 3);
            glfw::glfwWindowHint(
                glfw::GLFW_OPENGL_PROFILE as _,
                glfw::GLFW_OPENGL_CORE_PROFILE as _,
            );

            let window = glfw::glfwCreateWindow(
                640,
                480,
                "Simple example\0".as_ptr() as _,
                ptr::null_mut(),
                ptr::null_mut(),
            );

            if window.is_null() {
                eprintln!("glfwCreateWindow failed");
                break 'glfw;
            }

            glfw::glfwMakeContextCurrent(window);
            let gl = gl33::GlFns::load_from(&gl_get_proc_adress).unwrap();

            glfw::glfwSwapInterval(1);
            let start = Instant::now();
            while glfw::glfwWindowShouldClose(window) == glfw::GLFW_FALSE as _ {
                gl.ClearColor(0.0, 0.0, start.elapsed().as_secs_f32().sin().abs(), 1.0);
                gl.Clear(GL_COLOR_BUFFER_BIT);

                glfw::glfwSwapBuffers(window);
                glfw::glfwPollEvents();
            }

            glfw::glfwDestroyWindow(window);
        }

        glfw::glfwTerminate();
    }
}
