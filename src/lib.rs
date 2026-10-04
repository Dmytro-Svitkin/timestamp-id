use std::time::{Duration,SystemTime,UNIX_EPOCH};
use u8_base_converter::*;

const RANDOM_PADDING:u128=14776336;


pub struct Timestamp{
    pub(crate)timestamp_mcs:u128
}

pub struct TimestampId<'a>{
    pub(crate)timestamp_id:Numeral<'a>
}

impl Timestamp{
    pub fn new_system_timestamp()->Self{
        Self{
            timestamp_mcs:SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("[!] SYSTEM TIME SET TOO EARLY")
            .as_micros()
        }
    }

    pub fn new_system_timestamp_since(offset:Timestamp)->Self{
        Self{
            timestamp_mcs:SystemTime::now()
            .duration_since(UNIX_EPOCH+Duration::from_micros(offset.timestamp_mcs as u64))
            .expect("[!] SYSTEM TIME SET TOO EARLY OR OFFSET OUT OF BOUNDS")
            .as_micros()
        }
    }

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

    pub const fn as_u128(&self)->u128{
        self.timestamp_mcs
    }

    pub fn to_string(&self)->String{
        self.timestamp_mcs.to_string()
    }
}

impl<'a>TimestampId<'a>{
    pub fn from_timestamp(timestamp:Timestamp)->Self{
        let timestamp_id:u128=timestamp.timestamp_mcs*RANDOM_PADDING+rand::random_range(0..RANDOM_PADDING);// Concat of timestamp and random.
        Self{timestamp_id:Numeral::new_dec_from_u128(timestamp_id)}
    }

    pub const fn raw_from_timestamp(timestamp:Timestamp)->Self{
        let timestamp_id:u128=timestamp.timestamp_mcs*RANDOM_PADDING;
        Self{timestamp_id:Numeral::new_dec_from_u128(timestamp_id)}
    }

    pub fn from_time(year:i32,month:u8,day:u8,hour:u8,minute:u8,second:f32)->Self{
        TimestampId::from_timestamp(Timestamp::from_time(year,month,day,hour,minute,second))
    }

    pub const fn raw_from_time(year:i32,month:u8,day:u8,hour:u8,minute:u8,second:f32)->Self{
        TimestampId::raw_from_timestamp(Timestamp::from_time(year,month,day,hour,minute,second))
    }

    pub fn new_system_timestamp_id()->Self{
        TimestampId::from_timestamp(Timestamp::new_system_timestamp())
    }

    pub fn new_raw_system_timestamp_id()->Self{
        TimestampId::raw_from_timestamp(Timestamp::new_system_timestamp())
    }

    pub const fn as_u128(&self)->u128{
        self.timestamp_id.as_u128().unwrap()// Overflow is not realistic here, so unwrap.
    }

    pub fn as_str(&self)->&str{
        unsafe{self.timestamp_id.value_as_str_unchecked()}
    }

    pub fn to_string(&self)->String{
        self.as_str().to_owned()
    }

    pub fn alphanumeric(&self)->Self{
        Self{timestamp_id:self.timestamp_id.converted_to(ALPHANUMERIC)}
    }
}