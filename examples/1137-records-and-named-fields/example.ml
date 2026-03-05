(* Records and Named Fields *)
(* Define and use record types *)

type student = {
  name : string;
  id : int;
  gpa : float;
  year : int;
}

let alice = { name = "Alice"; id = 1001; gpa = 3.8; year = 3 }
let bob = { name = "Bob"; id = 1002; gpa = 3.5; year = 2 }

let promote s = { s with year = s.year + 1 }
let alice_next = promote alice

let honor_roll students =
  List.filter (fun s -> s.gpa >= 3.7) students

let () = Printf.printf "%s (year %d, GPA %.1f)\n"
  alice_next.name alice_next.year alice_next.gpa
