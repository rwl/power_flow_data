use nom::IResult;

pub use power_flow_data_derive::RawRecord;

pub trait RawRecord {
    fn parse_raw(input: &str) -> IResult<&str, Self>
    where
        Self: Sized;
}
