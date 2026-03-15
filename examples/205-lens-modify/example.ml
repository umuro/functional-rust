(* Example 205: Lens Modify — Apply Function Through a Lens *)

type ('s, 'a) lens = {
  get : 's -> 'a;
  set : 'a -> 's -> 's;
}

(* modify: the key operation — apply a function to the focused value *)
let modify (l : ('s, 'a) lens) (f : 'a -> 'a) (s : 's) : 's =
  l.set (f (l.get s)) s

(* Approach 1: Basic modify on flat records *)
type counter = { count : int; label : string }

let count_lens = {
  get = (fun c -> c.count);
  set = (fun n c -> { c with count = n });
}

let increment = modify count_lens (( + ) 1)
let double = modify count_lens (( * ) 2)
let reset = modify count_lens (fun _ -> 0)

(* Approach 2: Modify through composed lenses *)
let compose outer inner = {
  get = (fun s -> inner.get (outer.get s));
  set = (fun b s -> outer.set (inner.set b (outer.get s)) s);
}

type inner = { value : int }
type outer = { inner : inner; tag : string }

let inner_lens = {
  get = (fun o -> o.inner);
  set = (fun i o -> { o with inner = i });
}

let value_lens = {
  get = (fun i -> i.value);
  set = (fun v i -> { i with value = v });
}

let outer_value = compose inner_lens value_lens

let increment_value = modify outer_value (( + ) 1)

(* Approach 3: Multiple modifications chained *)
let ( %~ ) lens f = modify lens f

let pipeline s =
  s
  |> count_lens %~ (( + ) 10)
  |> count_lens %~ (( * ) 2)

(* === Tests === *)
let () =
  let c = { count = 5; label = "clicks" } in

  (* Basic modify *)
  assert (increment c = { count = 6; label = "clicks" });
  assert (double c = { count = 10; label = "clicks" });
  assert (reset c = { count = 0; label = "clicks" });

  (* Modify through composition *)
  let o = { inner = { value = 10 }; tag = "test" } in
  let o2 = increment_value o in
  assert (o2.inner.value = 11);
  assert (o2.tag = "test");

  (* Chained modifications *)
  let c2 = pipeline c in
  assert (c2.count = 30); (* (5 + 10) * 2 *)

  (* Modify preserves other fields *)
  assert (c2.label = "clicks");

  print_endline "✓ All tests passed"
