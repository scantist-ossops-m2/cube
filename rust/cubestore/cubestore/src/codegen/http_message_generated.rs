// automatically generated by the FlatBuffers compiler, do not modify



use std::mem;
use std::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::EndianScalar;

#[allow(non_camel_case_types)]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum HttpCommand {
  NONE = 0,
  HttpQuery = 1,
  HttpResultSet = 2,
  HttpError = 3,

}

pub const ENUM_MIN_HTTP_COMMAND: u8 = 0;
pub const ENUM_MAX_HTTP_COMMAND: u8 = 3;

impl<'a> flatbuffers::Follow<'a> for HttpCommand {
  type Inner = Self;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::read_scalar_at::<Self>(buf, loc)
  }
}

impl flatbuffers::EndianScalar for HttpCommand {
  #[inline]
  fn to_little_endian(self) -> Self {
    let n = u8::to_le(self as u8);
    let p = &n as *const u8 as *const HttpCommand;
    unsafe { *p }
  }
  #[inline]
  fn from_little_endian(self) -> Self {
    let n = u8::from_le(self as u8);
    let p = &n as *const u8 as *const HttpCommand;
    unsafe { *p }
  }
}

impl flatbuffers::Push for HttpCommand {
    type Output = HttpCommand;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        flatbuffers::emplace_scalar::<HttpCommand>(dst, *self);
    }
}

#[allow(non_camel_case_types)]
pub const ENUM_VALUES_HTTP_COMMAND:[HttpCommand; 4] = [
  HttpCommand::NONE,
  HttpCommand::HttpQuery,
  HttpCommand::HttpResultSet,
  HttpCommand::HttpError
];

#[allow(non_camel_case_types)]
pub const ENUM_NAMES_HTTP_COMMAND:[&'static str; 4] = [
    "NONE",
    "HttpQuery",
    "HttpResultSet",
    "HttpError"
];

pub fn enum_name_http_command(e: HttpCommand) -> &'static str {
  let index = e as u8;
  ENUM_NAMES_HTTP_COMMAND[index as usize]
}

pub struct HttpCommandUnionTableOffset {}
pub enum HttpMessageOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct HttpMessage<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for HttpMessage<'a> {
    type Inner = HttpMessage<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> HttpMessage<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        HttpMessage {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args HttpMessageArgs) -> flatbuffers::WIPOffset<HttpMessage<'bldr>> {
      let mut builder = HttpMessageBuilder::new(_fbb);
      if let Some(x) = args.command { builder.add_command(x); }
      builder.add_message_id(args.message_id);
      builder.add_command_type(args.command_type);
      builder.finish()
    }

    pub const VT_MESSAGE_ID: flatbuffers::VOffsetT = 4;
    pub const VT_COMMAND_TYPE: flatbuffers::VOffsetT = 6;
    pub const VT_COMMAND: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn message_id(&self) -> u32 {
    self._tab.get::<u32>(HttpMessage::VT_MESSAGE_ID, Some(0)).unwrap()
  }
  #[inline]
  pub fn command_type(&self) -> HttpCommand {
    self._tab.get::<HttpCommand>(HttpMessage::VT_COMMAND_TYPE, Some(HttpCommand::NONE)).unwrap()
  }
  #[inline]
  pub fn command(&self) -> Option<flatbuffers::Table<'a>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(HttpMessage::VT_COMMAND, None)
  }
  #[inline]
  #[allow(non_snake_case)]
  pub fn command_as_http_query(&self) -> Option<HttpQuery<'a>> {
    if self.command_type() == HttpCommand::HttpQuery {
      self.command().map(|u| HttpQuery::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn command_as_http_result_set(&self) -> Option<HttpResultSet<'a>> {
    if self.command_type() == HttpCommand::HttpResultSet {
      self.command().map(|u| HttpResultSet::init_from_table(u))
    } else {
      None
    }
  }

  #[inline]
  #[allow(non_snake_case)]
  pub fn command_as_http_error(&self) -> Option<HttpError<'a>> {
    if self.command_type() == HttpCommand::HttpError {
      self.command().map(|u| HttpError::init_from_table(u))
    } else {
      None
    }
  }

}

