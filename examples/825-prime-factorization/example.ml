(* Prime Factorization in OCaml — trial division + Pollard rho concept *)

(* GCD — the elegant one-liner *)
let rec gcd a b = if b = 0 then a else gcd b (a mod b)

(* Trial division factorization *)
let factorize (n : int) : (int * int) list =
  let factors = ref [] in
  let n = ref n in
  (* Handle factor 2 separately *)
  let cnt = ref 0 in
  while !n mod 2 = 0 do
    incr cnt; n := !n / 2
  done;
  if !cnt > 0 then factors := (2, !cnt) :: !factors;
  (* Odd factors from 3 to sqrt(n) *)
  let d = ref 3 in
  while !d * !d <= !n do
    cnt := 0;
    while !n mod !d = 0 do
      incr cnt; n := !n / !d
    done;
    if !cnt > 0 then factors := (!d, !cnt) :: !factors;
    d := !d + 2
  done;
  (* Remaining factor is prime *)
  if !n > 1 then factors := (!n, 1) :: !factors;
  List.rev !factors

(* Pollard's rho: find a non-trivial factor of n (n must be composite) *)
let pollard_rho (n : int) : int =
  if n mod 2 = 0 then 2
  else begin
    let x = ref 2 and y = ref 2 and c = ref 1 and d = ref 1 in
    while !d = 1 do
      x := (!x * !x + !c) mod n;
      y := (!y * !y + !c) mod n;
      y := (!y * !y + !c) mod n;
      d := gcd (abs (!x - !y)) n;
      if !d = n then begin
        (* Retry with different c *)
        c := !c + 1; x := 2; y := 2; d := 1
      end
    done;
    !d
  end

(* Pretty-print factorization *)
let pp_factors factors =
  String.concat " × " (List.map (fun (p, e) ->
    if e = 1 then string_of_int p
    else Printf.sprintf "%d^%d" p e
  ) factors)

let () =
  let tests = [12; 100; 360; 97; 1234567890; 720720] in
  List.iter (fun n ->
    let f = factorize n in
    Printf.printf "factorize(%d) = %s\n" n (pp_factors f)
  ) tests;
  (* Verify: product should equal n *)
  let n = 360 in
  let f = factorize n in
  let product = List.fold_left (fun acc (p, e) ->
    let rec pow b e = if e = 0 then 1 else b * pow b (e-1) in
    acc * pow p e
  ) 1 f in
  Printf.printf "product check: %d = %d → %b\n" n product (n = product)
