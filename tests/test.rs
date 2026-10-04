use timestamp_id::*;

#[test]
fn test_timestamp(){
    let current_timestamp:Timestamp=Timestamp::new_system_timestamp();
    let timestamp_since_2026:Timestamp=Timestamp::new_system_timestamp_since_2026();

    println!("{}\n{}",current_timestamp.to_string(),current_timestamp.as_u128());
    println!("{}",timestamp_since_2026.as_u128())
}

#[test]
fn test_timestamp_id(){
    let current_timestamp_id:TimestampId=TimestampId::new_system_timestamp_id().alphanumeric();
    let raw_current_timestamp_id:TimestampId=TimestampId::new_raw_system_timestamp_id().alphanumeric();

    println!("{}",current_timestamp_id.as_str());
    println!("{}",raw_current_timestamp_id.as_str())    
}