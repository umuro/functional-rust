(* lazy_static / once_cell patterns in OCaml *)

(* OCaml module-level lets are eager, not lazy *)
(* For lazy: use lazy keyword or ref *)

let lazy_value = lazy (
  Printf.printf "Initializing expensive value...\n";
  let result = List.fold_left (+) 0 (List.init 1000 (fun i -> i)) in
  result
)

(* Global config (eager in OCaml) *)
let global_config = Hashtbl.create 16

let () =
  Hashtbl.add global_config "host" "localhost";
  Hashtbl.add global_config "port" "8080"

(* Singleton via module *)
module Registry = struct
  let instance : (string, string) Hashtbl.t = Hashtbl.create 16
  let register key value = Hashtbl.replace instance key value
  let lookup key = Hashtbl.find_opt instance key
end

let () =
  Registry.register "version" "1.0.0";
  let v = Lazy.force lazy_value in
  Printf.printf "Lazy value: %d\n" v;
  Printf.printf "Again (cached): %d\n" (Lazy.force lazy_value);
  match Registry.lookup "version" with
  | Some v -> Printf.printf "Version: %s\n" v
  | None -> Printf.printf "Not found\n"
