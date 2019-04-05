extern crate pyo3;

use std::io::Write;
use std::path::Path;
use pyo3::prelude::*;
use pyo3::types::PyList;

pub fn invoke() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let syspath: &PyList = py.import("sys").unwrap().get("path").unwrap().try_into().unwrap();
    // let ref src_path: String = src.unpack("src").unwrap();
    // if let Some(CtxObj::Array(sys_paths)) = src.get("sys_path") {
    //     for item in sys_paths {
    //         if let CtxObj::Str(sys_path) = item {
    //             syspath.insert(0, sys_path).unwrap();
    //         }
    //     }
    // }
    syspath.insert(0, "src").unwrap();
    let mod_path;
    // if let Some(parent) = Path::new(src_path).parent() {
    //     mod_path = parent;
    // }
    // else {
        mod_path = Path::new(".");
    // }
    // syspath.insert(0, mod_path.to_str().unwrap()).unwrap();

    let mod_name;
    // if let Some(stem) = Path::new(src_path).file_stem() {
    //     mod_name = stem.to_str().unwrap();
    // }
    // else {
    //     unreachable!();
    // }
    mod_name = "payload";
    let action = "main";

    match py.import(mod_name) {
    //     let ref action: String = ctx_step.unpack("action").unwrap();
         Ok(mod_py) => { 
                match mod_py.call_method1(action, (pyo3::types::PyDict::new(py), )) {
                Ok(_) => {
                    flush_stdio(py);
                    // Ok(())
                },
                Err(e) => {
                    flush_stdio(py);
                    e.print_and_set_sys_last_vars(py);
                    // Err(TaskError { msg: String::from("There was an exception raised by the step action."), src: TaskErrorSource::Internal })
                }
            }
        }
        Err(e) => {
            println!("Error during import");
            e.print_and_set_sys_last_vars(py);
        }
    }


}


fn flush_stdio(py: pyo3::Python) {
    py.run("sys.stderr.flush(); sys.stdout.flush()", None, None);
    std::io::stderr().flush();
    std::io::stdout().flush();
}

fn main() {
    println!("==============");
    invoke();
    println!("==============");
}

