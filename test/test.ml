let _ =
    let v = Vec.create 10 in
    assert (Vec.length v = 0);
    Printf.printf "%d\n" (Vec.length v);
    Vec.push v 1;
    Vec.push v 2;
    Vec.push v 3;
    Vec.push v 4;
    assert (Vec.length v = 4);
    Printf.printf "%d\n" (Vec.length v);
    assert Vec.(v.|[0] = Some 1);
    Vec.(v.|[0] <- 555);
    assert Vec.(v.|[0] = Some 555);
    for i = 0 to 4 do
        match Vec.pop v with
        | Some x -> assert (i <= 3); Printf.printf "some %d\n" x
        | None -> assert (i > 3); print_endline "none"
    done;
    Vec.clear v;
    assert (Vec.length v = 0)
