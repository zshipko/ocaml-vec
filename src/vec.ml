
module C = struct
    type 'a vec
    external vec_create: int -> 'a vec = "vec_create"
    external vec_free: 'a vec -> unit = "vec_free"
    external vec_length: 'a vec -> int = "vec_length"
    external vec_push: 'a vec -> 'a -> 'a vec = "vec_push"
    external vec_pop: 'a vec -> 'a option * 'a vec = "vec_pop"
end

type 'a t = {
    mutable vec: 'a C.vec
}

let _wrap x =
    Gc.finalise (fun x -> C.vec_free x.vec) x;
    x

let create n = _wrap {
    vec = C.vec_create n
}

let length v = C.vec_length v.vec
let push v x = v.vec <- C.vec_push v.vec x
let pop v =
    let (x, vec) = C.vec_pop v.vec in
    v.vec <- vec;
    x


