(* Example 111: RefCell<T> — Runtime Borrow Checking *)

(* OCaml doesn't have borrow checking. Mutable data is always accessible.
   RefCell adds Rust's borrow rules at runtime for non-Copy types. *)

(* Approach 1: Mutable list inside immutable binding *)
let collect_items () =
  let items = ref [] in
  items := "first" :: !items;
  items := "second" :: !items;
  items := "third" :: !items;
  List.rev !items

(* Approach 2: Shared mutable stack using mutable record field *)
type 'a stack = { mutable data : 'a list }

let make_stack () = { data = [] }
let push s x = s.data <- x :: s.data
let pop s = match s.data with
  | [] -> None
  | x :: rest -> s.data <- rest; Some x
let peek s = match s.data with [] -> None | x :: _ -> Some x

(* Approach 3: Event logger — immutable handle, mutable interior *)
type logger = { mutable entries : string list }

let make_logger () = { entries = [] }
let log l msg = l.entries <- l.entries @ [msg]
let log_entries l = l.entries
let log_count l = List.length l.entries

let () =
  (* Test collect_items *)
  let items = collect_items () in
  assert (items = ["first"; "second"; "third"]);
  Printf.printf "Items: %s\n" (String.concat ", " items);

  (* Test stack *)
  let s = make_stack () in
  push s 1; push s 2; push s 3;
  assert (peek s = Some 3);
  assert (pop s = Some 3);
  assert (pop s = Some 2);

  (* Test logger *)
  let logger = make_logger () in
  log logger "connect";
  log logger "query";
  assert (log_count logger = 2);
  assert (log_entries logger = ["connect"; "query"]);

  print_endline "ok"
