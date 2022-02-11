pub fn ch_def_specifications() -> &'static str{
    r#"
["Table",    [{
  v: ["CilDef", [{
    seq: ["CilDef",[{
      "d?": ["Str", null],
      "n?": ["Str", null],
      "v": ["CilDef",[{
        kind: "",
        txt: "",
      }]],
      "c": ["CilDef",[{
        kind: "",
        "txt?": ["Str", null],
        Ref: { ev: ""},
      }]],
      "bonus": ["CilDef", [{
        Ref: {ev: ""}
      }]]
    }]],
  }]],
}]]"#
}