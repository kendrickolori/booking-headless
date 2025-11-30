use chrono::{Datelike, TimeZone, Timelike};
use chrono_tz::Tz;
use std::str::FromStr;
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time};

pub fn convert_to_local_primitive(
    dt_utc: OffsetDateTime,
    tz_name: &str,
) -> Result<PrimitiveDateTime, String> {
    let timestamp_secs = dt_utc.unix_timestamp();
    let dt_chrono_utc = chrono::Utc
        .timestamp_opt(timestamp_secs, 0)
        .single()
        .ok_or_else(|| "Invalid time conversion.".to_string())?;

    let tz: Tz = Tz::from_str(tz_name).map_err(|e| format!("Invalid Timezone: {}", e))?;
    let dt_local_chrono = dt_chrono_utc.with_timezone(&tz);
    let local_naive_datetime = dt_local_chrono.naive_local();
    let local_date = local_naive_datetime.date();
    let local_time = local_naive_datetime.time();

    let time_date = Date::from_calendar_date(
        local_date.year_ce().1 as i32,
        Month::try_from(local_date.month() as u8).map_err(|e| e.to_string())?,
        local_date.day() as u8,
    )
    .map_err(|e| format!("Invalid local date components: {}", e))?;

    let time_time = Time::from_hms_nano(
        local_time.hour() as u8,
        local_time.minute() as u8,
        local_time.second() as u8,
        local_time.nanosecond(),
    )
    .map_err(|e| format!("Invalid local time components: {}", e))?;

    Ok(PrimitiveDateTime::new(time_date, time_time))
}

pub fn local_to_utc(local_date: PrimitiveDateTime, tz: &Tz) -> Option<time::OffsetDateTime> {
    // Convert time::PrimitiveDateTime -> chrono::NaiveDateTime
    let chrono_naive = chrono::NaiveDate::from_ymd_opt(
        local_date.year(),
        local_date.month() as u32,
        local_date.day() as u32,
    )?
    .and_hms_opt(
        local_date.hour() as u32,
        local_date.minute() as u32,
        local_date.second() as u32,
    )?;

    // Attach Timezone
    let chrono_tz_aware = tz.from_local_datetime(&chrono_naive).single()?;

    // Convert to UTC
    let chrono_utc = chrono_tz_aware.with_timezone(&chrono::Utc);

    // Convert back to time::OffsetDateTime
    time::OffsetDateTime::from_unix_timestamp(chrono_utc.timestamp()).ok()
}
