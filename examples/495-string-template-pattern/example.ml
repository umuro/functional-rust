(* 495: String template pattern — render {{key}} templates in OCaml *)
(* No external library needed: scan for {{ ... }} with a buffer *)

(* Render a template string, replacing {{key}} with values from a lookup function.
   Unknown keys are left as {{key}} (same behaviour as the Rust version). *)
let render_fn ~lookup template =
  let buf = Buffer.create (String.length template) in
  let len = String.length template in
  let i = ref 0 in
  while !i < len do
    (* look for "{{" *)
    if !i + 1 < len && template.[!i] = '{' && template.[!i + 1] = '{' then begin
      let start = !i + 2 in
      (* search for closing "}}" *)
      let found = ref false in
      let j = ref start in
      while !j + 1 < len && not !found do
        if template.[!j] = '}' && template.[!j + 1] = '}' then
          found := true
        else
          incr j
      done;
      if !found then begin
        let key = String.sub template start (!j - start) in
        (match lookup key with
        | Some v -> Buffer.add_string buf v
        | None   ->
          Buffer.add_string buf "{{";
          Buffer.add_string buf key;
          Buffer.add_string buf "}}");
        i := !j + 2
      end else begin
        (* unclosed {{ — emit literally *)
        Buffer.add_string buf "{{";
        i := !i + 2
      end
    end else begin
      Buffer.add_char buf template.[!i];
      incr i
    end
  done;
  Buffer.contents buf

(* Convenience wrapper using a string-keyed list *)
let render vars template =
  render_fn ~lookup:(fun k -> List.assoc_opt k vars) template

let () =
  (* basic substitution *)
  let r = render [("x", "10"); ("y", "20")] "{{x}}+{{y}}" in
  assert (r = "10+20");
  Printf.printf "render: %s\n" r;

  (* missing key: keep placeholder *)
  let r2 = render [] "{{x}}" in
  assert (r2 = "{{x}}");
  Printf.printf "missing key: %s\n" r2;

  (* multiple occurrences *)
  let r3 = render [("n", "Alice")] "Hello {{n}}, nice to meet you {{n}}!" in
  assert (r3 = "Hello Alice, nice to meet you Alice!");
  Printf.printf "multi: %s\n" r3;

  (* render_fn with a function *)
  let r4 = render_fn ~lookup:(fun _ -> Some "42") "value={{k}}" in
  assert (r4 = "value=42");
  Printf.printf "render_fn: %s\n" r4;

  print_endline "All assertions passed."
