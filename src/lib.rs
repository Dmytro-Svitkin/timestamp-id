use u8_base_converter::{ALPHANUMERIC, DECIMAL, base_to_str, convert, str_to_base};

pub fn get_current_system_timestamp()->u128{
    use std::time::{Duration,SystemTime};

    SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH+Duration::from_micros(time_to_timnestamp(2026,1,1,0,0,0.0)as u64))
    .expect("[!] SYSTEM TIME SET TOO EARLY")
    .as_micros()// Absolute time all the way from 2026-01-01 00:00 in microseconds.
}

fn random_base62(x:u8)->u32{
    rand::random_range(0..62u32.pow(x as u32))
}

pub fn new_id()->String{
    new_id_u128().to_string()
}

pub fn new_id_u128()->u128{
    get_current_system_timestamp()*62u128.pow(2)+random_base62(2) as u128
}

pub fn new_base62_id()->String{
    base_to_str(&convert(str_to_base(&new_id()),&DECIMAL,&ALPHANUMERIC)).to_string()
}

pub fn id_u128_from_timestamp(timestamp:u128)->u128{
    timestamp*62u128.pow(2)+random_base62(2) as u128
}

pub fn id_from_timestamp(timestamp:u128)->String{
    id_u128_from_timestamp(timestamp).to_string()
}

pub fn base62_id_from_timestamp(timestamp:u128)->String{
    base_to_str(&convert(str_to_base(&id_u128_from_timestamp(timestamp).to_string()),&DECIMAL,&ALPHANUMERIC)).to_string()
}

pub const fn time_to_timnestamp(year:i32,month:u8,day:u8,hour:u8,minute:u8,second:f32,)->u128{
    let year:i64=year as i64-if month<=2{1}else{0};
    let era:i64=(if year>=0{year}else{year-399})/400;
    let year_of_era:u64=(year-era*400)as u64;
    let month:u64=month as u64;
    let day_of_year:u64=(153*(if month>2{month-3}else{month+9})+2)/5+(day as u64-1);
    let day_of_era:u64=year_of_era*365+year_of_era/4-year_of_era/100+day_of_year;
    let days:u128=(era*146_097+day_of_era as i64-719468) as u128;
    let seconds:u128=days*86400+(hour as u128)*3600+(minute as u128)*60;
    seconds*1000000+(second*1000000.0)as u128
}