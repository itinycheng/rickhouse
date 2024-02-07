mod column;
mod data_type;
mod type_util;

pub use data_type::DataType;

pub type Metadata = Vec<(String, DataType)>;

pub type MetadataRef<'a> = &'a Metadata;
