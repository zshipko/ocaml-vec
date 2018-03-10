let _ =
    let v = Vec.create 10 in
    Printf.printf "%d\n" (Vec.length v);
    Vec.push v 9;
    Vec.push v 8;
    Vec.push v 2;
    Vec.push v 3;
    Printf.printf "%d\n" (Vec.length v);
    for i = 0 to 4 do
        match Vec.pop v with
        | Some x -> Printf.printf "some %d\n" x
        | None -> print_endline "none"
    done