pub struct HttpMessageArgs {
    pub message_id: u32,
    pub command_type: HttpCommand,
    pub command: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
}
impl<'a> Default for HttpMessageArgs {
    #[inline]
    fn default() -> Self {
        HttpMessageArgs {
            message_id: 0,
            command_type: HttpCommand::NONE,
            command: None,
        }
    }
}
pub struct HttpMessageBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HttpMessageBuilder<'a, 'b> {
  #[inline]
  pub fn add_message_id(&mut self, message_id: u32) {
    self.fbb_.push_slot::<u32>(HttpMessage::VT_MESSAGE_ID, message_id, 0);
  }
  #[inline]
  pub fn add_command_type(&mut self, command_type: HttpCommand) {
    self.fbb_.push_slot::<HttpCommand>(HttpMessage::VT_COMMAND_TYPE, command_type, HttpCommand::NONE);
  }
  #[inline]
  pub fn add_command(&mut self, command: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpMessage::VT_COMMAND, command);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HttpMessageBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HttpMessageBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<HttpMessage<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum HttpQueryOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct HttpQuery<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for HttpQuery<'a> {
    type Inner = HttpQuery<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> HttpQuery<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        HttpQuery {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args HttpQueryArgs<'args>) -> flatbuffers::WIPOffset<HttpQuery<'bldr>> {
      let mut builder = HttpQueryBuilder::new(_fbb);
      if let Some(x) = args.inline_tables { builder.add_inline_tables(x); }
      if let Some(x) = args.trace_obj { builder.add_trace_obj(x); }
      if let Some(x) = args.query { builder.add_query(x); }
      builder.finish()
    }

    pub const VT_QUERY: flatbuffers::VOffsetT = 4;
    pub const VT_TRACE_OBJ: flatbuffers::VOffsetT = 6;
    pub const VT_INLINE_TABLES: flatbuffers::VOffsetT = 8;

  #[inline]
  pub fn query(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(HttpQuery::VT_QUERY, None)
  }
  #[inline]
  pub fn trace_obj(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(HttpQuery::VT_TRACE_OBJ, None)
  }
  #[inline]
  pub fn inline_tables(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<HttpTable<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<HttpTable<'a>>>>>(HttpQuery::VT_INLINE_TABLES, None)
  }
}

pub struct HttpQueryArgs<'a> {
    pub query: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub trace_obj: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub inline_tables: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<HttpTable<'a >>>>>,
}
impl<'a> Default for HttpQueryArgs<'a> {
    #[inline]
    fn default() -> Self {
        HttpQueryArgs {
            query: None,
            trace_obj: None,
            inline_tables: None,
        }
    }
}
pub struct HttpQueryBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HttpQueryBuilder<'a, 'b> {
  #[inline]
  pub fn add_query(&mut self, query: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpQuery::VT_QUERY, query);
  }
  #[inline]
  pub fn add_trace_obj(&mut self, trace_obj: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpQuery::VT_TRACE_OBJ, trace_obj);
  }
  #[inline]
  pub fn add_inline_tables(&mut self, inline_tables: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<HttpTable<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpQuery::VT_INLINE_TABLES, inline_tables);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HttpQueryBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HttpQueryBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<HttpQuery<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum HttpTableOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct HttpTable<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for HttpTable<'a> {
    type Inner = HttpTable<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> HttpTable<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        HttpTable {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args HttpTableArgs<'args>) -> flatbuffers::WIPOffset<HttpTable<'bldr>> {
      let mut builder = HttpTableBuilder::new(_fbb);
      if let Some(x) = args.rows { builder.add_rows(x); }
      if let Some(x) = args.csv_rows { builder.add_csv_rows(x); }
      if let Some(x) = args.types { builder.add_types(x); }
      if let Some(x) = args.columns { builder.add_columns(x); }
      if let Some(x) = args.name { builder.add_name(x); }
      builder.finish()
    }

    pub const VT_NAME: flatbuffers::VOffsetT = 4;
    pub const VT_COLUMNS: flatbuffers::VOffsetT = 6;
    pub const VT_TYPES: flatbuffers::VOffsetT = 8;
    pub const VT_CSV_ROWS: flatbuffers::VOffsetT = 10;
    pub const VT_ROWS: flatbuffers::VOffsetT = 12;

  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(HttpTable::VT_NAME, None)
  }
  #[inline]
  pub fn columns(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<&'a str>>>>(HttpTable::VT_COLUMNS, None)
  }
  #[inline]
  pub fn types(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<&'a str>>>>(HttpTable::VT_TYPES, None)
  }
  #[inline]
  pub fn csv_rows(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(HttpTable::VT_CSV_ROWS, None)
  }
  #[inline]
  pub fn rows(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<HttpRow<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<HttpRow<'a>>>>>(HttpTable::VT_ROWS, None)
  }
}

