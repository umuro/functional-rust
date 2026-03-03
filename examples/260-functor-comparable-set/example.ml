(* OCaml functor pattern: COMPARABLE module type + MakeSet functor *)

module type COMPARABLE = sig
  type t
  val compare : t -> t -> int
end

module MakeSet (C : COMPARABLE) = struct
  type t = C.t list
  let empty = []
  let mem x = List.exists (fun y -> C.compare x y = 0)
  let add x s = if mem x s then s else x :: s
  let to_list s = List.sort C.compare s
end

(* Instantiate the functor for Int and String *)
module IntSet = MakeSet(Int)
module StringSet = MakeSet(String)

let () =
  (* Int set — duplicates are ignored *)
  let s = IntSet.(empty |> add 3 |> add 1 |> add 3 |> add 2) in
  assert (IntSet.to_list s = [1; 2; 3]);
  assert (IntSet.mem 1 s = true);
  assert (IntSet.mem 5 s = false);

  (* String set *)
  let ss = StringSet.(empty |> add "banana" |> add "apple" |> add "cherry" |> add "apple") in
  assert (StringSet.to_list ss = ["apple"; "banana"; "cherry"]);

  List.iter (Printf.printf "%d ") (IntSet.to_list s);
  print_newline ();
  print_endline "ok"
