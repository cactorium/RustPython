//! The python `ctypes` module
/// See also:
/// https://docs.python.org/3/library/ctypes.html

use crate::pyobject::{PyClassImpl, PyResult};
use crate::vm::VirtualMachine;

use libloading::{Library, Symbol};

fn load_library(convention: CallingConvention, name: PyStringRef, _: &VirtualMachine) -> PyResult {
    match convention {
        CallingConvention::CDecl => {
            // TODO
            unimplemented!()
        },
        CallingConvention::WinDll => {
            // TODO
            unimplemented!()
        }
    }
}

#[pyclass(name = "_FuncPtr")]
struct PyStructFuncPtr {
    // TODO
}

#[pyclass(name = "CDLL")]
struct PyStructCDLL {
    // TODO
}

#[pyimpl]
impl PyStructCDLL {
    fn new() -> PyStructCDLL {
        unimplemented!()
    }

    #[pymethod(name = "__getattr__")]
    fn get_func(&self, name: PyObjectRef, _vm: &VirtualMachine) -> PyResult {
        unimplemented!()
        // TODO
    }
}

enum CallingConvention {
    CDll, WinDll
}

#[pyclass(name = "LibraryLoader")]
struct PyStructLibraryLoader {
    convention: CallingConvention
}

#[pyimpl]
impl PyStructLibraryLoader {
    fn new() -> PyStructLibraryLoader {
        unimplemented!()
    }

    #[pymethod(name = "__getattr__")]
    fn load_library(&self, name: PyObjectRef, _vm: &VirtualMachine) -> PyResult {
        unimplemented!()
    }

    #[pymethod(name = "LoadLibrary")]
    fn load_library2(&self, name: PyObjectRef, _vm: &VirtualMachine) -> PyResult {
        self.load_library(name, _vm)
    }
}

pub fn make_module(vm: &VirtualMachine) -> PyObjectRef {
    let ctx = &vm.ctx;

    let struct_cdll_type = PyStructCDLL::make_class(ctx);

    // TODO add c types
    // TODO add create_string_buffer
    // TODO add cdll, windll, etc
    py_module!(vm, "ctypes", {
    })
}
