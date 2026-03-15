(* 1029: HashMap Entry API *)
(* OCaml doesn't have a direct entry API — we simulate with find_opt + add *)

module StringMap = Map.Make(String)

(* Approach 1: Insert if absent (or_insert equivalent) *)
let or_insert () =
  let m = StringMap.empty |> StringMap.add "a" 1 in
  (* Insert "b" only if not present *)
  let m = match StringMap.find_opt "b" m with
    | Some _ -> m
    | None -> StringMap.add "b" 42 m
  in
  (* "a" already exists, so don't overwrite *)
  let m = match StringMap.find_opt "a" m with
    | Some _ -> m
    | None -> StringMap.add "a" 99 m
  in
  assert (StringMap.find "a" m = 1);
  assert (StringMap.find "b" m = 42)

(* Approach 2: Insert with default function (or_insert_with equivalent) *)
let or_insert_with () =
  let defaults = [("x", fun () -> 100); ("y", fun () -> 200)] in
  let m = StringMap.empty in
  let m = List.fold_left (fun acc (k, f) ->
    match StringMap.find_opt k acc with
    | Some _ -> acc
    | None -> StringMap.add k (f ()) acc
  ) m defaults in
  assert (StringMap.find "x" m = 100);
  assert (StringMap.find "y" m = 200)

(* Approach 3: Modify existing or insert (and_modify + or_insert) *)
let and_modify_or_insert () =
  let update_or_insert m key default modify =
    match StringMap.find_opt key m with
    | Some v -> StringMap.add key (modify v) m
    | None -> StringMap.add key default m
  in
  let m = StringMap.empty in
  let m = update_or_insert m "count" 1 (fun x -> x + 1) in
  assert (StringMap.find "count" m = 1);
  let m = update_or_insert m "count" 1 (fun x -> x + 1) in
  assert (StringMap.find "count" m = 2);
  let m = update_or_insert m "count" 1 (fun x -> x + 1) in
  assert (StringMap.find "count" m = 3)

let () =
  or_insert ();
  or_insert_with ();
  and_modify_or_insert ();
  Printf.printf "✓ All tests passed\n"
