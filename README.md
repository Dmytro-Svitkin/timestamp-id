# timestamp-id
`timestamp-id` is a lightweight crate for generating collision-resistant identifiers built from timestamps and random padding.

## Dependencies
`timestamp-id` has 2 dependencies: `rand` and `u8-base-converter`.

```toml
[dependencies]
rand = "0.10.3"
u8-base-converter = "0.1.1"
```

## Functionality
`timestamp-id` has 2 structs:
* `Timestamp`, which holds a timestamp in microseconds,
* `TimestampId`, which holds an idifier made from a timestamp and the random padding.

### `Timestamp` Methods
| Name | Method | Description | Input | Output |
| --- | --- | --- | --- | --- |
| New system timestamp | `new_system_timestamp` | Generates and returns a new (i.e., current) system timestamp in microseconds | ╱ | `Timestamp` |
| New system timestamp since | `new_system_timestamp_since` | Generates and returns a new (i.e., current) system timestamp in microseconds since the given offset-timestamp | `Timestamp` offset | `Timestamp` |
| New system timestamp since 2026 | `new_system_timestamp_since_2026` | Generates and returns a new (i.e., current) timestamp in microseconds since `2026-01-01 00:00.00` | ╱ | `Timestamp` |
| New timestamp from time | `from_time` | Returns a timestamp in microseconds based on the given time parameters | `i32` year, `u8` month, `u8` day, `u8` hour, `u8` minute, `f32` second | `Timestamp` |
| Timestamp as `u128` | `as_u128` | Returns a `u128` based on the timestamp | ╱ | `u128` |
| Timestamp to `String` | `to_string` | Returns a `String` based on the timestamp | ╱ | `String` |

```rust
// Example Usage

let current_time: Timestamp = Timestamp::new_system_timestamp();
const september_2026: Timestamp = Timestamp::from_time(2026, 9, 1, 0, 0, 0.0);
let current_time_since_september: Timestamp = Timestamp::new_system_timestamp_since(september_2026);

println!("{}",current_time.as_u128());
println!("{}",september_2026.as_u128());
println!("{}",currnet_time_since_semptember.to_string())
```

### `TimestampId` Methods
| Name | Method | Description | Input | Output |
| --- | --- | --- | --- | --- |
| New timestamp-id from timestamp | `from_timestamp` |
| New raw timestamp-id from timestamp | `raw_from_timestamp` |
| New timestamp-id from time | `from_time` |
| New raw timestamp-id from timestamp | `from_time` |
| New system timestamp-id | `new_system_timestamp_id` |
| New raw system timestamp-id | `new_raw_system_timestamp_id` |
| New system timestamp-id since | `new_system_timestamp_since` |
| New raw system timestamp-id since | `new_raw_system_timestamp_id_since` |
| New system timestamp-id since 2026 | `new_system_timestamp_since_2026` |
| New raw system timestamp-id since 2026 | `new_raw_system_timestamp_id_since_2026` |
| Timestamp-id as `u128` | `as_u128` |
| Timestamp-id as `String` | `to_string` |
| Alphanumerize timestamp-id | `alphanumeric` |
| Numerize timestamp-id | `numeric` |

## Installation
Add `timestamp-id` to your `Cargo.toml` dependencies:

```toml
[dependencies]
timestamp-id = "0.1.0"
```

<sub>This project is licensed under the BSD 3-Clause License.</sub>