pub struct HttpTableArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub columns: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<&'a  str>>>>,
    pub types: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<&'a  str>>>>,
    pub csv_rows: Option<flatbuffers::WIPOffset<&'a  str>>,
    pub rows: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<HttpRow<'a >>>>>,
}
impl<'a> Default for HttpTableArgs<'a> {
    #[inline]
    fn default() -> Self {
        HttpTableArgs {
            name: None,
            columns: None,
            types: None,
            csv_rows: None,
            rows: None,
        }
    }
}
pub struct HttpTableBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HttpTableBuilder<'a, 'b> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpTable::VT_NAME, name);
  }
  #[inline]
  pub fn add_columns(&mut self, columns: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpTable::VT_COLUMNS, columns);
  }
  #[inline]
  pub fn add_types(&mut self, types: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpTable::VT_TYPES, types);
  }
  #[inline]
  pub fn add_csv_rows(&mut self, csv_rows: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpTable::VT_CSV_ROWS, csv_rows);
  }
  #[inline]
  pub fn add_rows(&mut self, rows: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<HttpRow<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpTable::VT_ROWS, rows);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HttpTableBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HttpTableBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<HttpTable<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum HttpErrorOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct HttpError<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for HttpError<'a> {
    type Inner = HttpError<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> HttpError<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        HttpError {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args HttpErrorArgs<'args>) -> flatbuffers::WIPOffset<HttpError<'bldr>> {
      let mut builder = HttpErrorBuilder::new(_fbb);
      if let Some(x) = args.error { builder.add_error(x); }
      builder.finish()
    }

    pub const VT_ERROR: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn error(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(HttpError::VT_ERROR, None)
  }
}

