mod util;
use util::{get_file_path, get_lines};
extern crate chrono;
use chrono::offset::LocalResult;
use chrono::prelude::*;
extern crate regex;
use regex::Regex;

fn main() {
    let lines = get_lines(&get_file_path().unwrap());

    let part1 = part_1(&lines);
    //let part2 = part_2(&lines);

    println!("Part 1: {}", part1);
    // println!("Part 2: {}", part2);
}

enum GuardEvent {
    ShiftEvent(DateTime<Utc>, u32),
    SleepEvent(DateTime<Utc>),
    WakeEvent(DateTime<Utc>),
}

use std::fmt;
impl fmt::Debug for GuardEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GuardEvent::ShiftEvent(time, id) => write!(f, "[{}] Guard #{} begins shift", time, id),
            GuardEvent::SleepEvent(time) => write!(f, "[{}] falls asleep", time),
            GuardEvent::WakeEvent(time) => write!(f, "[{}] wakes up", time),
        }
    }
}

const shift_event_regex_string: &str =
    r"\[(\d{4})\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})\] Guard #(\d+) begins shift";
const sleep_event_regex_string: &str =
    r"\[(\d{4})\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})\] falls asleep";
const wake_event_regex_string: &str = r"\[(\d{4})\-(\d{2})\-(\d{2}) (\d{2}):(\d{2})\] wakes up";

fn shift_event(line: &String) -> Option<GuardEvent> {
    let shift_event_regex: Regex = Regex::new(shift_event_regex_string).unwrap();
    if shift_event_regex.is_match(&line) {
        let caps = shift_event_regex.captures(&line).unwrap();
        let (yyyy, m, d, hh, mm, id) = (
            caps.get(1)
                .map_or(0, |m| m.as_str().parse::<i32>().unwrap()),
            caps.get(2)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            caps.get(3)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            caps.get(4)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            caps.get(5)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            caps.get(6)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
        );

        let time = Utc.ymd(yyyy, m, d).and_hms(hh, mm, 0);
        return Some(GuardEvent::ShiftEvent(time, id));
    }
    None
}

fn sleep_event(line: &String) -> Option<GuardEvent> {
    let sleep_event_regex: Regex = Regex::new(sleep_event_regex_string).unwrap();
    if sleep_event_regex.is_match(&line) {
        let caps = sleep_event_regex.captures(&line).unwrap();
        let (yyyy, m, d, hh, mm) = (
            caps.get(1)
                .map_or(0, |m| m.as_str().parse::<i32>().unwrap()),
            caps.get(2)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            caps.get(3)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            caps.get(4)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            caps.get(5)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
        );

        let time = Utc.ymd(yyyy, m, d).and_hms(hh, mm, 0);
        return Some(GuardEvent::SleepEvent(time));
    }
    None
}

fn wake_event(line: &String) -> Option<GuardEvent> {
    let wake_event_regex: Regex = Regex::new(wake_event_regex_string).unwrap();
    if wake_event_regex.is_match(&line) {
        let caps = wake_event_regex.captures(&line).unwrap();
        let (yyyy, m, d, hh, mm) = (
            caps.get(1)
                .map_or(0, |m| m.as_str().parse::<i32>().unwrap()),
            caps.get(2)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            caps.get(3)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            caps.get(4)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
            caps.get(5)
                .map_or(0, |m| m.as_str().parse::<u32>().unwrap()),
        );

        let time = Utc.ymd(yyyy, m, d).and_hms(hh, mm, 0);
        return Some(GuardEvent::WakeEvent(time));
    }
    None
}

use std::cmp::Ordering;

fn sort(a: &GuardEvent, b: &GuardEvent) -> Ordering {
    let a_time = match a {
        GuardEvent::ShiftEvent(t, _) => t,
        GuardEvent::SleepEvent(t) => t,
        GuardEvent::WakeEvent(t) => t,
    };

    let b_time = match b {
        GuardEvent::ShiftEvent(t, _) => t,
        GuardEvent::SleepEvent(t) => t,
        GuardEvent::WakeEvent(t) => t,
    };

    let a_date_component = a_time.date();
    let a_time_component = a_time.time();

    let b_date_component = b_time.date();
    let b_time_component = b_time.time();

    match a_date_component.cmp(&b_date_component) {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => a_time_component.cmp(&b_time_component),
    }
}

