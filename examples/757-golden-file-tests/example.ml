(* 757: Golden File Tests — OCaml *)

(* Code under test: renders an AST to a string *)
type expr =
  | Num   of int
  | Add   of expr * expr
  | Mul   of expr * expr
  | Var   of string

let rec render = function
  | Num n       -> string_of_int n
  | Var s       -> s
  | Add (a, b)  -> Printf.sprintf "(%s + %s)" (render a) (render b)
  | Mul (a, b)  -> Printf.sprintf "(%s * %s)" (render a) (render b)

(* Golden file infrastructure *)
let read_file path =
  try
    let ic = open_in path in
    let n = in_channel_length ic in
    let s = Bytes.create n in
    really_input ic s 0 n;
    close_in ic;
    Some (Bytes.to_string s)
  with Sys_error _ -> None

let write_file path content =
  let oc = open_out path in
  output_string oc content;
  close_out oc

let golden_dir = "tests/golden"

let assert_golden name actual =
  let path = Printf.sprintf "%s/%s.txt" golden_dir name in
  let update = (try Sys.getenv "UPDATE_GOLDEN" = "1" with Not_found -> false) in
  match read_file path, update with
  | None, _ | _, true ->
    (try Unix.mkdir golden_dir 0o755 with Unix.Unix_error _ -> ());
    write_file path actual;
    Printf.printf "[golden:%s] %s\n" name (if update then "Updated" else "Created")
  | Some expected, false ->
    if actual = expected
    then Printf.printf "[golden:%s] OK\n" name
    else begin
      Printf.printf "[golden:%s] MISMATCH\n" name;
      Printf.printf "Expected:\n%s\nActual:\n%s\n" expected actual;
      failwith "golden test failed"
    end

let () =
  let expr = Add (Mul (Num 2, Var "x"), Num 3) in
  assert_golden "expr_render" (render expr);
  let complex = Mul (Add (Var "a", Var "b"), Add (Num 1, Num 2)) in
  assert_golden "complex_expr" (render complex);
  Printf.printf "Golden tests done!\n"
