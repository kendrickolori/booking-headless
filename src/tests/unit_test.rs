use crate::{
    structs::util_struct::TimeSlot,
    utils::others_utils::{convert_to_local_primitive, generate_slots},
};
use time::{Duration, macros::datetime};

#[test]
fn test_timezone_conversion_works() {
    // Setup: Define a UTC time (e.g., 10:00 AM UTC)
    let utc_time = datetime!(2025-12-25 10:00:00 UTC);

    // Execution: Convert to Nigeria time (UTC+1)
    let result = convert_to_local_primitive(utc_time, "Africa/Lagos");

    // Assertion: It should succeed
    assert!(result.is_ok());

    // Verification: 10:00 UTC should be 11:00 in Lagos
    let local_time = result.unwrap();
    assert_eq!(local_time.hour(), 11);
}

#[test]
fn test_invalid_timezone_fails() {
    let utc_time = datetime!(2025-12-25 10:00:00 UTC);

    let result = convert_to_local_primitive(utc_time, "Asia/Lagos");

    assert!(result.is_err());
}

/* -------------------------------------------------------------------------- */
/*                         TIMEZONE MIDNIGHT CROSSING                         */
/* -------------------------------------------------------------------------- */
#[test]
fn test_timezone_crossing_midnight() {
    // 11:00 PM in London (UTC) on Dec 25th
    let late_night_utc = datetime!(2025-12-25 23:00:00 UTC);

    // Convert to Tokyo (UTC+9). Should become Dec 26th, 08:00 AM.
    let result =
        crate::utils::others_utils::convert_to_local_primitive(late_night_utc, "Asia/Tokyo");

    assert!(result.is_ok());
    let tokyo_time = result.unwrap();

    // Assert the DATE changed
    assert_eq!(tokyo_time.date().day(), 26);
    // Assert the TIME is correct
    assert_eq!(tokyo_time.hour(), 8);
}

/* -------------------------------------------------------------------------- */
/*                           DATE MATH SANITY CHECK                           */
/* -------------------------------------------------------------------------- */
#[test]
fn test_duration_math_works() {
    // Booking logic relies heavily on "Start Time + Duration = End Time"
    let start_time = datetime!(2025-01-01 10:00:00);
    let duration = Duration::minutes(90); // 1 hour 30 mins

    let end_time = start_time + duration;

    // Should be 11:30
    assert_eq!(end_time.hour(), 11);
    assert_eq!(end_time.minute(), 30);
}

/* -------------------------------------------------------------------------- */
/*                     JSON SERIALIZATION (CONTRACT TEST)                     */
/* -------------------------------------------------------------------------- */
#[test]
fn test_timeslot_serializes_correctly() {
    // Booking systems live and die by Date Formats.
    // This test ensures we never accidentally break the API contract.

    let slot = TimeSlot {
        start_time: "2025-12-25T09:00:00+00:00".to_string(),
        end_time: "2025-12-25T10:00:00+00:00".to_string(),
    };

    let json_output = serde_json::to_value(&slot).expect("Failed to serialize");

    // Assert keys match exactly what Frontend expects
    assert_eq!(json_output["start_time"], "2025-12-25T09:00:00+00:00");
    assert_eq!(json_output["end_time"], "2025-12-25T10:00:00+00:00");
}

/* -------------------------------------------------------------------------- */
/*                    BOOKING ENGINE LOGIC (Overlap Check)                    */
/* -------------------------------------------------------------------------- */
#[test]
fn test_slot_generation_blocks_overlaps() {
    // Setup: 9:00 to 12:00 window
    let start = datetime!(2025-01-01 09:00:00);
    let end = datetime!(2025-01-01 12:00:00);

    // Block: 10:00 to 11:00 is BUSY
    let blocked = vec![(
        datetime!(2025-01-01 10:00:00 UTC),
        datetime!(2025-01-01 11:00:00 UTC),
    )];

    // Generate 60 min slots
    let slots = generate_slots(start, end, 60, &chrono_tz::Tz::UTC, &blocked);

    // Assert
    // Should have: 09:00-10:00 (OK), 10:00-11:00 (BLOCKED), 11:00-12:00 (OK)
    assert_eq!(slots.len(), 2);
    assert_eq!(slots[0].start_time, "2025-01-01T09:00:00Z");
    assert_eq!(slots[1].start_time, "2025-01-01T11:00:00Z");
}
