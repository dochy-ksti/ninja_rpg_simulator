use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use crate::{GuiItem, GuiItems};

pub(crate) fn test_data() -> GuiItems{
    let v = vec![
        t("hoihoi"),
        t("haihai"),
        t("hiihii"),
    ];

    let nexts = vec![
        p(0, &[1,2]),
        p(1, &[2]),
        p(2, &[]),
    ];
    k(v, &nexts)
}

fn t(s : &str) -> String{
    s.to_string()
}
fn p(i : usize, nexts : &[usize]) -> (usize, BTreeSet<usize>){
    (i, nexts.iter().map(|i| *i).collect())
}

fn k(v : Vec<String>, nexts : &[(usize, BTreeSet<usize>)]) -> GuiItems{
    let mut prevs : HashMap<usize, BTreeSet<usize>> = HashMap::new();

    for (i, ns ) in nexts{
        for n in ns{
            prevs.entry(*n)
                .or_insert_with(|| BTreeSet::new())
                .insert(*i);
        }
    }

    let v = &v;
    let mut items : Vec<GuiItem> = vec![];

    for (i, nexts) in nexts{

        let item = GuiItem::new(v[*i].to_string(),
                     nexts.iter().map(|i| *i).collect(),
                                prevs.get(i).map(|p| p.iter().map(|i| *i).collect()).unwrap_or_else(|| vec![]));
        items.push(item);
    }
    GuiItems::new(items, vec![])
}