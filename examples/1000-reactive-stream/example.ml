(* 1000: Reactive Stream
   Push-based observable in OCaml using higher-order functions.
   An observable is just a function that accepts a callback (observer).
   map/filter/take are operators returning new observables — pure composition. *)

(* An observable is a function: observer -> unit *)
(* observer receives values via on_next, completion via on_complete *)
type 'a observable = {
  subscribe: ('a -> unit) -> (unit -> unit) -> unit;
  (* subscribe on_next on_complete *)
}

(* Create an observable from a list *)
let from_list items =
  { subscribe = fun on_next on_complete ->
      List.iter on_next items;
      on_complete ()
  }

(* Operator: map *)
let obs_map f obs =
  { subscribe = fun on_next on_complete ->
      obs.subscribe (fun v -> on_next (f v)) on_complete
  }

(* Operator: filter *)
let obs_filter pred obs =
  { subscribe = fun on_next on_complete ->
      obs.subscribe (fun v -> if pred v then on_next v) on_complete
  }

(* Operator: take — emit at most n values *)
let obs_take n obs =
  { subscribe = fun on_next on_complete ->
      let remaining = ref n in
      obs.subscribe
        (fun v ->
           if !remaining > 0 then begin
             decr remaining;
             on_next v
           end)
        on_complete
  }

(* Collect all emitted values into a list *)
let collect obs =
  let buf = ref [] in
  obs.subscribe (fun v -> buf := v :: !buf) (fun () -> ());
  List.rev !buf

let () =
  (* from_list *)
  assert (collect (from_list [1; 2; 3]) = [1; 2; 3]);

  (* map *)
  let mapped = obs_map (fun x -> x * 2) (from_list [1; 2; 3]) in
  assert (collect mapped = [2; 4; 6]);

  (* filter *)
  let filtered = obs_filter (fun x -> x mod 2 = 0) (from_list [1; 2; 3; 4; 5]) in
  assert (collect filtered = [2; 4]);

  (* take *)
  let taken = obs_take 3 (from_list [1; 2; 3; 4; 5]) in
  assert (collect taken = [1; 2; 3]);

  (* chain: filter evens, square, take 3 *)
  let source = from_list [1; 2; 3; 4; 5; 6; 7; 8; 9; 10] in
  let result =
    source
    |> obs_filter (fun x -> x mod 2 = 0)
    |> obs_map (fun x -> x * x)
    |> obs_take 3
    |> collect
  in
  assert (result = [4; 16; 36]);

  (* empty observable *)
  assert (collect (from_list []) = []);

  Printf.printf "chain result: [%s]\n"
    (String.concat "; " (List.map string_of_int result))
