(* 358: Insertion-Ordered Map (IndexMap)
   OCaml has no built-in ordered-insertion map, but it's easy to
   build one with a Hashtbl for O(1) lookup + a list to track order. *)

type ('k, 'v) ordered_map = {
  tbl   : ('k, 'v) Hashtbl.t;
  order : 'k Queue.t;
}

let create () = { tbl = Hashtbl.create 8; order = Queue.create () }

(* Insert preserves insertion order; duplicate keys update in-place
   without changing their position in the order. *)
let insert om key value =
  if not (Hashtbl.mem om.tbl key) then
    Queue.push key om.order;
  Hashtbl.replace om.tbl key value

let get om key = Hashtbl.find_opt om.tbl key

(* Iterate in insertion order *)
let iter_ordered om f =
  Queue.iter (fun k ->
    match Hashtbl.find_opt om.tbl k with
    | Some v -> f k v
    | None   -> ()
  ) om.order

let to_list om =
  let result = ref [] in
  iter_ordered om (fun k v -> result := (k, v) :: !result);
  List.rev !result

let length om = Hashtbl.length om.tbl

let () =
  let m = create () in
  insert m "b" 2;
  insert m "a" 1;
  insert m "c" 3;

  (* Preserves insertion order b, a, c — NOT alphabetical *)
  let keys = to_list m |> List.map fst in
  assert (keys = ["b";"a";"c"]);
  Printf.printf "insertion order: %s\n%!"
    (keys |> String.concat ", ");

  (* Lookup by key works fine *)
  assert (get m "b" = Some 2);
  assert (get m "a" = Some 1);
  Printf.printf "get \"b\"=%s\n%!" (get m "b" |> Option.get |> string_of_int);

  (* Update in place does not change order *)
  insert m "a" 99;
  let keys2 = to_list m |> List.map fst in
  assert (keys2 = ["b";"a";"c"]);
  assert (get m "a" = Some 99);
  Printf.printf "after update, order unchanged: %s, a=%d\n%!"
    (keys2 |> String.concat ", ")
    (get m "a" |> Option.get)
