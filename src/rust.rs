use api::{JsRuntimeHandle, JsContextRef, JsValueRef, JsCreateRuntime, JsCreateContext, JsSetCurrentContext, JsRuntimeAttributes, JsDisposeRuntime, JsSourceContext, JsRunScript, JsErrorCode};
use std::ptr;
use libc::{c_void, wchar_t};

/// A Rust equivalent of the ChakraCore runtime.
///
/// All dependent ChakraCore runtimes and contexts are destroyed when this goes out of scope.
pub struct Runtime {
    rt: JsRuntimeHandle,
    cx: JsContextRef
}

impl Runtime {
    /// Creates a new `Runtime`.
    ///
    /// An equivalent runtime and context is created by ChakraCore when this is done.
    pub fn new(attributes: JsRuntimeAttributes) -> Result<Runtime, JsErrorCode> {
        let mut rt = JsRuntimeHandle(ptr::null_mut() as *mut c_void);
        let mut cx = JsContextRef(ptr::null_mut() as *mut c_void);
        unsafe {
            let status = JsCreateRuntime(attributes, None, &mut rt);
            match status {
                JsErrorCode::JsNoError => {
                    let status = JsCreateContext(rt, &mut cx);
                    match status {
                        JsErrorCode::JsNoError => {
                            let status = JsSetCurrentContext(cx);
                            match status {
                                JsErrorCode::JsNoError => Ok(Runtime { rt: rt, cx: cx }),
                                _ => Err(status)
                            }
                        },
                        _ => Err(status)
                    }
                },
                _ => Err(status)
            }
        }
    }

    /// Returns the `JsRuntimeHandle` from ChakraCore.
    pub fn rt(&self) -> JsRuntimeHandle {
        self.rt
    }

    /// Returns the `JsContextRef` from ChakraCore.
    pub fn cx(&self) -> JsContextRef {
        self.cx
    }

    /// Runs a script and returns the result.
    pub fn run_script(&self, script: *const wchar_t, context: JsSourceContext, label: *const wchar_t) -> Result<JsValueRef, JsErrorCode> {
        let mut result = JsValueRef(ptr::null_mut() as *mut c_void);
        let status = unsafe { JsRunScript(script, context, label, &mut result) };
        match status {
            JsErrorCode::JsNoError => Ok(result),
            _ => Err(status)
        }
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            JsSetCurrentContext(JsContextRef(ptr::null_mut() as *mut c_void));
            JsDisposeRuntime(self.rt);
        }
    }
}
