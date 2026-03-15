(* 487: String interning — deduplicate strings via a hash table in OCaml *)
(* In OCaml, strings are compared structurally; physical equality (==) tests
   pointer identity. An interner maps equal strings to the same physical object,
   so you can use (==) for O(1) equality checks after interning. *)

module Interner = struct
  type t = {
    table : (string, string) Hashtbl.t;
  }

  let create () = { table = Hashtbl.create 16 }

  (* Intern a string: return the canonical (physically unique) representative *)
  let intern t s =
    match Hashtbl.find_opt t.table s with
    | Some v -> v
    | None   ->
      (* Store s itself as the canonical representative *)
      Hashtbl.add t.table s s;
      s

  let size t = Hashtbl.length t.table
end

let () =
  let i = Interner.create () in

  (* Two calls with equal content return the SAME physical string *)
  let a = Interner.intern i "hi" in
  let b = Interner.intern i "hi" in
  (* In OCaml: (==) is physical equality — they share the same heap object *)
  assert (a == b);
  Printf.printf "intern same content → same object: ok\n";

  (* Different strings are different objects *)
  let c = Interner.intern i "ho" in
  assert (not (a == c));
  Printf.printf "intern different content → different objects: ok\n";

  (* Size: "hi" + "ho" = 2 unique strings *)
  Interner.intern i "a" |> ignore;
  Interner.intern i "b" |> ignore;
  Interner.intern i "a" |> ignore;  (* duplicate — no new entry *)
  assert (Interner.size i = 4);     (* "hi", "ho", "a", "b" *)
  Printf.printf "interner size after duplicates: %d\n" (Interner.size i);

  (* Structural equality still holds *)
  assert (a = "hi");
  assert (b = "hi");

  print_endline "All assertions passed."
