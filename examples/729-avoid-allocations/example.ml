(* 729: Avoid allocations — hot-path techniques in OCaml *)
(* Rust avoids allocations by reusing buffers, using iterators, and stack arrays.
   OCaml's GC makes short-lived allocations cheap (minor GC), but:
   - Avoid repeated string concatenation (O(n²)) — use Buffer.
   - Reuse a Buffer or array across calls by clearing instead of recreating.
   - Use lazy sequences (Seq) to avoid intermediate arrays.
   - Count words by scanning bytes — zero allocation. *)

(* ── Technique 1: Reusable Buffer — reset without freeing ────────────────── *)

(* Write a formatted record into a pre-allocated Buffer.
   Caller owns the buffer and reuses it across many calls. *)
let format_record_into buf name score =
  Buffer.clear buf;         (* reset without freeing the underlying memory *)
  Buffer.add_string buf name;
  Buffer.add_char   buf ':';
  Buffer.add_string buf (string_of_int score)

(* ── Technique 2: Seq — zero intermediate allocations ───────────────────── *)

let sum_squares n =
  (* Seq.init produces elements lazily — no intermediate array *)
  Seq.init n (fun i -> i * i)
  |> Seq.fold_left (+) 0

let hot_filter_sum data =
  (* Array.to_seq is lazy; filter and map without intermediate arrays *)
  Array.to_seq data
  |> Seq.filter (fun x -> x > 0)
  |> Seq.map    (fun x -> x * 2)
  |> Seq.fold_left (+) 0

(* ── Technique 3: Reuse an array by clearing, not dropping ──────────────── *)

type pipeline = {
  mutable scratch : int array;
  mutable scratch_len : int;
}

let pipeline_create () = { scratch = Array.make 1024 0; scratch_len = 0 }

(* Process input: keep evens * 3, reusing the scratch buffer *)
let pipeline_process p input =
  p.scratch_len <- 0;
  let n = Array.length input in
  (* Grow scratch if needed — amortized; in hot code fix a max capacity *)
  if Array.length p.scratch < n then
    p.scratch <- Array.make n 0;
  Array.iter (fun x ->
    if x mod 2 = 0 then begin
      p.scratch.(p.scratch_len) <- x * 3;
      p.scratch_len <- p.scratch_len + 1
    end
  ) input;
  Array.sub p.scratch 0 p.scratch_len

(* ── Technique 4: Word count — scan bytes, zero allocation ──────────────── *)

let count_words_no_alloc s =
  let len = String.length s in
  let in_word = ref false in
  let count = ref 0 in
  for i = 0 to len - 1 do
    let is_ws = match s.[i] with ' '|'\t'|'\n' -> true | _ -> false in
    if not is_ws && not !in_word then begin
      in_word := true;
      incr count
    end else if is_ws then
      in_word := false
  done;
  !count

let () =
  (* format_record reuse *)
  let buf = Buffer.create 32 in
  format_record_into buf "Alice" 99;
  assert (Buffer.contents buf = "Alice:99");
  format_record_into buf "Bob" 0;
  assert (Buffer.contents buf = "Bob:0");
  print_endline "format_record reuse: ok";

  (* sum_squares *)
  assert (sum_squares 0 = 0);
  assert (sum_squares 4 = 0 + 1 + 4 + 9);  (* 14 *)
  print_endline "sum_squares: ok";

  (* hot_filter_sum *)
  assert (hot_filter_sum [| 1; -2; 3; -4 |] = (1 + 3) * 2);
  assert (hot_filter_sum [||] = 0);
  print_endline "hot_filter_sum: ok";

  (* pipeline reuse *)
  let p = pipeline_create () in
  let r1 = pipeline_process p [| 2; 3; 4 |] in
  assert (r1 = [| 6; 12 |]);
  let r2 = pipeline_process p [| 10 |] in
  assert (r2 = [| 30 |]);
  print_endline "pipeline reuse: ok";

  (* word count *)
  assert (count_words_no_alloc "" = 0);
  assert (count_words_no_alloc "hello" = 1);
  assert (count_words_no_alloc "hello world" = 2);
  assert (count_words_no_alloc "  a  b  c  " = 3);
  print_endline "word count: ok";

  print_endline "All assertions passed."
