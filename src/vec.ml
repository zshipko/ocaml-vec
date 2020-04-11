type t

external create : int -> t = "vec_create" "vec_create"

external length : t -> int = "vec_length"

external push : t -> int -> unit = "vec_push"

external pop : t -> int option = "vec_pop"

external clear : t -> unit = "vec_clear"

external get : t -> int -> int option = "vec_index"

external set : t -> int -> int -> unit = "vec_set_index"

let ( .|[] ) = get

let ( .|[]<- ) = set
