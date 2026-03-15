(* 1045: Small Vector Optimization Concept *)
(* Stack-allocate up to N elements, heap beyond *)
(* OCaml doesn't have this optimization — GC handles allocation *)

(* Approach 1: Simulated small-vec with variant type *)
type 'a small_vec =
  | Inline of 'a array * int   (* array of capacity N, length *)
  | Heap of 'a list             (* spills to list *)

let sv_capacity = 4

let sv_empty () = Inline (Array.make sv_capacity (Obj.magic 0), 0)

let sv_push sv x =
  match sv with
  | Inline (arr, len) when len < sv_capacity ->
    arr.(len) <- x;
    Inline (arr, len + 1)
  | Inline (arr, len) ->
    (* Spill to heap *)
    let lst = ref [x] in
    for i = len - 1 downto 0 do
      lst := arr.(i) :: !lst
    done;
    Heap !lst
  | Heap lst -> Heap (lst @ [x])

let sv_to_list = function
  | Inline (arr, len) ->
    let rec go i acc =
      if i < 0 then acc
      else go (i - 1) (arr.(i) :: acc)
    in
    go (len - 1) []
  | Heap lst -> lst

let sv_length = function
  | Inline (_, len) -> len
  | Heap lst -> List.length lst

let small_vec_test () =
  let sv = sv_empty () in
  let sv = sv_push sv 1 in
  let sv = sv_push sv 2 in
  let sv = sv_push sv 3 in
  assert (sv_length sv = 3);
  assert (sv_to_list sv = [1; 2; 3]);
  (* Still inline *)
  (match sv with Inline _ -> () | Heap _ -> assert false);
  let sv = sv_push sv 4 in
  (* At capacity, still inline *)
  (match sv with Inline _ -> () | Heap _ -> assert false);
  let sv = sv_push sv 5 in
  (* Spilled to heap *)
  (match sv with Inline _ -> assert false | Heap _ -> ());
  assert (sv_to_list sv = [1; 2; 3; 4; 5])

(* Approach 2: Why OCaml doesn't need this *)
(* OCaml's GC makes small allocations very cheap:
   - Minor heap allocation is just a pointer bump
   - Short-lived small objects are collected in microseconds
   - No stack vs heap distinction for the programmer
   The small-vec optimization is a Rust/C++ concern where
   heap allocation has higher fixed cost. *)

(* Approach 3: Fixed-size array as "small vec" *)
let fixed_array_demo () =
  let arr = Array.make 4 0 in
  let len = ref 0 in
  let push x =
    if !len < 4 then begin
      arr.(!len) <- x;
      incr len
    end
  in
  push 10; push 20; push 30;
  assert (!len = 3);
  assert (arr.(0) = 10);
  assert (arr.(2) = 30)

let () =
  small_vec_test ();
  fixed_array_demo ();
  Printf.printf "✓ All tests passed\n"
