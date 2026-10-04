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
| New system timestamp since | `new_system_timestamp_since` ||  | `Timestamp` |
| New system timestamp since 2026 | `new_system_timestamp_since_2026` || ╱ | `Timestamp` |
| New timestamp from time | `from_time` ||||
| Timestamp as `u128` | `as_u128` || ╱ ||
| Timestamp to `String` | `to_string` || ╱ ||

### `TimestampId` Methods

## Installation
Add `timestamp-id` to your `Cargo.toml` dependencies:

```toml
[dependencies]
timestamp-id = "0.1.0"
```

<sub>This project is licensed under the BSD 3-Clause License.</sub>