type 'a t

external create: int -> 'a t = "vec_create"
external length: 'a t -> int = "vec_length"
external push: 'a t -> 'a -> unit = "vec_push"
external pop: 'a t -> 'a option = "vec_pop"
external clear: 'a t -> unit = "vec_clear"
external get: 'a t -> int -> 'a option = "vec_index"
external set: 'a t -> int -> 'a -> unit = "vec_set_index"

let ( .|[] ) = get
let ( .|[]<- ) = set
