(* 500: Run-Length Encoding — compress/decompress strings in OCaml *)
(* Encode consecutive repeated characters as (count, char) pairs.
   "aabbbcc" → [(2,'a'); (3,'b'); (2,'c')]
   Demonstrates fold-based accumulation, mirroring the Rust iterator approach. *)

(* Encode: produce a list of (count, char) pairs using List.fold_left *)
let encode s =
  if String.length s = 0 then []
  else begin
    let first = s.[0] in
    let rest_len = String.length s - 1 in
    (* Initial accumulator: (current_char, current_count, reversed_pairs) *)
    let (cur, cnt, acc) =
      Seq.fold_left
        (fun (cur, cnt, acc) c ->
          if c = cur then (cur, cnt + 1, acc)
          else ((* flush current run *) c, 1, (cnt, cur) :: acc))
        (first, 1, [])
        (String.to_seq (String.sub s 1 rest_len))
    in
    List.rev ((cnt, cur) :: acc)
  end

(* Decode: reconstruct string from (count, char) pairs using Buffer *)
let decode pairs =
  let buf = Buffer.create 16 in
  List.iter (fun (n, c) ->
    for _ = 1 to n do Buffer.add_char buf c done
  ) pairs;
  Buffer.contents buf

(* Format encoded pairs as human-readable "2a3b2c" *)
let show_encoded pairs =
  let buf = Buffer.create 16 in
  List.iter (fun (n, c) ->
    Buffer.add_string buf (string_of_int n);
    Buffer.add_char buf c
  ) pairs;
  Buffer.contents buf

let () =
  (* basic encode *)
  assert (encode "aabbbcc" = [(2,'a'); (3,'b'); (2,'c')]);
  Printf.printf "encode(aabbbcc) = %s\n" (show_encoded (encode "aabbbcc"));

  (* all same *)
  assert (encode "aaaa" = [(4,'a')]);

  (* no repeats *)
  assert (encode "abcde" = [(1,'a');(1,'b');(1,'c');(1,'d');(1,'e')]);

  (* empty *)
  assert (encode "" = []);
  print_endline "encode: ok";

  (* roundtrip *)
  List.iter (fun s ->
    assert (decode (encode s) = s)
  ) ["aabbbcc"; "aaaa"; "abcde"; "aabbcc"; "z"];
  print_endline "roundtrip: ok";

  (* show *)
  assert (show_encoded [(2,'a');(3,'b');(2,'c')] = "2a3b2c");
  print_endline "show_encoded: ok";

  print_endline "All assertions passed."
