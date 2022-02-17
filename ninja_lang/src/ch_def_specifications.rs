
pub const ch_def_specifications : &str = r#"["Table",    [{
  o: ["CilDef", [{
    cseq: ["CilDef",[{
      "d?": ["Str", null],
      "n?": ["Str", null],
      "cv": ["CilDef",[{
        kind: "",
        txt: "",
      }]],
      "cc": ["CilDef",[{
        kind: "",
        "txt?": ["Str", null],
        Ref: { ev: ""},
      }]],
      "action": ["CilDef", [{
        Ref: {ev: ""}
      }]]
    }]],
  }]],
}]]"#;