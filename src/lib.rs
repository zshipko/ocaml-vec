#[macro_use]
extern crate ocaml;

use std::mem;

extern "C" fn finalize(value: ocaml::core::Value) {
    let handle = ocaml::Value(value);
    let ptr = handle.custom_ptr_val_mut::<Vec<i32>>();
    mem::drop(ptr);
    println!("Finalize");
}

caml!(vec_create(n) {
    let mut vec: Vec<i32> = Vec::with_capacity(n.usize_val());
    let ptr = &mut vec as *mut Vec<i32>;
    mem::forget(vec);
    ocaml::Value::alloc_custom(ptr, finalize)
});

caml!(vec_length(handle) {
    let p = handle.custom_ptr_val::<Vec<i32>>();
    ocaml::Value::usize((*p).len())
});

caml!(vec_push(handle, x) {
    let vec = &mut *handle.custom_ptr_val_mut::<Vec<i32>>();
    vec.push(x.i32_val());
    ocaml::Value::unit()
});

caml!(vec_pop(handle) {
    let vec = &mut *handle.custom_ptr_val_mut::<Vec<i32>>();
    match vec.pop() {
        Some(x) => ocaml::Value::some(x),
        None => ocaml::Value::none()
    }
});

caml!(vec_clear(handle) {
    let vec = &mut *handle.custom_ptr_val_mut::<Vec<i32>>();
    vec.clear();
    ocaml::Value::unit()
});

caml!(vec_index(handle, index) {
    let vec = &mut *handle.custom_ptr_val_mut::<Vec<i32>>();
    if vec.len() <= index.usize_val() {
        ocaml::Value::none()
    } else {
        ocaml::Value::some(vec[index.usize_val()])
    }
});

caml!(vec_set_index(handle, index, x) {
    let vec = &mut *handle.custom_ptr_val_mut::<Vec<i32>>();
    if vec.len() <= index.usize_val() {
        return ocaml::Value::unit();
    }

    vec[index.usize_val()] = x.i32_val();

    ocaml::Value::unit()
});
