(* Currying in OCaml — natural, all functions are curried *)

(* Standard curried functions *)
let add x y = x + y
let multiply x y = x * y
let clamp lo hi x = max lo (min hi x)

(* Uncurried (tuple form) *)
let add_uncurried (x, y) = x + y

(* curry: convert uncurried to curried *)
let curry f x y = f (x, y)

(* uncurry: convert curried to uncurried *)
let uncurry f (x, y) = f x y

(* Flip argument order *)
let flip f x y = f y x

let () =
  (* Natural partial application via currying *)
  let add5 = add 5 in
  let times3 = multiply 3 in
  Printf.printf "add5(10) = %d\n" (add5 10);
  Printf.printf "times3(7) = %d\n" (times3 7);

  (* Three-arg curried function *)
  let clamp_0_100 = clamp 0 100 in
  Printf.printf "clamp_0_100(150) = %d\n" (clamp_0_100 150);

  (* curry/uncurry conversion *)
  let curried_add = curry add_uncurried in
  let add7 = curried_add 7 in
  Printf.printf "add7(3) = %d\n" (add7 3);

  (* Flip *)
  let sub = fun x y -> x - y in
  let rsub = flip sub in
  Printf.printf "rsub 3 10 = %d (10 - 3)\n" (rsub 3 10);

  (* Point-free pipeline using curried functions *)
  let process = List.map (add 10) [1;2;3;4;5] in
  Printf.printf "map (add 10) [1..5] = [%s]\n"
    (String.concat ";" (List.map string_of_int process))
