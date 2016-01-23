#![crate_type = "dylib"]
#![feature(plugin)]
#![plugin(interpolate_idents)]
#[macro_use] extern crate cpython;

use cpython::*;

py_module_initializer!(rustmotifs, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));
    try!(m.add(py, "run", py_fn!(run(args: PyList))));
    // try!(add_val(py, &m));
    Ok(())
});

fn run(py: Python, xss: PyList) -> PyResult<PyObject> {
    println!("Rust says: Hello Python!");
    for xs in xss.iter(py) {
        let xs: PyList  = try!(xs.cast_into(py));
        for x in xs.iter(py) {
            println!("{}", try!(x.extract::<u64>(py)));
        }
    }
    Ok(py.None())
}
