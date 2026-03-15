(* 1045: Small Vector Optimization Concept
   Rust uses enum{Inline{..N..}, Heap(Vec)} to avoid heap allocations for small N.
   OCaml: store capacity in the Inline variant so each small-vec has its own N. *)

type 'a small_vec =
  | Inline of { data : 'a option array; mutable len : int; capacity : int }
  | Heap of 'a list

(* Create a small vec with inline capacity n *)
let make ?(capacity = 4) () =
  Inline { data = Array.make capacity None; len = 0; capacity }

let push sv value =
  match sv with
  | Inline { data; len; capacity } when len < capacity ->
    data.(len) <- Some value;
    Inline { data; len = len + 1; capacity }
  | Inline { data; len; _ } ->
    (* Spill to heap *)
    let lst = ref [] in
    for i = len - 1 downto 0 do
      lst := (match data.(i) with Some v -> v | None -> assert false) :: !lst
    done;
    Heap (!lst @ [value])
  | Heap lst -> Heap (lst @ [value])

let length = function
  | Inline { len; _ } -> len
  | Heap lst -> List.length lst

let is_inline = function Inline _ -> true | Heap _ -> false

let get sv idx =
  match sv with
  | Inline { data; len; _ } ->
    if idx < len then data.(idx) else None
  | Heap lst ->
    (try Some (List.nth lst idx) with _ -> None)

let to_list = function
  | Inline { data; len; _ } ->
    let result = ref [] in
    for i = len - 1 downto 0 do
      result := (match data.(i) with Some v -> v | None -> assert false) :: !result
    done;
    !result
  | Heap lst -> lst

let () =
  (* Default capacity = 4 *)
  let sv = ref (make ()) in
  sv := push !sv 1;
  sv := push !sv 2;
  sv := push !sv 3;
  assert (length !sv = 3);
  assert (is_inline !sv);
  assert (to_list !sv = [1; 2; 3]);

  sv := push !sv 4;        (* at capacity = 4, still inline *)
  assert (is_inline !sv);

  sv := push !sv 5;        (* spills to heap *)
  assert (not (is_inline !sv));
  assert (to_list !sv = [1; 2; 3; 4; 5]);

  (* Indexed access *)
  let sv2 = ref (make ()) in
  sv2 := push !sv2 "hello";
  sv2 := push !sv2 "world";
  assert (get !sv2 0 = Some "hello");
  assert (get !sv2 1 = Some "world");
  assert (get !sv2 2 = None);

  (* Capacity = 2: spill after 2 elements *)
  let sv3 = ref (make ~capacity:2 ()) in
  sv3 := push !sv3 10;
  sv3 := push !sv3 20;
  assert (is_inline !sv3);
  sv3 := push !sv3 30;     (* spills *)
  assert (not (is_inline !sv3));
  assert (length !sv3 = 3);
  assert (get !sv3 2 = Some 30);

  (* Empty *)
  let sv4 = make () in
  assert (length sv4 = 0);
  assert (is_inline sv4);
  assert (get sv4 0 = None);

  (* Large N: stays inline for 16 elements *)
  let sv5 = ref (make ~capacity:16 ()) in
  for i = 0 to 15 do sv5 := push !sv5 i done;
  assert (is_inline !sv5);
  sv5 := push !sv5 16;
  assert (not (is_inline !sv5));

  Printf.printf "All small-vec tests passed.\n"
