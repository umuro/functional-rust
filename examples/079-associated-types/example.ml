(* 079: Associated Types
   OCaml expresses associated types via module type members *)

(* --- Approach 1: Module type with abstract type member --- *)

module type CONTAINER = sig
  type item                         (* the "associated type" *)
  type t
  val create   : unit -> t
  val push     : t -> item -> unit
  val pop      : t -> item option
  val is_empty : t -> bool
end

(* int stack implementation *)
module IntStack : CONTAINER with type item = int = struct
  type item = int
  type t    = { mutable elements: int list }
  let create () = { elements = [] }
  let is_empty s = s.elements = []
  let push s x   = s.elements <- x :: s.elements
  let pop s =
    match s.elements with
    | []     -> None
    | x :: r -> s.elements <- r; Some x
end

(* string queue implementation *)
module StringQueue : CONTAINER with type item = string = struct
  type item = string
  type t    = { mutable elements: string list }
  let create () = { elements = [] }
  let is_empty q = q.elements = []
  let push q x   = q.elements <- q.elements @ [x]   (* append = FIFO *)
  let pop q =
    match q.elements with
    | []     -> None
    | x :: r -> q.elements <- r; Some x
end

(* --- Approach 2: drain_all using the module's associated type --- *)

let drain_all (type a) (module C : CONTAINER with type item = a and type t = a C.t) container =
  let rec aux acc =
    match C.pop container with
    | None   -> List.rev acc
    | Some v -> aux (v :: acc)
  in
  aux []
[@@warning "-38"]  (* suppress unused module warning in demo *)

(* Simpler non-functor version for a concrete type: *)
let drain_int_stack s =
  let rec aux acc =
    match IntStack.pop s with
    | None   -> List.rev acc
    | Some v -> aux (v :: acc)
  in
  aux []

(* --- Approach 3: Custom sequence (range) using a module --- *)

module Range = struct
  type t = { mutable current: int; stop: int }
  let make start stop = { current = start; stop }
  let next r =
    if r.current >= r.stop then None
    else begin
      let v = r.current in
      r.current <- r.current + 1;
      Some v
    end
  let to_list r =
    let rec aux acc =
      match next r with
      | None   -> List.rev acc
      | Some v -> aux (v :: acc)
    in
    aux []
end

let () =
  (* int stack *)
  let s = IntStack.create () in
  IntStack.push s 1; IntStack.push s 2; IntStack.push s 3;
  Printf.printf "int stack pop = %s\n"
    (match IntStack.pop s with Some v -> string_of_int v | None -> "None");

  (* string queue *)
  let q = StringQueue.create () in
  StringQueue.push q "a"; StringQueue.push q "b";
  Printf.printf "string queue pop = %s\n"
    (match StringQueue.pop q with Some v -> v | None -> "None");

  (* drain *)
  let s2 = IntStack.create () in
  IntStack.push s2 1; IntStack.push s2 2; IntStack.push s2 3;
  let items = drain_int_stack s2 in
  Printf.printf "drained: [%s]\n"
    (String.concat "; " (List.map string_of_int items));

  (* range *)
  let r = Range.make 0 5 in
  Printf.printf "range 0..5 = [%s]\n"
    (String.concat "; " (List.map string_of_int (Range.to_list r)))
