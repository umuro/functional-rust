(* Complex Numbers *)
(* Complex number arithmetic with real/imaginary parts *)

type complex = { re : float; im : float }

let create re im = { re; im }
let real c = c.re
let imaginary c = c.im

let add a b = { re = a.re +. b.re; im = a.im +. b.im }
let sub a b = { re = a.re -. b.re; im = a.im -. b.im }

let mul a b = {
  re = a.re *. b.re -. a.im *. b.im;
  im = a.re *. b.im +. a.im *. b.re
}

let conjugate c = { re = c.re; im = -. c.im }
let abs c = sqrt (c.re *. c.re +. c.im *. c.im)

let div a b =
  let denom = b.re *. b.re +. b.im *. b.im in
  { re = (a.re *. b.re +. a.im *. b.im) /. denom;
    im = (a.im *. b.re -. a.re *. b.im) /. denom }

let exp c =
  let ea = Stdlib.exp c.re in
  { re = ea *. cos c.im; im = ea *. sin c.im }
