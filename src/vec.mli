type 'a t

val create: int -> 'a t
val length: 'a t -> int
val push: 'a t -> 'a -> unit
val pop: 'a t -> 'a option
