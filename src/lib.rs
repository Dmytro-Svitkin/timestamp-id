use std::time::{Duration,SystemTime,UNIX_EPOCH};
use u8_base_converter::*;

const EPOCH_2026:u128=Timestamp::from_time(2026,1,1,0,0,0.0).as_u128();
const RANDOM_PADDING:u128=62u128.pow(4);

/// Timestamp in microseconds.
pub struct Timestamp{
    pub(crate)timestamp_mcs:u128
}

/// Timestamp-id - timestamp in microseconds with random padding.
pub struct TimestampId<'a>{
    pub(crate)timestamp_id:Numeral<'a>
}

impl Timestamp{
    /// New current timestamp given by system.
    /// 
    /// Returns the current system timestamp in microseconds.
    pub fn new_system_timestamp()->Self{
        Self{
            timestamp_mcs:SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("[!] SYSTEM TIME SET TOO EARLY")
            .as_micros()
        }
    }

    /// New current timestamp given by system since the given offset-timestamp.
    /// 
    /// Returns the current system timestamp since the given offset-timestamp in microseconds.
    pub fn new_system_timestamp_since(offset:Timestamp)->Self{
        Self{
            timestamp_mcs:SystemTime::now()
            .duration_since(UNIX_EPOCH+Duration::from_micros(offset.timestamp_mcs as u64))
            .expect("[!] SYSTEM TIME SET TOO EARLY OR OFFSET OUT OF BOUNDS")
            .as_micros()
        }
    }

    /// New current timestamp given by system since `2026-01-01 00:00.00`.
    /// 
    /// Returns the current system timestamp since `2026-01-01 00:00.00` in microseconds.
    pub fn new_system_timestamp_since_2026()->Self{
        Self{
            timestamp_mcs:SystemTime::now()
            .duration_since(UNIX_EPOCH+Duration::from_micros(EPOCH_2026 as u64))
            .expect("[!] SYSTEM TIME SET TOO EARLY OR OFFSET OUT OF BOUNDS")
            .as_micros()
        }
    }

    /// Timestamp from time.
    /// 
    /// Returns a timestamp based on given time.
    pub const fn from_time(year:i32,month:u8,day:u8,hour:u8,minute:u8,second:f32)->Self{
        let year:i64=year as i64-if month<=2{1}else{0};
        let era:i64=(if year>=0{year}else{year-399})/400;
        let year_of_era:u64=(year-era*400)as u64;
        let month:u64=month as u64;
        let day_of_year:u64=(153*(if month>2{month-3}else{month+9})+2)/5+(day as u64-1);
        let day_of_era:u64=year_of_era*365+year_of_era/4-year_of_era/100+day_of_year;
        let days:u128=(era*146_097+day_of_era as i64-719468) as u128;
        let seconds:u128=days*86400+(hour as u128)*3600+(minute as u128)*60;
        
        Self{timestamp_mcs:seconds*1000000+(second*1000000.0)as u128}
    }

    /// Timestamp as `u128`.
    /// 
    /// Returns an `u128` value based on the given timestamp.
    pub const fn as_u128(&self)->u128{
        self.timestamp_mcs
    }

    /// Timestamp to `String`.
    /// 
    /// Returns a `String` based on timestamp.
    pub fn to_string(&self)->String{
        self.timestamp_mcs.to_string()
    }
}

impl<'a>TimestampId<'a>{
    /// Timestamp-id from timestamp.
    /// 
    /// Returns a timestamp-id based on the given timestamp.
    pub fn from_timestamp(timestamp:Timestamp)->Self{
        let timestamp_id:u128=timestamp.timestamp_mcs*RANDOM_PADDING+rand::random_range(0..RANDOM_PADDING);// Concat of timestamp and random.
        Self{timestamp_id:Numeral::new_dec_from_u128(timestamp_id)}
    }

    /// Raw timestamp-id from timestamp.
    /// 
    /// Returns a raw (i.e., without the random padding) timestamp-id based on the given timestamp.
    pub const fn raw_from_timestamp(timestamp:Timestamp)->Self{
        let timestamp_id:u128=timestamp.timestamp_mcs;
        Self{timestamp_id:Numeral::new_dec_from_u128(timestamp_id)}
    }

