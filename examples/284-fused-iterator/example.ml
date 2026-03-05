(* 284. FusedIterator for terminated sequences - OCaml *)
(* OCaml Seq.t is naturally fused — once None, stays None *)

(* An unfused iterator that might return Some after None (bad practice) *)
let unfused_bad =
  let count = ref 0 in
  let limit = ref 3 in
  Seq.unfold (fun () ->
    incr count;
    if !count > !limit then begin
      (* Bad: could reset and return Some again *)
      None
    end else
      Some (!count, ())
  ) ()

let () =
  (* OCaml Seq is lazy and effectively fused by convention *)
  let fused = Seq.take 5 (Seq.of_list [1; 2; 3]) in
  List.iter (fun x -> Printf.printf "%d " x) (List.of_seq fused);
  print_newline ();

  (* Creating a guaranteed-terminating sequence *)
  let countdown n =
    Seq.unfold (fun i -> if i <= 0 then None else Some (i, i-1)) n
  in
  List.iter (fun x -> Printf.printf "%d " x) (List.of_seq (countdown 5));
  print_newline ()
