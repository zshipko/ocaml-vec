use ocaml::{FromValue, Pointer};

unsafe extern "C" fn finalize(value: ocaml::Value) {
    let ptr = Pointer::<Vec<ocaml::Value>>::from_value(value);
    ptr.drop_in_place();
}

#[ocaml::func]
pub fn vec_create(n: ocaml::Int) -> Pointer<Vec<ocaml::Value>> {
    let vec: Vec<ocaml::Value> = Vec::with_capacity(n as usize);
    let ptr: Pointer<Vec<ocaml::Value>> = Pointer::alloc_final(gc, vec, Some(finalize), None);
    ptr
}

#[ocaml::func]
pub fn vec_length(handle: Pointer<Vec<ocaml::Value>>) -> ocaml::Int {
    let p = handle.as_ref();
    p.len() as ocaml::Int
}

#[ocaml::func]
pub unsafe fn vec_push(mut handle: Pointer<Vec<ocaml::Value>>, x: ocaml::Value) {
    let p = handle.as_mut();
    p.push(x.deep_clone_to_rust());
}

#[ocaml::func]
pub unsafe fn vec_pop(mut handle: Pointer<Vec<ocaml::Value>>) -> Option<ocaml::Value> {
    let p = handle.as_mut();
    Some(p.pop()?.deep_clone_to_ocaml(gc))
}

#[ocaml::func]
pub fn vec_clear(mut handle: Pointer<Vec<ocaml::Value>>) {
    let p = handle.as_mut();
    p.clear();
}

#[ocaml::func]
pub unsafe fn vec_index(
    handle: Pointer<Vec<ocaml::Value>>,
    index: ocaml::Int,
) -> Option<ocaml::Value> {
    let p = handle.as_ref();
    p.get(index as usize).map(|x| x.deep_clone_to_ocaml(gc))
}

#[ocaml::func]
pub unsafe fn vec_set_index(
    mut handle: Pointer<Vec<ocaml::Value>>,
    index: ocaml::Int,
    x: ocaml::Value,
) {
    let p = handle.as_mut();
    p[index as usize] = x.deep_clone_to_rust();
}
