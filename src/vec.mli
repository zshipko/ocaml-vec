type t

val create: int -> t
val length: t -> int
val push: t -> int -> unit
val pop: t -> int option
val clear: t -> unit
val get: t -> int -> int option
val set: t -> int -> int -> unit

val ( .|[] ): t -> int -> int option
val ( .|[]<- ): t -> int -> int -> unit