    /// Timestamp-id from time.
    /// 
    /// Returns a timestamp-id from the given time.
    pub fn from_time(year:i32,month:u8,day:u8,hour:u8,minute:u8,second:f32)->Self{
        TimestampId::from_timestamp(Timestamp::from_time(year,month,day,hour,minute,second))
    }

    /// Raw timestamp-id from time.
    /// 
    /// Returns a raw (i.e., without the random padding) timestamp-id from the given time.
    pub const fn raw_from_time(year:i32,month:u8,day:u8,hour:u8,minute:u8,second:f32)->Self{
        TimestampId::raw_from_timestamp(Timestamp::from_time(year,month,day,hour,minute,second))
    }

    /// New timestamp-id given by system.
    /// 
    /// Returns a new timestamp-id based on the timestamp given by system and a the random padding.
    pub fn new_system_timestamp_id()->Self{
        TimestampId::from_timestamp(Timestamp::new_system_timestamp())
    }

    /// New raw timestamp-id given by system.
    /// 
    /// Returns a new raw (i.e., without the random padding) based on the timestamp given by system.
    pub fn new_raw_system_timestamp_id()->Self{
        TimestampId::raw_from_timestamp(Timestamp::new_system_timestamp())
    }

    /// New timestamp-id given by system since the given offset-timestamp.
    /// 
    /// Returns a new timestamp-id based on the timestamp given by system since the given offset-timestamp in microseconds and a the random padding.
    pub fn new_system_timestamp_id_since(offset:Timestamp)->Self{
        TimestampId::from_timestamp(Timestamp::new_system_timestamp_since(offset))
    }

    /// New raw timestamp-id given by system since the given offset-timestamp.
    /// 
    /// Returns a new raw (i.e., without the random padding) based on the timestamp given by system since the given offset-timestamp in microseconds.
    pub fn new_raw_system_timestamp_id_since(offset:Timestamp)->Self{
        TimestampId::raw_from_timestamp(Timestamp::new_system_timestamp_since(offset))
    }

    /// New timestamp-id given by system since `2026-01-01 00:00.00`.
    /// 
    /// Returns a new timestamp-id based on the timestamp given by system since `2026-01-01 00:00.00` and a the random padding.
    pub fn new_system_timestamp_id_since_2026()->Self{
        TimestampId::from_timestamp(Timestamp::new_system_timestamp_since_2026())
    }

    /// New raw timestamp-id given by system since `2026-01-01 00:00.00`.
    /// 
    /// Returns a new raw (i.e., without the random padding) based on the timestamp given by system since `2026-01-01 00:00.00`.
    pub fn new_raw_system_timestamp_id_since_2026()->Self{
        TimestampId::raw_from_timestamp(Timestamp::new_system_timestamp_since_2026())
    }

    /// Timestamp-id as `u128`.
    /// 
    /// Returns an `u128` value based on the timestamp-id.
    pub const fn as_u128(&self)->u128{
        self.timestamp_id.as_u128().unwrap()// Overflow is not realistic here, so unwrap.
    }

    /// Timestamp-id as string slice (`&str`).
    ///
    /// Returns a string slice (`&str`) based on timestamp-id.
    pub const fn as_str(&self)->&str{
        self.timestamp_id.value_as_str()
    }

    /// Timestamp-id to `String`.
    /// 
    /// Returns a `String` based on timestamp-id.
    pub fn to_string(&self)->String{
        self.as_str().to_owned()
    }

    /// Timestamp-id to alphanumeric timestamp-id.
    /// 
    /// Returns a converted to alphanumeric (i.e., base62) timestamp-id based on timestamp-id numeral.
    pub const fn alphanumeric(&self)->Self{
        Self{timestamp_id:self.timestamp_id.converted_to(ALPHANUMERIC)}
    }

    /// Timestamp-id to numeric timestamp-id.
    /// 
    /// Returns a converted to numeric (i.e., decimal) timestamp-id based on timestamp-id numeral.
    pub const fn numeric(&self)->Self{
        Self{timestamp_id:self.timestamp_id.converted_to(DECIMAL)}
    }
}