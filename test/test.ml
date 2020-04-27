let _ =
  let v = Vec.create 10 in
  Printf.printf "%d\n" (Vec.length v);
  assert (Vec.length v = 0);
  Printf.printf "%d\n" (Vec.length v);
  for i = 1 to 100 do
    Vec.push v (Float.of_int i)
  done;
  print_endline "Checking length after push";
  assert (Vec.length v = 100);
  Printf.printf "%d\n" (Vec.length v);
  assert (Vec.(v.|[0] = Some 1.0));
  Vec.(v.|[0] <- 555.1);
  assert (Vec.(v.|[0] = Some 555.1));
  print_endline "Checking values";
  for i = 0 to 100 do
    match Vec.pop v with
    | Some x ->
        Printf.printf "some %f\n" x;
        assert (i < 100)
    | None ->
        print_endline "none";
        assert (i = 100)
  done;
  Vec.clear v;
  print_endline "Checking length after clear";
  assert (Vec.length v = 0);
  Gc.minor ();
  Gc.full_major ()
