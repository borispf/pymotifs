#![crate_type = "dylib"]
#![feature(plugin)]
#![plugin(interpolate_idents)]
#[macro_use] extern crate cpython;

use cpython::*;

extern crate rustmotifs;
use rustmotifs::motifs::*;
use rustmotifs::network::*;

py_module_initializer!(rustmotifs_binding, |py, m| {
    try!(m.add(py, "__doc__", "Module documentation string"));
    try!(m.add(py, "run", py_fn!(run(args: PyList))));
    // try!(add_val(py, &m));
    Ok(())
});

fn run(py: Python, xss: PyList) -> PyResult<PyObject> {
    println!("Rust says: Hello Python!");
    let n = xss.len(py);
    let mut net = Network::with_capacity(n, 0);
    for i in 0..n {
        net.add_node(format!("{}", i + 1));
    }
    for (u, xs) in xss.iter(py).enumerate() {
        let xs: PyList  = try!(xs.cast_into(py));
        if xs.len(py) != n {
            return Err(PyErr::new::<cpython::exc::IndexError, String>(py, format!("all rows must have the same length (expected: {}, got: {})", n, xs.len(py))))
        }
        for (v, x) in xs.iter(py).enumerate() {
            let w = (try!(x.extract::<i8>(py)) as EdgeType) & 0x3;
            if w != 0 {
                net.add_edge(NodeIndex::new(u), NodeIndex::new(v), w);
            }
        }
    }
    let graphlets = py.allow_threads(|| enumerate_subgraphs(3, &net));
    let ret = PyDict::new(py);
    for (k, v) in graphlets {
        try!(ret.set_item(py, k, v));
    }
    Ok(ret.into_object())
}
