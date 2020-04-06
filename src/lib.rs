use ocaml::{FromValue, Pointer, Value};

extern "C" fn finalize(value: Value) {
    let mut ptr: Pointer<Vec<ocaml::Int>> = Pointer::from_value(value);
    println!("{}", ptr.data().len());
    unsafe {
        std::ptr::drop_in_place(ptr.ptr_mut());
    }
}

#[ocaml::func]
pub fn vec_create(n: ocaml::Int) -> Pointer<'static, Vec<ocaml::Int>> {
    let mut vec: Vec<ocaml::Int> = vec![0; n as usize];
    let ptr = &mut vec as *mut Vec<_>;
    std::mem::forget(vec);
    Pointer::new(ptr, Some(finalize))
}

#[ocaml::func]
pub fn vec_length(handle: Pointer<Vec<ocaml::Int>>) -> ocaml::Int {
    let p = handle.data();
    p.len() as ocaml::Int
}

#[ocaml::func]
pub fn vec_push(mut handle: Pointer<Vec<ocaml::Int>>, x: ocaml::Int) {
    let p = handle.data_mut();
    p.push(x);
}

#[ocaml::func]
pub fn vec_pop(mut handle: Pointer<Vec<ocaml::Int>>) -> Option<ocaml::Int> {
    let p = handle.data_mut();
    p.pop()
}

#[ocaml::func]
pub fn vec_clear(mut handle: Pointer<Vec<ocaml::Int>>) {
    let p = handle.data_mut();
    p.clear();
}

#[ocaml::func]
pub fn vec_index(handle: Pointer<Vec<ocaml::Int>>, index: ocaml::Int) -> Option<ocaml::Int> {
    let p = handle.data();
    p.get(index as usize).map(|x| *x)
}

#[ocaml::func]
pub fn vec_set_index(mut handle: Pointer<Vec<ocaml::Int>>, index: ocaml::Int, x: ocaml::Int) {
    let p = handle.data_mut();
    p[index as usize] = x;
}
