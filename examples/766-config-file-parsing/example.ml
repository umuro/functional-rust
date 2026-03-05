(* Config file parsing in OCaml — INI/TOML-like *)

type config = (string, (string, string) Hashtbl.t) Hashtbl.t

let parse_config text : config =
  let cfg = Hashtbl.create 8 in
  let current_section = ref "global" in
  Hashtbl.replace cfg "global" (Hashtbl.create 4);
  List.iter (fun raw_line ->
    (* Strip comment *)
    let line =
      match String.index_opt raw_line '#' with
      | Some i -> String.sub raw_line 0 i
      | None   -> raw_line
    in
    let line = String.trim line in
    if String.length line = 0 then ()  (* empty *)
    else if line.[0] = '[' then begin
      (* Section header [name] *)
      let name = String.sub line 1 (String.length line - 2) |> String.trim in
      current_section := name;
      if not (Hashtbl.mem cfg name) then
        Hashtbl.replace cfg name (Hashtbl.create 4)
    end else begin
      (* key = value *)
      match String.index_opt line '=' with
      | Some eq ->
        let key = String.trim (String.sub line 0 eq) in
        let value = String.trim (String.sub line (eq + 1) (String.length line - eq - 1)) in
        let section_tbl = Hashtbl.find cfg !current_section in
        Hashtbl.replace section_tbl key value
      | None -> ()  (* malformed line, skip *)
    end
  ) (String.split_on_char '\n' text);
  cfg

let get_str cfg section key default =
  match Hashtbl.find_opt cfg section with
  | None -> default
  | Some tbl -> Option.value ~default (Hashtbl.find_opt tbl key)

let get_int cfg section key default =
  match int_of_string_opt (get_str cfg section key "") with
  | Some n -> n
  | None   -> default

let () =
  let text = {|
# Main config
[server]
host = localhost
port = 8080

[database]
host = db.example.com
port = 5432
name = mydb  # production DB
max_connections = 10
|} in
  let cfg = parse_config text in
  Printf.printf "server.host = %s\n"  (get_str cfg "server"   "host" "");
  Printf.printf "server.port = %d\n"  (get_int cfg "server"   "port" 80);
  Printf.printf "db.host     = %s\n"  (get_str cfg "database" "host" "");
  Printf.printf "db.maxconn  = %d\n"  (get_int cfg "database" "max_connections" 5)
