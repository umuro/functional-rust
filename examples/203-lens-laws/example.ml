(* Example 203: Lens Laws — GetSet, SetGet, SetSet *)

type ('s, 'a) lens = {
  get : 's -> 'a;
  set : 'a -> 's -> 's;
}

(* The three lens laws:
   1. GetSet: set (get s) s = s        — setting what you got changes nothing
   2. SetGet: get (set a s) = a        — you get back what you set
   3. SetSet: set b (set a s) = set b s — setting twice = setting last value *)

type point = { x : float; y : float }

(* Approach 1: A lawful lens *)
let x_lens : (point, float) lens = {
  get = (fun p -> p.x);
  set = (fun x p -> { p with x });
}

let y_lens : (point, float) lens = {
  get = (fun p -> p.y);
  set = (fun y p -> { p with y });
}

(* Approach 2: An UNLAWFUL lens — to show what goes wrong *)
let bad_lens : (point, float) lens = {
  get = (fun p -> p.x);
  set = (fun x p -> { x; y = p.y +. 1.0 }); (* side effect! mutates y *)
}

(* Approach 3: Law verification functions *)
let check_get_set lens s =
  let result = lens.set (lens.get s) s in
  result = s

let check_set_get lens a s =
  let result = lens.get (lens.set a s) in
  result = a

let check_set_set lens a b s =
  let r1 = lens.set b (lens.set a s) in
  let r2 = lens.set b s in
  r1 = r2

let verify_laws name lens s a b =
  let gs = check_get_set lens s in
  let sg = check_set_get lens a s in
  let ss = check_set_set lens a b s in
  Printf.printf "%s: GetSet=%b SetGet=%b SetSet=%b\n" name gs sg ss;
  (gs, sg, ss)

(* === Tests === *)
let () =
  let p = { x = 3.0; y = 4.0 } in

  (* x_lens is lawful *)
  let (gs, sg, ss) = verify_laws "x_lens" x_lens p 10.0 20.0 in
  assert gs; assert sg; assert ss;

  (* y_lens is lawful *)
  let (gs, sg, ss) = verify_laws "y_lens" y_lens p 10.0 20.0 in
  assert gs; assert sg; assert ss;

  (* bad_lens violates GetSet *)
  let (gs, _sg, _ss) = verify_laws "bad_lens" bad_lens p 10.0 20.0 in
  assert (not gs); (* GetSet fails: set changes y! *)

  (* Verify the specific violations *)
  let p2 = bad_lens.set (bad_lens.get p) p in
  assert (p2.y = 5.0); (* y was mutated! *)
  assert (p2 <> p);

  print_endline "✓ All tests passed"
