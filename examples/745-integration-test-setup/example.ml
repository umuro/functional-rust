(* 745: Integration Test Setup — OCaml
   In OCaml with Dune, integration tests are separate executables
   in a test/ directory. We simulate the pattern here. *)

(* === lib.ml (the library) === *)
module MyLib = struct
  type config = {
    host: string;
    port: int;
    max_connections: int;
  }

  let default_config = { host = "localhost"; port = 8080; max_connections = 100 }

  let with_config host port max_connections =
    { host; port; max_connections }

  let config_to_string c =
    Printf.sprintf "%s:%d (max=%d)" c.host c.port c.max_connections

  type 'a result = Ok of 'a | Err of string

  let parse_port s =
    match int_of_string_opt s with
    | Some n when n > 0 && n < 65536 -> Ok n
    | Some n -> Err (Printf.sprintf "port %d out of range" n)
    | None   -> Err (Printf.sprintf "'%s' is not a number" s)
end

(* === tests/common.ml (shared helpers) === *)
module TestCommon = struct
  let test_config = MyLib.with_config "test-host" 9999 10

  let make_test_config ?(host="test") ?(port=9999) ?(max=10) () =
    MyLib.with_config host port max
end

(* === tests/config_test.ml === *)
let () =
  (* Integration test: uses only public API *)
  let cfg = TestCommon.make_test_config () in
  assert (cfg.MyLib.port = 9999);

  let s = MyLib.config_to_string cfg in
  assert (String.length s > 0);

  (match MyLib.parse_port "8080" with
  | MyLib.Ok n -> assert (n = 8080)
  | MyLib.Err e -> failwith e);

  (match MyLib.parse_port "99999" with
  | MyLib.Err _ -> ()
  | MyLib.Ok _ -> failwith "expected error");

  Printf.printf "Integration tests: OK\n"
