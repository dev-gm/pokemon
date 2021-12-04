pub enum DictValue {
    Null,
    String(String),
    Char(char),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    U128(u128),
    I128(i128),
    F32(f32),
    F64(f64),
    Array(Vec<DictValue>),
    Dict(Dict),
    Func(fn(dict: &Dict) -> DictValue),
    FuncMut(fn(dict: &mut Dict) -> DictValue),
    Object(Box<dyn IsDictValue>),
}

pub type Dict = HashMap<String, DictValue>;

pub trait IsDictValue {}

