(* syn + quote concepts in OCaml via ppxlib *)

(* Showing what ppxlib-equivalent code looks like *)
(* ppxlib.Ast_pattern + ppxlib.Ast_builder *)

(* Simulate what syn::DeriveInput parses *)
type ast_field = {
  name: string;
  typ: string;
}

type ast_struct = {
  struct_name: string;
  fields: ast_field list;
}

(* Simulate what quote! generates *)
let generate_impl ast =
  let field_impls = List.map (fun f ->
    Printf.sprintf "  fn %s(&self) -> &%s { &self.%s }"
      f.name f.typ f.name
  ) ast.fields in
  Printf.sprintf "impl %s {\n%s\n}"
    ast.struct_name
    (String.concat "\n" field_impls)

let () =
  let ast = {
    struct_name = "User";
    fields = [
      {name = "name"; typ = "String"};
      {name = "age"; typ = "u32"};
      {name = "email"; typ = "String"};
    ]
  } in
  let code = generate_impl ast in
  Printf.printf "Generated code:\n%s\n" code
