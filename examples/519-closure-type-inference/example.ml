(* OCaml: Hindley-Milner inference — closures can be polymorphic *)

(* Polymorphic identity closure *)
let id = fun x -> x  (* 'a -> 'a *)

(* Polymorphic map *)
let my_map f lst = List.map f lst

let () =
  (* id is polymorphic *)
  Printf.printf "id 42 = %d\n" (id 42);
  Printf.printf "id \"hello\" = %s\n" (id "hello");

  (* Inferred types in closures *)
  let add = fun x y -> x + y in  (* int -> int -> int, inferred from + *)
  Printf.printf "add 3 4 = %d\n" (add 3 4);

  (* Type annotation for clarity *)
  let parse : string -> int = int_of_string in
  Printf.printf "parse \"42\" = %d\n" (parse "42");

  (* Closure type fixed when used in context *)
  let double = fun x -> x * 2 in  (* int *)
  Printf.printf "%s\n" (String.concat " " (List.map (fun x -> string_of_int (double x)) [1;2;3]))
