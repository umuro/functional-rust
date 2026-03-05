(* DnD Character *)
(* Random generation with record types *)

type character = {
  charisma : int; constitution : int; dexterity : int;
  hitpoints : int; intelligence : int; strength : int; wisdom : int;
}

let ability () =
  let rolls = List.init 4 (fun _ -> 1 + Random.int 6) in
  let sorted = List.sort compare rolls |> List.tl in
  List.fold_left (+) 0 sorted

let modifier ~score =
  int_of_float (floor ((float_of_int score -. 10.0) /. 2.0))

let generate_character () =
  let () = Random.self_init () in
  let con = ability () in
  { charisma = ability (); constitution = con; dexterity = ability ();
    hitpoints = 10 + modifier ~score:con;
    intelligence = ability (); strength = ability (); wisdom = ability () }
