(* Space Age — Float Computation *)

type planet = Mercury | Venus | Earth | Mars | Jupiter | Saturn | Uranus | Neptune

let orbital_period = function
  | Mercury -> 0.2408467 | Venus -> 0.61519726 | Earth -> 1.0
  | Mars -> 1.8808158 | Jupiter -> 11.862615 | Saturn -> 29.447498
  | Uranus -> 84.016846 | Neptune -> 164.79132

let earth_year_seconds = 31557600.0

let age_on planet seconds =
  seconds /. (earth_year_seconds *. orbital_period planet)

let () =
  let seconds = 1_000_000_000.0 in
  let age = age_on Earth seconds in
  assert (abs_float (age -. 31.69) < 0.01)
