/// Catchall Error Type

pub enum TDErrorCodes {
    InfoUpdateError
}


pub struct TDError {
    msg : &'static str,
    code: TDErrorCodes
}
