(* 978: Count-Min Sketch
   A probabilistic frequency estimator using d hash functions and w counters each.
   Overestimates frequencies (never underestimates); error bounded by ε with
   probability 1-δ where w = ⌈e/ε⌉ and d = ⌈ln(1/δ)⌉.
   OCaml: 2D array of counters + independent hash seeds. *)

type cms = {
  table  : int array array;  (* d rows × w columns *)
  seeds  : int array;        (* hash seed per row *)
  d : int;  (* depth  = number of hash functions *)
  w : int;  (* width  = number of buckets per row *)
}

(* Create with d hash functions and w buckets *)
let create ~d ~w =
  assert (d > 0 && w > 0);
  { table  = Array.init d (fun _ -> Array.make w 0);
    seeds  = Array.init d (fun _ -> Random.bits ());
    d; w }

(* Create for (ε, δ) error guarantee *)
let create_eps_delta ~epsilon ~delta =
  let w = int_of_float (ceil (exp 1.0 /. epsilon)) in
  let d = int_of_float (ceil (log (1.0 /. delta))) in
  create ~d ~w

(* Polynomial rolling hash seeded by seed *)
let hash_str s seed w =
  let h = ref seed in
  String.iter (fun c ->
    h := (!h * 31 + Char.code c) land max_int
  ) s;
  !h mod w

let add cms s =
  for i = 0 to cms.d - 1 do
    let col = hash_str s cms.seeds.(i) cms.w in
    cms.table.(i).(col) <- cms.table.(i).(col) + 1
  done

let count cms s =
  (* take the minimum across all rows — min overestimates cancel noise *)
  let m = ref max_int in
  for i = 0 to cms.d - 1 do
    let col = hash_str s cms.seeds.(i) cms.w in
    if cms.table.(i).(col) < !m then m := cms.table.(i).(col)
  done;
  !m

(* Merge two sketches (same d, w) — useful for distributed counting *)
let merge a b =
  assert (a.d = b.d && a.w = b.w);
  let c = create ~d:a.d ~w:a.w in
  (* copy seeds from a *)
  Array.blit a.seeds 0 c.seeds 0 a.d;
  for i = 0 to a.d - 1 do
    for j = 0 to a.w - 1 do
      c.table.(i).(j) <- a.table.(i).(j) + b.table.(i).(j)
    done
  done;
  c

(* Estimate total count (sum of first row suffices as a lower bound) *)
let total_count cms = Array.fold_left ( + ) 0 cms.table.(0)

let () =
  Random.self_init ();

  (* ε=0.01, δ=0.01 → w=272, d=5 *)
  let cms = create_eps_delta ~epsilon:0.01 ~delta:0.01 in
  Printf.printf "CMS: d=%d w=%d\n" cms.d cms.w;

  (* Simulate a stream with known frequencies *)
  let stream = [
    "apple",100; "banana",50; "cherry",25;
    "date",10;   "elderberry",5
  ] in
  List.iter (fun (word, freq) ->
    for _ = 1 to freq do add cms word done
  ) stream;

  Printf.printf "\nFrequency estimates (true vs estimated):\n";
  List.iter (fun (word, true_freq) ->
    let est = count cms word in
    Printf.printf "  %-15s true=%-4d est=%-4d  error=%d\n"
      word true_freq est (est - true_freq)
  ) stream;

  (* Small sketch to show mechanics *)
  Printf.printf "\n=== Small sketch (d=3, w=20) ===\n";
  let small = create ~d:3 ~w:20 in
  let words = ["foo"; "bar"; "foo"; "baz"; "foo"; "bar"; "foo"] in
  List.iter (add small) words;
  Printf.printf "foo count ≈ %d (true=4)\n" (count small "foo");
  Printf.printf "bar count ≈ %d (true=2)\n" (count small "bar");
  Printf.printf "baz count ≈ %d (true=1)\n" (count small "baz");
  Printf.printf "qux count ≈ %d (true=0)\n" (count small "qux")
