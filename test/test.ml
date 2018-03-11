let _ =
    let v = Vec.create 10 in
    assert (Vec.length v = 0);
    Printf.printf "%d\n" (Vec.length v);
    for i = 1 to 100 do
        Vec.push v i
    done;
    assert (Vec.length v = 100);
    Printf.printf "%d\n" (Vec.length v);
    assert Vec.(v.|[0] = Some 1);
    Vec.(v.|[0] <- 555);
    assert Vec.(v.|[0] = Some 555);
    for i = 0 to 100 do
        match Vec.pop v with
        | Some x -> assert (i < 100); Printf.printf "some %d\n" x
        | None -> assert (i = 100); print_endline "none"
    done;
    Vec.clear v;
    assert (Vec.length v = 0)
