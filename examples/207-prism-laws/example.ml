(* Example 207: Prism Laws — ReviewPreview and PreviewReview *)

type ('s, 'a) prism = {
  preview : 's -> 'a option;
  review  : 'a -> 's;
}

(* Prism Laws:
   1. ReviewPreview: preview (review a) = Some a
      (round-trip: inject then extract always succeeds)
   2. PreviewReview: if preview s = Some a then review a = s
      (round-trip: if extraction succeeds, re-injection gives back original)
*)

(* Approach 1: Lawful prisms *)
type json =
  | JString of string
  | JInt of int
  | JBool of bool
  | JNull
  | JArray of json list

let jstring_prism : (json, string) prism = {
  preview = (function JString s -> Some s | _ -> None);
  review = (fun s -> JString s);
}

let jint_prism : (json, int) prism = {
  preview = (function JInt n -> Some n | _ -> None);
  review = (fun n -> JInt n);
}

let jbool_prism : (json, bool) prism = {
  preview = (function JBool b -> Some b | _ -> None);
  review = (fun b -> JBool b);
}

(* Approach 2: An UNLAWFUL prism *)
let bad_prism : (json, string) prism = {
  preview = (function JString s -> Some (String.uppercase_ascii s) | _ -> None);
  (* violates PreviewReview: preview gives uppercase, review uses original *)
  review = (fun s -> JString s);
}

(* Approach 3: Law verification *)
let check_review_preview prism a =
  prism.preview (prism.review a) = Some a

let check_preview_review prism s =
  match prism.preview s with
  | None -> true  (* law only applies when preview succeeds *)
  | Some a -> prism.review a = s

let verify_prism_laws name prism values sources =
  let rp = List.for_all (check_review_preview prism) values in
  let pr = List.for_all (check_preview_review prism) sources in
  Printf.printf "%s: ReviewPreview=%b PreviewReview=%b\n" name rp pr;
  (rp, pr)

(* === Tests === *)
let () =
  (* jstring_prism is lawful *)
  let (rp, pr) = verify_prism_laws "jstring"
    jstring_prism
    ["hello"; "world"; ""]
    [JString "hello"; JInt 42; JNull] in
  assert rp; assert pr;

  (* jint_prism is lawful *)
  let (rp, pr) = verify_prism_laws "jint"
    jint_prism
    [1; 0; -99]
    [JInt 1; JString "x"; JBool true] in
  assert rp; assert pr;

  (* bad_prism violates PreviewReview *)
  let rp = check_review_preview bad_prism "hello" in
  assert (not rp); (* preview(review "hello") = Some "HELLO" ≠ Some "hello" *)

  (* Verify specific violation *)
  let s = JString "hello" in
  (match bad_prism.preview s with
   | Some a -> assert (bad_prism.review a <> s) (* "HELLO" ≠ "hello" *)
   | None -> assert false);

  print_endline "✓ All tests passed"
