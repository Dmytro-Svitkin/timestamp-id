use timestamp_id::*;

#[test]
fn test_timestamp(){
    let a:Timestamp=Timestamp::new_system_timestamp();
    println!("{}\n{}",a.to_string(),a.as_u128())
}

#[test]
fn test_timestamp_id(){
    let current_timestamp_id:TimestampId=TimestampId::new_system_timestamp_id().alphanumeric();
    let raw_current_timestamp_id:TimestampId=TimestampId::new_raw_system_timestamp_id().alphanumeric();

    println!("{}",current_timestamp_id.as_str());
    println!("{}",raw_current_timestamp_id.as_str())
}