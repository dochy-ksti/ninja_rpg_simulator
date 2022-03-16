
pub const CH_DEF_SPECIFICATIONS : &str = r#"["Table",    [{
  o: ["CilDef", [{
    cseq: ["CilDef",[{
      "d?": ["Str", null],
      "n?": ["Str", null],
      "cval": ["CilDef",[{
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
  spot : 0,
  listen : 0,
  initial_events : ["CilDef",[{
    Ref : { ev :  "" }
  }]]
}]]"#;