fn parse_lines(lines: &Vec<String>) -> Vec<GuardEvent> {
    let mut result: Vec<GuardEvent> = lines
        .iter()
        .map(|l| match shift_event(&l) {
            Some(e) => e,
            None => match sleep_event(&l) {
                Some(e) => e,
                None => match wake_event(&l) {
                    Some(e) => e,
                    None => panic!("unknown - {}", &l),
                },
            },
        }).collect();

    result.sort_by(|a, b| sort(&a, &b));

    result
}

struct Shift {
    id: u32,
    date: DateTime<Utc>,
    time_asleep: i64,
}

impl fmt::Debug for Shift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}] Guard #{} {} mins total",
            self.date, self.id, self.time_asleep
        )
    }
}

impl Shift {
    fn new(id: u32, datetime: &DateTime<Utc>, total_time_asleep: i64) -> Shift {
        Shift {
            id: id,
            time_asleep: total_time_asleep,
            date: datetime.clone(),
        }
    }

    fn add_mins(&self, mins: i64) -> Shift {
        Shift::new(self.id, &self.date, self.time_asleep + mins)
    }
}

fn get_shifts(events: &Vec<GuardEvent>) -> Vec<Shift> {
    let mut shift: Option<Shift> = None;
    let mut shifts: Vec<Shift> = Vec::new();
    let mut wake_time: Option<chrono::naive::NaiveTime> = None;

    for e in events.iter() {
        match e {
            GuardEvent::ShiftEvent(time, id) => {
                match shift {
                    None => shift = Some(Shift::new(id.clone(), &time, 0)),
                    Some(s) => {
                        let diff: chrono::Duration = time.time() - (wake_time.unwrap());
                        shifts.push(s.add_mins(diff.num_minutes()));
                        shift = Some(Shift::new(id.clone(), &time, 0))
                    }
                };
                wake_time = Some(time.time());
            }
            GuardEvent::SleepEvent(time) => {
                let diff: chrono::Duration = time.time() - (wake_time.unwrap());
                match shift {
                    None => panic!("shouldn't get here"),
                    Some(s) => shift = Some(s.add_mins(diff.num_minutes())),
                }
                wake_time = None;
            }
            GuardEvent::WakeEvent(time) => {
                wake_time = Some(time.time());
            }
        };
    }

    shifts
}

fn part_1(lines: &Vec<String>) -> u32 {
    let events = parse_lines(&lines);
    let shifts = get_shifts(&events);
    println!("\n\nEvents:{:?}", events);
    println!("\n\nShifts{:?}", shifts);
    panic!("Not impemented");
}

fn part_2(lines: &Vec<String>) -> () {
    panic!("Not implemented");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_data() -> Vec<String> {
        vec![
            String::from("[1518-11-01 00:00] Guard #10 begins shift"),
            String::from("[1518-11-01 00:05] falls asleep"),
            String::from("[1518-11-01 00:25] wakes up"),
            String::from("[1518-11-01 00:30] falls asleep"),
            String::from("[1518-11-01 00:55] wakes up"),
            String::from("[1518-11-01 23:58] Guard #99 begins shift"),
            String::from("[1518-11-02 00:40] falls asleep"),
            String::from("[1518-11-02 00:50] wakes up"),
            String::from("[1518-11-03 00:05] Guard #10 begins shift"),
            String::from("[1518-11-03 00:24] falls asleep"),
            String::from("[1518-11-03 00:29] wakes up"),
            String::from("[1518-11-04 00:02] Guard #99 begins shift"),
            String::from("[1518-11-04 00:36] falls asleep"),
            String::from("[1518-11-04 00:46] wakes up"),
            String::from("[1518-11-05 00:03] Guard #99 begins shift"),
            String::from("[1518-11-05 00:45] falls asleep"),
            String::from("[1518-11-05 00:55] wakes up"),
        ]
    }

    #[test]
    fn part_1_assert_guard_10_sleeps_most_on_min_24() {
        let result = part_1(&get_test_data());
        assert_eq!(240, result)
    }
}
