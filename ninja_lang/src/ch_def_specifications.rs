
pub const CH_DEF_SPECIFICATIONS: &str = r#"["Table",    [{
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