#[macro_use]
extern crate nom;
extern crate chrono;

use chrono::naive::NaiveDate;
use std::collections::HashMap;
use std::str::FromStr;

pub fn most_asleep_guard(input: &str) -> Option<u16> {
    let nights = parse_nights(input);
    let mut sleeping_guards = HashMap::new();
    for night in nights.iter() {
        *sleeping_guards.entry(night.guard_id).or_insert(0) += u16::from(night.time_asleep());
    }
    sleeping_guards
        .iter()
        .max_by_key(|(_id, &time)| time)
        .map(|(&id, _time)| id)
}

pub fn most_asleep_minute(input: &str, guard_id: u16) -> Option<u8> {
    let nights = parse_nights(input);
    let nights_on_duty = nights.iter().filter(|night| night.guard_id == guard_id);
    let mut minutes = HashMap::new();
    for night in nights_on_duty {
        for (minute, &asleep) in night.sleeps.iter().enumerate() {
            if asleep {
                *minutes.entry(minute as u8).or_insert(0) += 1;
            }
        }
    }
    minutes
        .iter()
        .max_by_key(|(_minute, &time)| time)
        .map(|(&minute, _time)| minute)
}

pub fn most_consistently_asleep_guard(input: &str) -> Option<u16> {
    let nights = parse_nights(input);
    let mut guard_minutes = HashMap::new();
    for night in nights.iter() {
        for (minute, &asleep) in night.sleeps.iter().enumerate() {
            if asleep {
                *guard_minutes
                    .entry((night.guard_id, minute as u8))
                    .or_insert(0) += 1;
            }
        }
    }
    guard_minutes
        .iter()
        .max_by_key(|(_guard_minute, &time)| time)
        .map(|(guard_minute, _time)| guard_minute.0)
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Event {
    Begin { guard_id: u16 },
    FallAsleep,
    WakeUp,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Record {
    date: NaiveDate,
    hour: u8,
    minute: u8,
    event: Event,
}

impl Record {
    fn night_of(&self) -> NaiveDate {
        if self.hour == 0 {
            self.date
        } else {
            self.date.succ()
        }
    }
}

#[derive(Debug)]
struct Night {
    date: NaiveDate,
    guard_id: u16,
    sleeps: Vec<bool>,
}

impl Night {
    fn time_asleep(&self) -> u8 {
        self.sleeps.iter().filter(|&&m| m).count() as u8
    }
}

fn parse_nights(input: &str) -> Vec<Night> {
    let mut records = parse_records(input);
    records.sort_unstable();

    let mut nights = Vec::new();
    let mut some_night = None;
    let mut fall_asleep = 0;
    for record in records.drain(..) {
        match record.event {
            Event::Begin { guard_id } => {
                if let Some(night) = some_night {
                    nights.push(night);
                }
                some_night = Some(Night {
                    date: record.night_of(),
                    guard_id,
                    sleeps: (0..60).map(|_minute| false).collect(),
                });
            }
            Event::FallAsleep => {
                fall_asleep = record.minute;
            }
            Event::WakeUp => {
                if let Some(ref mut night) = some_night {
                    for minute in fall_asleep..record.minute {
                        night.sleeps[minute as usize] = true;
                    }
                }
            }
        }
    }
    if let Some(night) = some_night {
        nights.push(night);
    }
    nights
}

fn parse_records(mut input: &str) -> Vec<Record> {
    let mut records = Vec::new();
    loop {
        match parse_record(input.trim_start()) {
            Ok((remaining, record)) => {
                input = remaining;
                records.push(record);
            }
            Err(_) => return records,
        }
    }
}

named!(parse_record<&str, Record>,
    do_parse!(
        tag_s!("[1518-") >>
        month: map_res!(nom::digit, FromStr::from_str) >>
        tag_s!("-") >>
        day: map_res!(nom::digit, FromStr::from_str) >>
        tag_s!(" ") >>
        hour: map_res!(nom::digit, FromStr::from_str) >>
        tag_s!(":") >>
        minute: map_res!(nom::digit, FromStr::from_str) >>
        tag_s!("] ") >>
        event: alt!(
            do_parse!(
                tag_s!("Guard #") >>
                guard_id: map_res!(nom::digit, FromStr::from_str) >>
                tag_s!(" begins shift") >>
                (Event::Begin { guard_id})
            ) |
            do_parse!(
                tag_s!("falls asleep") >>
                (Event::FallAsleep)
            ) |
            do_parse!(
                tag_s!("wakes up") >>
                (Event::WakeUp)
            )
        ) >>
        (Record { date: NaiveDate::from_ymd(1518, month, day), hour, minute, event })
    )
);

#[cfg(test)]
mod tests {
    #[test]
    fn most_asleep_guard_test() {
        use most_asleep_guard;
        assert_eq!(most_asleep_guard(include_str!("../example.txt")), Some(10));
    }

    #[test]
    fn most_asleep_minute_test() {
        use most_asleep_minute;
        assert_eq!(
            most_asleep_minute(include_str!("../example.txt"), 10),
            Some(24)
        );
    }

    #[test]
    fn most_consistently_asleep_guard_test() {
        use most_consistently_asleep_guard;
        assert_eq!(
            most_consistently_asleep_guard(include_str!("../example.txt")),
            Some(99)
        );
    }

    #[test]
    fn puzzle_one() {
        static GUARD_ID: u16 = 3209;
        static ASLEEP_MINUTE: u8 = 32;

        use most_asleep_guard;
        assert_eq!(
            most_asleep_guard(include_str!("../input.txt")),
            Some(GUARD_ID)
        );

        use most_asleep_minute;
        assert_eq!(
            most_asleep_minute(include_str!("../input.txt"), GUARD_ID),
            Some(ASLEEP_MINUTE)
        );

        assert_eq!(GUARD_ID as u32 * ASLEEP_MINUTE as u32, 102688);
    }

    #[test]
    fn puzzle_two() {
        static GUARD_ID: u16 = 1459;
        static ASLEEP_MINUTE: u8 = 39;

        use most_consistently_asleep_guard;
        assert_eq!(
            most_consistently_asleep_guard(include_str!("../input.txt")),
            Some(GUARD_ID)
        );

        use most_asleep_minute;
        assert_eq!(
            most_asleep_minute(include_str!("../input.txt"), GUARD_ID),
            Some(ASLEEP_MINUTE)
        );

        assert_eq!(GUARD_ID as u32 * ASLEEP_MINUTE as u32, 56901);
    }

}
