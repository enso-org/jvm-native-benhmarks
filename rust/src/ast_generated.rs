// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Ast {
  NONE = 0,
  App = 1,
  Name = 2,
  Offset = 3,
  Position = 4,

}

pub const ENUM_MIN_AST: u8 = 0;
pub const ENUM_MAX_AST: u8 = 4;

impl<'a> flatbuffers::Follow<'a> for Ast {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for Ast {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const Ast;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const Ast;
    unsafe { *p }
  }
}

impl flatbuffers::Push for Ast {
    type Output = Ast;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<Ast>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
pub const ENUM_VALUES_AST:[Ast; 5] = [
  Ast::NONE,
  Ast::App,
  Ast::Name,
  Ast::Offset,
  Ast::Position
];

#[allow(non_camel_case_types)]
pub const ENUM_NAMES_AST:[&'static str; 5] = [
    "NONE",
    "App",
    "Name",
    "Offset",
    "Position"
];

pub fn enum_name_ast(e: Ast) -> &'static str {
  let index = e as u8;
  ENUM_NAMES_AST[index as usize]
}

pub struct AstUnionTableOffset {}
pub enum AppOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct App<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for App<'a> {
    type Inner = App<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> App<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        App {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args AppArgs) -> flatbuffers::WIPOffset<App<'bldr>> {
      let mut builder = AppBuilder::new(_fbb);
      if let Some(x) = args.arg { builder.add_arg(x); }
      if let Some(x) = args.fun { builder.add_fun(x); }
      builder.add_arg_type(args.arg_type);
      builder.add_fun_type(args.fun_type);
      builder.finish()
    }

    pub const VT_FUN_TYPE: flatbuffers::VOffsetT = 4;
    pub const VT_FUN: flatbuffers::VOffsetT = 6;
    pub const VT_ARG_TYPE: flatbuffers::VOffsetT = 8;
    pub const VT_ARG: flatbuffers::VOffsetT = 10;

  #[inline]
  pub fn fun_type(&self) -> Ast {
    self._tab.get::<Ast>(App::VT_FUN_TYPE, Some(Ast::NONE)).unwrap()
  }
  #[inline]
  pub fn fun(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(App::VT_FUN, None)
  }
  #[inline]
  pub fn arg_type(&self) -> Ast {
    self._tab.get::<Ast>(App::VT_ARG_TYPE, Some(Ast::NONE)).unwrap()
  }
  #[inline]
  pub fn arg(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(App::VT_ARG, None)
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn fun_as_app(&self) -> Option<App<'a>> {
    if self.fun_type() == Ast::App {
      self.fun().map(|u| App::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn fun_as_name(&self) -> Option<Name<'a>> {
    if self.fun_type() == Ast::Name {
      self.fun().map(|u| Name::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn fun_as_offset(&self) -> Option<Offset<'a>> {
    if self.fun_type() == Ast::Offset {
      self.fun().map(|u| Offset::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn fun_as_position(&self) -> Option<Position<'a>> {
    if self.fun_type() == Ast::Position {
      self.fun().map(|u| Position::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn arg_as_app(&self) -> Option<App<'a>> {
    if self.arg_type() == Ast::App {
      self.arg().map(|u| App::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn arg_as_name(&self) -> Option<Name<'a>> {
    if self.arg_type() == Ast::Name {
      self.arg().map(|u| Name::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn arg_as_offset(&self) -> Option<Offset<'a>> {
    if self.arg_type() == Ast::Offset {
      self.arg().map(|u| Offset::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn arg_as_position(&self) -> Option<Position<'a>> {
    if self.arg_type() == Ast::Position {
      self.arg().map(|u| Position::init_from_table(u))
    } else {
      None
    }
  }

}

pub struct AppArgs {
    pub fun_type: Ast,
    pub fun: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
    pub arg_type: Ast,
    pub arg: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for AppArgs {
    #[inline]
    fn default() -> Self {
        AppArgs {
            fun_type: Ast::NONE,
            fun: None,
            arg_type: Ast::NONE,
            arg: None,
        }
    }
}
pub struct AppBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> AppBuilder<'a, 'b> {
  #[inline]
  pub fn add_fun_type(&mut self, fun_type: Ast) {
    self.fbb_.push_slot::<Ast>(App::VT_FUN_TYPE, fun_type, Ast::NONE);
  }
  #[inline]
  pub fn add_fun(&mut self, fun: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(App::VT_FUN, fun);
  }
  #[inline]
  pub fn add_arg_type(&mut self, arg_type: Ast) {
    self.fbb_.push_slot::<Ast>(App::VT_ARG_TYPE, arg_type, Ast::NONE);
  }
  #[inline]
  pub fn add_arg(&mut self, arg: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(App::VT_ARG, arg);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> AppBuilder<'a, 'b> {
    let start = _fbb.start_table();
    AppBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<App<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum NameOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Name<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Name<'a> {
    type Inner = Name<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Name<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Name {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args NameArgs<'args>) -> flatbuffers::WIPOffset<Name<'bldr>> {
      let mut builder = NameBuilder::new(_fbb);
      if let Some(x) = args.str { builder.add_str(x); }
      builder.finish()
    }

    pub const VT_STR: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn str(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Name::VT_STR, None)
  }
}

pub struct NameArgs<'a> {
    pub str: Option<flatbuffers::WIPOffset<&'a  str>>,
}
impl<'a> Default for NameArgs<'a> {
    #[inline]
    fn default() -> Self {
        NameArgs {
            str: None,
        }
    }
}
pub struct NameBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> NameBuilder<'a, 'b> {
  #[inline]
  pub fn add_str(&mut self, str: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Name::VT_STR, str);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> NameBuilder<'a, 'b> {
    let start = _fbb.start_table();
    NameBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Name<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum OffsetOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Offset<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Offset<'a> {
    type Inner = Offset<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Offset<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Offset {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args OffsetArgs) -> flatbuffers::WIPOffset<Offset<'bldr>> {
      let mut builder = OffsetBuilder::new(_fbb);
      builder.add_value(args.value);
      builder.finish()
    }

    pub const VT_VALUE: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn value(&self) -> i64 {
    self._tab.get::<i64>(Offset::VT_VALUE, Some(0)).unwrap()
  }
}

pub struct OffsetArgs {
    pub value: i64,
}
impl<'a> Default for OffsetArgs {
    #[inline]
    fn default() -> Self {
        OffsetArgs {
            value: 0,
        }
    }
}
pub struct OffsetBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> OffsetBuilder<'a, 'b> {
  #[inline]
  pub fn add_value(&mut self, value: i64) {
    self.fbb_.push_slot::<i64>(Offset::VT_VALUE, value, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> OffsetBuilder<'a, 'b> {
    let start = _fbb.start_table();
    OffsetBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Offset<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum PositionOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Position<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Position<'a> {
    type Inner = Position<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Position<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Position {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args PositionArgs) -> flatbuffers::WIPOffset<Position<'bldr>> {
      let mut builder = PositionBuilder::new(_fbb);
      builder.add_to(args.to);
      builder.add_at(args.at);
      builder.finish()
    }

    pub const VT_AT: flatbuffers::VOffsetT = 4;
    pub const VT_TO: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn at(&self) -> i64 {
    self._tab.get::<i64>(Position::VT_AT, Some(0)).unwrap()
  }
  #[inline]
  pub fn to(&self) -> i64 {
    self._tab.get::<i64>(Position::VT_TO, Some(0)).unwrap()
  }
}

pub struct PositionArgs {
    pub at: i64,
    pub to: i64,
}
impl<'a> Default for PositionArgs {
    #[inline]
    fn default() -> Self {
        PositionArgs {
            at: 0,
            to: 0,
        }
    }
}
pub struct PositionBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> PositionBuilder<'a, 'b> {
  #[inline]
  pub fn add_at(&mut self, at: i64) {
    self.fbb_.push_slot::<i64>(Position::VT_AT, at, 0);
  }
  #[inline]
  pub fn add_to(&mut self, to: i64) {
    self.fbb_.push_slot::<i64>(Position::VT_TO, to, 0);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> PositionBuilder<'a, 'b> {
    let start = _fbb.start_table();
    PositionBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Position<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}
