let add x y = x + y
let add5 = add 5

let add_tup (x, y) = x + y

let curry   f x y = f (x, y)
let uncurry f (x, y) = f x y

let double    = ( * ) 2
let increment = ( + ) 1
let halve     = Fun.flip ( / ) 2

let scale_and_shift ~scale ~shift x = x * scale + shift
let celsius_of_fahrenheit = scale_and_shift ~scale:5 ~shift:(-160)

let () =
  assert (add5 10 = 15);
  assert (double 7 = 14);
  assert (halve 20 = 10);
  let pipeline = [double; increment; halve] in
  let result = List.fold_left (fun acc f -> f acc) 6 pipeline in
  assert (result = 6);
  print_endline "All assertions passed."
