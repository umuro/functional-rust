(* List.assoc — Association Lists as Simple Maps *)
(* Use association lists for key-value lookups *)

let phonebook = [("Alice", "555-1234"); ("Bob", "555-5678"); ("Carol", "555-9012")]
let bobs_number = List.assoc "Bob" phonebook
let has_dave = List.mem_assoc "Dave" phonebook
let without_bob = List.remove_assoc "Bob" phonebook
let () = Printf.printf "Bob: %s, Dave exists: %b\n" bobs_number has_dave