pub struct HttpErrorArgs<'a> {
    pub error: Option<flatbuffers::WIPOffset<&'a  str>>,
}
impl<'a> Default for HttpErrorArgs<'a> {
    #[inline]
    fn default() -> Self {
        HttpErrorArgs {
            error: None,
        }
    }
}
pub struct HttpErrorBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HttpErrorBuilder<'a, 'b> {
  #[inline]
  pub fn add_error(&mut self, error: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpError::VT_ERROR, error);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HttpErrorBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HttpErrorBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<HttpError<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum HttpResultSetOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct HttpResultSet<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for HttpResultSet<'a> {
    type Inner = HttpResultSet<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> HttpResultSet<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        HttpResultSet {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args HttpResultSetArgs<'args>) -> flatbuffers::WIPOffset<HttpResultSet<'bldr>> {
      let mut builder = HttpResultSetBuilder::new(_fbb);
      if let Some(x) = args.rows { builder.add_rows(x); }
      if let Some(x) = args.columns { builder.add_columns(x); }
      builder.finish()
    }

    pub const VT_COLUMNS: flatbuffers::VOffsetT = 4;
    pub const VT_ROWS: flatbuffers::VOffsetT = 6;

  #[inline]
  pub fn columns(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<&'a str>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<&'a str>>>>(HttpResultSet::VT_COLUMNS, None)
  }
  #[inline]
  pub fn rows(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<HttpRow<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<HttpRow<'a>>>>>(HttpResultSet::VT_ROWS, None)
  }
}

pub struct HttpResultSetArgs<'a> {
    pub columns: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<&'a  str>>>>,
    pub rows: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<HttpRow<'a >>>>>,
}
impl<'a> Default for HttpResultSetArgs<'a> {
    #[inline]
    fn default() -> Self {
        HttpResultSetArgs {
            columns: None,
            rows: None,
        }
    }
}
pub struct HttpResultSetBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HttpResultSetBuilder<'a, 'b> {
  #[inline]
  pub fn add_columns(&mut self, columns: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<&'b  str>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpResultSet::VT_COLUMNS, columns);
  }
  #[inline]
  pub fn add_rows(&mut self, rows: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<HttpRow<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpResultSet::VT_ROWS, rows);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HttpResultSetBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HttpResultSetBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<HttpResultSet<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum HttpRowOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct HttpRow<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for HttpRow<'a> {
    type Inner = HttpRow<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> HttpRow<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        HttpRow {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args HttpRowArgs<'args>) -> flatbuffers::WIPOffset<HttpRow<'bldr>> {
      let mut builder = HttpRowBuilder::new(_fbb);
      if let Some(x) = args.values { builder.add_values(x); }
      builder.finish()
    }

    pub const VT_VALUES: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn values(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<HttpColumnValue<'a>>>> {
    self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<flatbuffers::ForwardsUOffset<HttpColumnValue<'a>>>>>(HttpRow::VT_VALUES, None)
  }
}

pub struct HttpRowArgs<'a> {
    pub values: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a , flatbuffers::ForwardsUOffset<HttpColumnValue<'a >>>>>,
}
impl<'a> Default for HttpRowArgs<'a> {
    #[inline]
    fn default() -> Self {
        HttpRowArgs {
            values: None,
        }
    }
}
pub struct HttpRowBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HttpRowBuilder<'a, 'b> {
  #[inline]
  pub fn add_values(&mut self, values: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<HttpColumnValue<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpRow::VT_VALUES, values);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HttpRowBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HttpRowBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<HttpRow<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

pub enum HttpColumnValueOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct HttpColumnValue<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for HttpColumnValue<'a> {
    type Inner = HttpColumnValue<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> HttpColumnValue<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        HttpColumnValue {
            _tab: table,
        }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args HttpColumnValueArgs<'args>) -> flatbuffers::WIPOffset<HttpColumnValue<'bldr>> {
      let mut builder = HttpColumnValueBuilder::new(_fbb);
      if let Some(x) = args.string_value { builder.add_string_value(x); }
      builder.finish()
    }

    pub const VT_STRING_VALUE: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn string_value(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(HttpColumnValue::VT_STRING_VALUE, None)
  }
}

pub struct HttpColumnValueArgs<'a> {
    pub string_value: Option<flatbuffers::WIPOffset<&'a  str>>,
}
impl<'a> Default for HttpColumnValueArgs<'a> {
    #[inline]
    fn default() -> Self {
        HttpColumnValueArgs {
            string_value: None,
        }
    }
}
pub struct HttpColumnValueBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> HttpColumnValueBuilder<'a, 'b> {
  #[inline]
  pub fn add_string_value(&mut self, string_value: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(HttpColumnValue::VT_STRING_VALUE, string_value);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> HttpColumnValueBuilder<'a, 'b> {
    let start = _fbb.start_table();
    HttpColumnValueBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<HttpColumnValue<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

#[inline]
pub fn get_root_as_http_message<'a>(buf: &'a [u8]) -> HttpMessage<'a> {
  flatbuffers::get_root::<HttpMessage<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_http_message<'a>(buf: &'a [u8]) -> HttpMessage<'a> {
  flatbuffers::get_size_prefixed_root::<HttpMessage<'a>>(buf)
}

#[inline]
pub fn finish_http_message_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<HttpMessage<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_http_message_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<HttpMessage<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
