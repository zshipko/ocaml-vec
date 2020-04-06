use ocaml::{FromValue, Opaque, Value};

extern "C" fn finalize(value: Value) {
    let mut ptr: Opaque<Vec<ocaml::Int>> = Opaque::from_value(value);
    ptr.data_mut().clear();
    println!("{}", ptr.data().len());
    unsafe {
        std::ptr::drop_in_place(ptr.ptr_mut());
    }
}

#[ocaml::func]
pub fn vec_create(n: ocaml::Int) -> Opaque<'static, Vec<ocaml::Int>> {
    let vec: Vec<ocaml::Int> = Vec::with_capacity(n as usize);
    let ptr = Opaque::new(&vec, Some(finalize));
    std::mem::forget(vec);
    ptr
}

#[ocaml::func]
pub fn vec_length(handle: Opaque<Vec<ocaml::Int>>) -> ocaml::Int {
    let p = handle.data();
    p.len() as ocaml::Int
}

#[ocaml::func]
pub fn vec_push(mut handle: Opaque<Vec<ocaml::Int>>, x: ocaml::Int) {
    let p = handle.data_mut();
    p.push(x);
}

#[ocaml::func]
pub fn vec_pop(mut handle: Opaque<Vec<ocaml::Int>>) -> Option<ocaml::Int> {
    let p = handle.data_mut();
    p.pop()
}

#[ocaml::func]
pub fn vec_clear(mut handle: Opaque<Vec<ocaml::Int>>) {
    let p = handle.data_mut();
    p.clear();
}

#[ocaml::func]
pub fn vec_index(handle: Opaque<Vec<ocaml::Int>>, index: ocaml::Int) -> Option<ocaml::Int> {
    let p = handle.data();
    p.get(index as usize).map(|x| *x)
}

#[ocaml::func]
pub fn vec_set_index(mut handle: Opaque<Vec<ocaml::Int>>, index: ocaml::Int, x: ocaml::Int) {
    let p = handle.data_mut();
    p[index as usize] = x;
}
