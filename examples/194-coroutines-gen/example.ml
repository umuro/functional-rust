effect Yield_val : int -> unit

let generator f =
  let resume = ref (fun () -> ()) in
  let values = ref [] in
  resume := (fun () ->
    match f () with
    | () -> ()
    | effect (Yield_val n) k ->
      values := n :: !values;
      resume := (fun () -> continue k ())
  );
  !resume ();
  List.rev !values

(* Range generator *)
let range lo hi () =
  let i = ref lo in
  while !i <= hi do
    perform (Yield_val !i);
    incr i
  done

let () =
  let vals = generator (range 1 5) in
  Printf.printf "range 1..5: [%s]\n"
    (vals |> List.map string_of_int |> String.concat ";")
