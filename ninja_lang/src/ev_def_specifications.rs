
pub const EV_DEF_SPECIFICATIONS : &str = r#"["Table",    [{
  v: ["CilDef", [{
    eseq: ["CilDef",[{
      "d?": ["Str", null],
      "n?": ["Str", null],
      "eval": ["CilDef",[{
        kind: "",
        txt: "",
      }]],
      "ec": ["CilDef",[{
        kind: "",
        "txt?": ["Str", null],
        Ref: { ev: ""},
      }]],
      "run": ["CilDef", [{
        Ref: {ev: ""}
      }]],
      "bonus": ["CilDef", [{
        Ref: {ev: ""}
      }]]
    }]],
  }]],
  own: "",
  chain: ["CilDef",[{
    and: ["CilDef",[{
      Ref: { ev : "" }
    }]]
  }]],

//  give: ["CilDef",[{
//    Ref:{ch: "", ev: ""}
//  }]],
//  take: ["CilDef",[{
//    Ref:{ch: "", ev: ""}
//  }]],
//  lose: ["CilDef",[{
//    Ref: {ev: ""}
//  }]],
  free: false,
//  peek: ["CilDef",[{
//    Ref: {ch: ""}
//  }]],
//  observe: ["CilDef",[{
//    Ref: {ch: ""}
//  }]]
}]]"#;