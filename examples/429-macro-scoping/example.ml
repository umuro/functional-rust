(* Macro scoping in OCaml via module visibility *)

(* Macros (or macro-like helpers) scoped to module *)
module Internal = struct
  let debug_log msg = Printf.printf "[DEBUG] %s\n" msg
  let assert_ok condition msg =
    if not condition then failwith msg
end

(* Exported to users *)
module Public = struct
  let log = Internal.debug_log

  let assert_equal a b =
    Internal.assert_ok (a = b)
      (Printf.sprintf "Expected %d but got %d" b a)
end

(* Textual scoping: helpers must come before uses *)
let helper x = x * 2  (* defined first *)
let main_logic () = helper 21  (* uses helper — works *)
(* let bad = early_use ()  -- would fail: undefined *)
(* let early_use () = 42   -- comes after bad *)

let () =
  Public.log "Application started";
  Public.assert_equal (main_logic ()) 42;
  Printf.printf "All assertions passed\n"
