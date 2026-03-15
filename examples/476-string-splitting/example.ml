(* 476: String Splitting
   OCaml's String.split_on_char, and custom helpers for split_once
   and splitn equivalents. *)

(* Split into at most [n] pieces — like Rust's splitn(n, pat) *)
let splitn s sep n =
  if n <= 0 then []
  else if n = 1 then [s]
  else begin
    let result = ref [] in
    let remaining = ref s in
    let count = ref 1 in
    let running = ref true in
    while !running do
      if !count >= n then begin
        result := !remaining :: !result;
        running := false
      end else
        match String.index_opt !remaining sep with
        | None   -> result := !remaining :: !result; running := false
        | Some i ->
          result := String.sub !remaining 0 i :: !result;
          remaining := String.sub !remaining (i+1)
                         (String.length !remaining - i - 1);
          incr count
    done;
    List.rev !result
  end

(* Split on first occurrence — like Rust's split_once(pat) *)
let split_once s sep =
  match String.index_opt s sep with
  | None   -> None
  | Some i ->
    let before = String.sub s 0 i in
    let after  = String.sub s (i+1) (String.length s - i - 1) in
    Some (before, after)

(* Split on whitespace — like Rust's split_whitespace() *)
let split_whitespace s =
  String.split_on_char ' ' s
  |> List.concat_map (String.split_on_char '\t')
  |> List.concat_map (String.split_on_char '\n')
  |> List.filter (fun t -> t <> "")

let () =
  (* Basic split *)
  let parts = String.split_on_char ',' "a,b,c" in
  assert (parts = ["a";"b";"c"]);
  Printf.printf "split on ',': %s\n%!" (String.concat "|" parts);

  (* splitn *)
  let parts2 = splitn "a:b:c:d" ':' 3 in
  assert (parts2 = ["a";"b";"c:d"]);
  Printf.printf "splitn 3 on ':': %s\n%!" (String.concat "|" parts2);

  (* split_once *)
  assert (split_once "k=v" '=' = Some ("k","v"));
  assert (split_once "noeq" '=' = None);
  Printf.printf "split_once \"k=v\": (%s,%s)\n%!"
    (split_once "k=v" '=' |> Option.get |> fst)
    (split_once "k=v" '=' |> Option.get |> snd);

  (* split_whitespace *)
  let ws = split_whitespace "  a  b  c  " in
  assert (ws = ["a";"b";"c"]);
  Printf.printf "split_whitespace: %s\n%!" (String.concat "|" ws)
