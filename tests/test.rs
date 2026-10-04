use timestamp_id::*;

#[test]
fn test_timestamp(){
    let a:Timestamp=Timestamp::new_system_timestamp();
    println!("{}\n{}",a.to_string(),a.as_u128())
}