type 'a t

val create: int -> 'a t
val length: 'a t -> int
val push: 'a t -> 'a -> unit
val pop: 'a t -> 'a option
val clear: 'a t -> unit
val get: 'a t -> int -> 'a option
val set: 'a t -> int -> 'a -> unit

val ( .|[] ): 'a t -> int -> 'a option
val ( .|[]<- ): 'a t -> int -> 'a -> unit
