use ocaml::{FromValue, Pointer};

fn remove_all_roots(p: &mut Vec<ocaml::Value>) {
    p.iter_mut().for_each(|x| x.remove_global_root());
}

unsafe extern "C" fn finalize(value: ocaml::Value) {
    let mut ptr = Pointer::<Vec<ocaml::Value>>::from_value(value);
    remove_all_roots(ptr.as_mut());
    ptr.drop_in_place();
}

#[ocaml::func]
pub fn vec_create(n: ocaml::Int) -> Pointer<Vec<ocaml::Value>> {
    let vec: Vec<ocaml::Value> = Vec::with_capacity(n as usize);
    let mut ptr: Pointer<Vec<ocaml::Value>> = Pointer::alloc_final(vec, Some(finalize), None);
    ptr
}

#[ocaml::func]
pub fn vec_length(handle: Pointer<Vec<ocaml::Value>>) -> ocaml::Int {
    let p = handle.as_ref();
    p.len() as ocaml::Int
}

#[ocaml::func]
pub fn vec_push(mut handle: Pointer<Vec<ocaml::Value>>, mut x: ocaml::Value) {
    let p = handle.as_mut();
    x.register_global_root();
    p.push(x);
}

#[ocaml::func]
pub fn vec_pop(mut handle: Pointer<Vec<ocaml::Value>>) -> Option<ocaml::Value> {
    ocaml::local!(x);
    let p = handle.as_mut();
    x = p.pop()?;
    x.remove_global_root();
    Some(x)
}

#[ocaml::func]
pub fn vec_clear(mut handle: Pointer<Vec<ocaml::Value>>) {
    let p = handle.as_mut();
    remove_all_roots(p);
    p.clear();
}

#[ocaml::func]
pub fn vec_index(handle: Pointer<Vec<ocaml::Value>>, index: ocaml::Int) -> Option<ocaml::Value> {
    let p = handle.as_ref();
    p.get(index as usize).map(|x| *x)
}

#[ocaml::func]
pub fn vec_set_index(
    mut handle: Pointer<Vec<ocaml::Value>>,
    index: ocaml::Int,
    mut x: ocaml::Value,
) {
    let p = handle.as_mut();
    x.register_global_root();
    p[index as usize] = x;
}
