#[macro_use]
extern crate ocaml;

use std::mem;

extern "C" fn finalize(value: ocaml::core::Value) {
    let handle = ocaml::Value(value);
    let ptr = handle.custom_ptr_val_mut::<(*mut i32, usize, usize)>();

    let (p, len, cap) = unsafe {
        *ptr
    };

    let vec: Vec<i32> = unsafe {
        Vec::from_raw_parts(p, len, cap)
    };

    mem::drop(vec)
}

macro_rules! load_vec {
    ($v:ident, $vec:ident, $block:block) => {
        let mut $vec = &mut *$v.custom_ptr_val_mut::<Vec<i32>>();

        $block

        mem::forget($vec);
    }
}

macro_rules! modify_vec {
    ($v:ident, $vec:ident, $block:block) => {
        load_vec!($v, $vec, {
            $block
        });
    }
}

caml!(vec_create, |n|, <dest>, {
    let mut vec: Vec<i32> = Vec::with_capacity(n.usize_val());
    let ptr = &mut vec as *mut Vec<i32>;
    mem::forget(vec);
    dest = ocaml::Value::alloc_custom(ptr, finalize);
} -> dest);

caml!(vec_length, |handle|, <dest>, {
    let p = handle.custom_ptr_val::<Vec<i32>>();
    dest = ocaml::Value::usize((*p).len())
} -> dest);

caml!(vec_push, |handle, x|, {
    modify_vec!(handle, vec, {
        vec.push(x.i32_val());
    });
});

caml!(vec_pop, |handle|, <dest>, {
    modify_vec!(handle, vec, {
        dest = match vec.pop() {
            Some(x) => ocaml::Value::some(x),
            None => ocaml::Value::none()
        };
    });
} -> dest);

caml!(vec_clear, |handle|, {
    modify_vec!(handle, vec, {
        vec.clear()
    });
});

caml!(vec_index, |handle, index|, <dest>, {
    load_vec!(handle, vec, {
        if vec.len() <= index.usize_val() {
            dest = ocaml::Value::none();
        } else {
            dest = ocaml::Value::some(vec[index.usize_val()].clone())
        }
    });
} -> dest);

caml!(vec_set_index, |handle, index, x|, {
    load_vec!(handle, vec, {
        if vec.len() <= index.usize_val() {
            return
        }

        vec[index.usize_val()] = x.i32_val();
    });
});
