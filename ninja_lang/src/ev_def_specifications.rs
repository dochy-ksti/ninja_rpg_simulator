pub fn ev_def_specifications() -> &'static str{
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
}]]"#
}