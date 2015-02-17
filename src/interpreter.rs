use std::ffi::{c_str_to_bytes, CString};
use std::path::Path;

use ll::*;
use tcl::{TclResult, TclEnvironment};

pub struct Interpreter <'env> {
    _env: &'env TclEnvironment,
    raw: *mut Tcl_Interp
}

#[unsafe_destructor]
impl <'env> Drop for Interpreter <'env> {
    fn drop(&mut self) {
        unsafe { Tcl_DeleteInterp(self.raw) };
    }
}

impl <'env> Interpreter <'env> {

    pub fn new(env: &TclEnvironment) -> Interpreter {
        Interpreter {
            _env: env,
            raw: unsafe { Tcl_CreateInterp() }
        }
    }

    pub unsafe fn raw(&mut self) -> *mut Tcl_Interp {
        self.raw
    }

    //TODO: Child interpreters - create, get, get parent, paths

    pub fn is_safe(&self) -> bool {
        unsafe { Tcl_IsSafe(self.raw) == 1 }
    }

    pub fn make_safe(&mut self) -> TclResult {
        let result = unsafe { Tcl_MakeSafe(self.raw) };
        TclResult::from_ll(result, self)
    }

    pub fn string_result(&self) -> String {
        unsafe {
            let string = Tcl_GetStringResult(self.raw);
            String::from_utf8_lossy(c_str_to_bytes(&string)).to_string()
        }
    }

    pub fn eval_file(&mut self, path: &Path) -> TclResult {
        let buf = CString::from_slice(path.to_string_lossy().as_bytes()).as_ptr();
        let result = unsafe {
            Tcl_EvalFile(self.raw, buf)
        };
        TclResult::from_ll(result, self)
    }

    pub fn eval(&mut self, code: &str) -> TclResult {
        let buf = CString::from_slice(code.as_bytes()).as_ptr();
        let result = unsafe {
            Tcl_Eval(self.raw, buf)
        };
        TclResult::from_ll(result, self)
    }
}
