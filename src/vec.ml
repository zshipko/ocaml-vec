
module C = struct
    type 'a vec
    external vec_create: int -> 'a vec = "vec_create"
    external vec_free: 'a vec -> unit = "vec_free"
    external vec_length: 'a vec -> int = "vec_length"
    external vec_push: 'a vec -> 'a -> 'a vec = "vec_push"
    external vec_pop: 'a vec -> 'a option * 'a vec = "vec_pop"
    external vec_clear: 'a vec -> 'a vec = "vec_clear"
    external vec_index: 'a vec -> int -> 'a option = "vec_index"
    external vec_set_index: 'a vec -> int -> 'a -> unit = "vec_set_index"
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

let clear v =
    v.vec <- C.vec_clear v.vec

let get v i =
    C.vec_index v.vec i

let set v i x =
    C.vec_set_index v.vec i x

let ( .|[] ) = get
let ( .|[]<- ) = set
