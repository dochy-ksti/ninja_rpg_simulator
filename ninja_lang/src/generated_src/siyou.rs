use docchi::core::intf::*;
use docchi::core::structs::*;
unsafe impl Send for RootIntf {}
unsafe impl Sync for RootIntf {}
#[derive(Debug)]
pub struct RootIntf{
    root : Box<RootObject>,
    ptr : RootObjectPtr,
}
impl RootIntf{
    pub fn new(obj : RootObject) -> RootIntf{
		let mut root = Box::new(obj);
		let ptr = RootObjectPtr::new(root.as_mut());
		RootIntf { root, ptr }
	}
    pub fn root_obj_ref(&self) -> &RootObject{ self.root.as_ref() }
    pub fn root_obj_ref_mut(&mut self) -> &mut RootObject{ self.root.as_mut() }
    pub fn deconstruct(self) -> RootObject{ *self.root }

	pub fn ev(&self) -> CTableConst<EvTable>{
		let t = EvTable::new(root::get_table(self.ptr.def(), "ev").unwrap());
		CTableConst::new(t, self)
	}
	pub fn cv(&self) -> MListConst<CvMItem>{
		let mil = root::get_mlist_const(self.ptr, "cv").unwrap().unwrap();
		MListConst::new(mil, self)
	}
	pub fn cv_mut(&mut self) -> MListMut<CvMItem>{
		let mil = root::get_mlist_mut(self.ptr, "cv").unwrap();
		MListMut::new(mil, self)
	}
	pub fn ch(&self) -> CTableConst<ChTable>{
		let t = ChTable::new(root::get_table(self.ptr.def(), "ch").unwrap());
		CTableConst::new(t, self)
	}
}
#[derive(Debug, PartialEq)]
pub struct EvTable {
	ptr : TablePtr,
}
impl EvTable {
	pub fn new(ptr : TablePtr) -> EvTable{ EvTable{ ptr } } 
	pub unsafe fn required1_us(&self) -> EvCItem {
		let ptr = table::get_value(self.ptr, "required1").unwrap();
		EvCItem::from(ptr)
	}
	pub fn required1(&self) -> CItemConst<EvCItem> {
		let ptr = table::get_value(self.ptr, "required1").unwrap();
		CItemConst::new(EvCItem::from(ptr), self)
	}
	pub unsafe fn tarou_first_us(&self) -> EvCItem {
		let ptr = table::get_value(self.ptr, "tarou_first").unwrap();
		EvCItem::from(ptr)
	}
	pub fn tarou_first(&self) -> CItemConst<EvCItem> {
		let ptr = table::get_value(self.ptr, "tarou_first").unwrap();
		CItemConst::new(EvCItem::from(ptr), self)
	}
	pub unsafe fn sample1_us(&self) -> EvCItem {
		let ptr = table::get_value(self.ptr, "sample1").unwrap();
		EvCItem::from(ptr)
	}
	pub fn sample1(&self) -> CItemConst<EvCItem> {
		let ptr = table::get_value(self.ptr, "sample1").unwrap();
		CItemConst::new(EvCItem::from(ptr), self)
	}
	pub unsafe fn get_by_id_us(&self, id : EvTableID) -> EvCItem{
		match id{
			EvTableID::Required1 => self.required1_us(),
			EvTableID::TarouFirst => self.tarou_first_us(),
			EvTableID::Sample1 => self.sample1_us(),
		}
	}
	pub fn get_by_id(&self, id : EvTableID) -> CItemConst<EvCItem>{
		CItemConst::new(unsafe{ self.get_by_id_us(id) }, self)
	}
}
#[repr(u64)] pub enum EvTableID{ Required1, TarouFirst, Sample1, }
impl EvTableID{
	pub fn from_str(id : &str) -> Option<Self>{
		match id{
			"required1" => Some(Self::Required1),
			"tarou_first" => Some(Self::TarouFirst),
			"sample1" => Some(Self::Sample1),
			_ =>{ None }
		}
	}
	pub fn from_num(id : usize) -> Self{
		match id{
			0 => Self::Required1,
			1 => Self::TarouFirst,
			2 => Self::Sample1,
			_ => panic!("invalid ID num {} EvTableID", id),
		}
	}
	pub fn len() -> usize{ 3 }
	pub fn to_num(&self) -> usize{
		match self{
			EvTableID::Required1 => 0,
			EvTableID::TarouFirst => 1,
			EvTableID::Sample1 => 2,
		}
	}
	pub fn metadata() -> &'static [&'static str]{
		&["required1", "tarou_first", "sample1", ]
	}
	pub fn to_str(&self) -> &'static str{
		Self::metadata()[self.to_num()]
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EvCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for EvCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl EvCItem {
	pub fn own_def_val(&self) -> &String{
		let qv = citem::get_str_def(self.ptr, "own").unwrap();
		qv.into_value().unwrap()
	}
	pub fn own(&self) -> &String{
		let qv = citem::get_immutable_str(self.ptr, "own").unwrap();
		qv.into_value().unwrap()
	}
	pub fn v(&self) -> CListConst<VCItem>{
		CListConst::new(citem::get_cil(self.ptr, "v").unwrap(), self)
	}
	pub fn chain(&self) -> CListConst<ChainCItem>{
		CListConst::new(citem::get_cil(self.ptr, "chain").unwrap(), self)
	}
	pub fn free(&self) -> bool{
		let qv = citem::get_bool(self.ptr, "free").unwrap();
		qv.into_value().unwrap()
	}
	pub fn free_def_val(&self) -> bool{
		let qv = citem::get_bool_def(self.ptr, "free").unwrap();
		qv.into_value().unwrap()
	}
	
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for VCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl VCItem {
	pub fn eseq(&self) -> CListConst<EseqCItem>{
		CListConst::new(citem::get_cil(self.ptr, "eseq").unwrap(), self)
	}
	
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EseqCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for EseqCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl EseqCItem {
	pub fn eval(&self) -> CListConst<EvalCItem>{
		CListConst::new(citem::get_cil(self.ptr, "eval").unwrap(), self)
	}
	pub fn bonus(&self) -> CListConst<BonusCItem>{
		CListConst::new(citem::get_cil(self.ptr, "bonus").unwrap(), self)
	}
	pub fn n_def_val(&self) -> NullOr<&String>{
		let qv = citem::get_str_def(self.ptr, "n").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn n(&self) -> NullOr<&String>{
		let qv = citem::get_immutable_str(self.ptr, "n").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn ec(&self) -> CListConst<EcCItem>{
		CListConst::new(citem::get_cil(self.ptr, "ec").unwrap(), self)
	}
	pub fn d_def_val(&self) -> NullOr<&String>{
		let qv = citem::get_str_def(self.ptr, "d").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn d(&self) -> NullOr<&String>{
		let qv = citem::get_immutable_str(self.ptr, "d").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn run(&self) -> CListConst<RunCItem>{
		CListConst::new(citem::get_cil(self.ptr, "run").unwrap(), self)
	}
	
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EvalCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for EvalCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl EvalCItem {
	pub fn kind_def_val(&self) -> &String{
		let qv = citem::get_str_def(self.ptr, "kind").unwrap();
		qv.into_value().unwrap()
	}
	pub fn kind(&self) -> &String{
		let qv = citem::get_immutable_str(self.ptr, "kind").unwrap();
		qv.into_value().unwrap()
	}
	pub fn txt_def_val(&self) -> &String{
		let qv = citem::get_str_def(self.ptr, "txt").unwrap();
		qv.into_value().unwrap()
	}
	pub fn txt(&self) -> &String{
		let qv = citem::get_immutable_str(self.ptr, "txt").unwrap();
		qv.into_value().unwrap()
	}
	
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BonusCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for BonusCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl BonusCItem {
	pub fn ref_ev(&self) -> EvCItem{
		let qv = citem::get_ref(self.ptr, "ev").unwrap();
		EvCItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_ev(&self) -> &String{
		let qv = citem::get_ref_id(self.ptr, "ev").unwrap();
		qv.into_value().unwrap()
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EcCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for EcCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl EcCItem {
	pub fn kind_def_val(&self) -> &String{
		let qv = citem::get_str_def(self.ptr, "kind").unwrap();
		qv.into_value().unwrap()
	}
	pub fn kind(&self) -> &String{
		let qv = citem::get_immutable_str(self.ptr, "kind").unwrap();
		qv.into_value().unwrap()
	}
	pub fn txt_def_val(&self) -> NullOr<&String>{
		let qv = citem::get_str_def(self.ptr, "txt").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn txt(&self) -> NullOr<&String>{
		let qv = citem::get_immutable_str(self.ptr, "txt").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn ref_ev(&self) -> EvCItem{
		let qv = citem::get_ref(self.ptr, "ev").unwrap();
		EvCItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_ev(&self) -> &String{
		let qv = citem::get_ref_id(self.ptr, "ev").unwrap();
		qv.into_value().unwrap()
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct RunCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for RunCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl RunCItem {
	pub fn ref_ev(&self) -> EvCItem{
		let qv = citem::get_ref(self.ptr, "ev").unwrap();
		EvCItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_ev(&self) -> &String{
		let qv = citem::get_ref_id(self.ptr, "ev").unwrap();
		qv.into_value().unwrap()
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ChainCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for ChainCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl ChainCItem {
	pub fn and(&self) -> CListConst<AndCItem>{
		CListConst::new(citem::get_cil(self.ptr, "and").unwrap(), self)
	}
	
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct AndCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for AndCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl AndCItem {
	pub fn ref_ev(&self) -> EvCItem{
		let qv = citem::get_ref(self.ptr, "ev").unwrap();
		EvCItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_ev(&self) -> &String{
		let qv = citem::get_ref_id(self.ptr, "ev").unwrap();
		qv.into_value().unwrap()
	}
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CvMItem {
	ptr : MItemPtr,
}
impl From<MItemPtr> for CvMItem {
	fn from(ptr : MItemPtr) -> Self {
		Self{ ptr }
	}
}
impl CvMItem {
	pub fn events(&self) -> MListConst<EventsMItem>{
		let mil = mitem::get_mil_const(self.ptr, "events").unwrap().unwrap();
		MListConst::new(mil, self)
	}
	pub fn events_mut(&mut self) -> MListMut<EventsMItem>{
		let p = mitem::get_mil_mut(self.ptr, "events").unwrap();
		MListMut::new(p, self)
	}
	pub fn spot(&self) -> i64{
		let qv = mitem::get_int(self.ptr, "spot").unwrap();
		qv.into_value().unwrap()
	}
	pub fn spot_def_val(&self) -> i64{
		let qv = mitem::get_int_def(self.ptr, "spot").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_spot(&mut self, spot : i64){
		mitem::set_int(self.ptr, "spot", Qv::Val(spot));
	}
	pub fn listen(&self) -> i64{
		let qv = mitem::get_int(self.ptr, "listen").unwrap();
		qv.into_value().unwrap()
	}
	pub fn listen_def_val(&self) -> i64{
		let qv = mitem::get_int_def(self.ptr, "listen").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_listen(&mut self, listen : i64){
		mitem::set_int(self.ptr, "listen", Qv::Val(listen));
	}
	pub fn ref_ch(&self) -> ChCItem{
		let qv = mitem::get_ref(self.ptr, "ch").unwrap();
		ChCItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_ch(&self) -> &String{
		let qv = mitem::get_ref_id(self.ptr, "ch").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_ref_ch(&mut self, id : ChTableID){
		mitem::set_ref(self.ptr, "ch", Qv::Val(id.to_str().to_string()));
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct EventsMItem {
	ptr : MItemPtr,
}
impl From<MItemPtr> for EventsMItem {
	fn from(ptr : MItemPtr) -> Self {
		Self{ ptr }
	}
}
impl EventsMItem {
	pub fn ref_ev(&self) -> EvCItem{
		let qv = mitem::get_ref(self.ptr, "ev").unwrap();
		EvCItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_ev(&self) -> &String{
		let qv = mitem::get_ref_id(self.ptr, "ev").unwrap();
		qv.into_value().unwrap()
	}
	pub fn set_ref_ev(&mut self, id : EvTableID){
		mitem::set_ref(self.ptr, "ev", Qv::Val(id.to_str().to_string()));
	}
}

#[derive(Debug, PartialEq)]
pub struct ChTable {
	ptr : TablePtr,
}
impl ChTable {
	pub fn new(ptr : TablePtr) -> ChTable{ ChTable{ ptr } } 
	pub unsafe fn tarou_us(&self) -> ChCItem {
		let ptr = table::get_value(self.ptr, "tarou").unwrap();
		ChCItem::from(ptr)
	}
	pub fn tarou(&self) -> CItemConst<ChCItem> {
		let ptr = table::get_value(self.ptr, "tarou").unwrap();
		CItemConst::new(ChCItem::from(ptr), self)
	}
	pub unsafe fn get_by_id_us(&self, id : ChTableID) -> ChCItem{
		match id{
			ChTableID::Tarou => self.tarou_us(),
		}
	}
	pub fn get_by_id(&self, id : ChTableID) -> CItemConst<ChCItem>{
		CItemConst::new(unsafe{ self.get_by_id_us(id) }, self)
	}
}
#[repr(u64)] pub enum ChTableID{ Tarou, }
impl ChTableID{
	pub fn from_str(id : &str) -> Option<Self>{
		match id{
			"tarou" => Some(Self::Tarou),
			_ =>{ None }
		}
	}
	pub fn from_num(id : usize) -> Self{
		match id{
			0 => Self::Tarou,
			_ => panic!("invalid ID num {} ChTableID", id),
		}
	}
	pub fn len() -> usize{ 1 }
	pub fn to_num(&self) -> usize{
		match self{
			ChTableID::Tarou => 0,
		}
	}
	pub fn metadata() -> &'static [&'static str]{
		&["tarou", ]
	}
	pub fn to_str(&self) -> &'static str{
		Self::metadata()[self.to_num()]
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ChCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for ChCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl ChCItem {
	pub fn o(&self) -> CListConst<OCItem>{
		CListConst::new(citem::get_cil(self.ptr, "o").unwrap(), self)
	}
	pub fn listen(&self) -> i64{
		let qv = citem::get_int(self.ptr, "listen").unwrap();
		qv.into_value().unwrap()
	}
	pub fn listen_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "listen").unwrap();
		qv.into_value().unwrap()
	}
	pub fn spot(&self) -> i64{
		let qv = citem::get_int(self.ptr, "spot").unwrap();
		qv.into_value().unwrap()
	}
	pub fn spot_def_val(&self) -> i64{
		let qv = citem::get_int_def(self.ptr, "spot").unwrap();
		qv.into_value().unwrap()
	}
	pub fn initial_events(&self) -> CListConst<InitialEventsCItem>{
		CListConst::new(citem::get_cil(self.ptr, "initial_events").unwrap(), self)
	}
	
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct OCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for OCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl OCItem {
	pub fn cseq(&self) -> CListConst<CseqCItem>{
		CListConst::new(citem::get_cil(self.ptr, "cseq").unwrap(), self)
	}
	
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CseqCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for CseqCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl CseqCItem {
	pub fn action(&self) -> CListConst<ActionCItem>{
		CListConst::new(citem::get_cil(self.ptr, "action").unwrap(), self)
	}
	pub fn n_def_val(&self) -> NullOr<&String>{
		let qv = citem::get_str_def(self.ptr, "n").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn n(&self) -> NullOr<&String>{
		let qv = citem::get_immutable_str(self.ptr, "n").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn cval(&self) -> CListConst<CvalCItem>{
		CListConst::new(citem::get_cil(self.ptr, "cval").unwrap(), self)
	}
	pub fn d_def_val(&self) -> NullOr<&String>{
		let qv = citem::get_str_def(self.ptr, "d").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn d(&self) -> NullOr<&String>{
		let qv = citem::get_immutable_str(self.ptr, "d").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn cc(&self) -> CListConst<CcCItem>{
		CListConst::new(citem::get_cil(self.ptr, "cc").unwrap(), self)
	}
	
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ActionCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for ActionCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl ActionCItem {
	pub fn ref_ev(&self) -> EvCItem{
		let qv = citem::get_ref(self.ptr, "ev").unwrap();
		EvCItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_ev(&self) -> &String{
		let qv = citem::get_ref_id(self.ptr, "ev").unwrap();
		qv.into_value().unwrap()
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CvalCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for CvalCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl CvalCItem {
	pub fn kind_def_val(&self) -> &String{
		let qv = citem::get_str_def(self.ptr, "kind").unwrap();
		qv.into_value().unwrap()
	}
	pub fn kind(&self) -> &String{
		let qv = citem::get_immutable_str(self.ptr, "kind").unwrap();
		qv.into_value().unwrap()
	}
	pub fn txt_def_val(&self) -> &String{
		let qv = citem::get_str_def(self.ptr, "txt").unwrap();
		qv.into_value().unwrap()
	}
	pub fn txt(&self) -> &String{
		let qv = citem::get_immutable_str(self.ptr, "txt").unwrap();
		qv.into_value().unwrap()
	}
	
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CcCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for CcCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl CcCItem {
	pub fn kind_def_val(&self) -> &String{
		let qv = citem::get_str_def(self.ptr, "kind").unwrap();
		qv.into_value().unwrap()
	}
	pub fn kind(&self) -> &String{
		let qv = citem::get_immutable_str(self.ptr, "kind").unwrap();
		qv.into_value().unwrap()
	}
	pub fn txt_def_val(&self) -> NullOr<&String>{
		let qv = citem::get_str_def(self.ptr, "txt").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn txt(&self) -> NullOr<&String>{
		let qv = citem::get_immutable_str(self.ptr, "txt").unwrap();
		NullOr::from_qv(qv).unwrap()
	}
	pub fn ref_ev(&self) -> EvCItem{
		let qv = citem::get_ref(self.ptr, "ev").unwrap();
		EvCItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_ev(&self) -> &String{
		let qv = citem::get_ref_id(self.ptr, "ev").unwrap();
		qv.into_value().unwrap()
	}
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InitialEventsCItem {
	ptr : CItemPtr,
}
impl From<CItemPtr> for InitialEventsCItem {
	fn from(ptr : CItemPtr) -> Self { Self{ ptr } }
}
impl InitialEventsCItem {
	pub fn ref_ev(&self) -> EvCItem{
		let qv = citem::get_ref(self.ptr, "ev").unwrap();
		EvCItem::from(qv.into_value().unwrap())
	}
	pub fn ref_id_ev(&self) -> &String{
		let qv = citem::get_ref_id(self.ptr, "ev").unwrap();
		qv.into_value().unwrap()
	}
}

