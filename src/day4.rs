use std::collections::HashMap;

struct Time {
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

enum Event {
    BeginShift(Time, u32),
    FallAsleep(Time),
    WakeUp(Time),
}

fn parse_line(line: &str) -> Event {
    let year = &line[1..5];
    let year: u32 = year.parse().unwrap();

    let month = &line[6..8];
    let month: u32 = month.parse().unwrap();

    let day = &line[9..11];
    let day: u32 = day.parse().unwrap();

    let hour = &line[12..14];
    let hour: u32 = hour.parse().unwrap();

    let minute = &line[15..17];
    let minute: u32 = minute.parse().unwrap();

    let time = Time { year, month, day, hour, minute };

    let indicator = &line[19..24];

    if indicator == "Guard" {
        let id_length = &line[26..].find(" ").unwrap();
        let id = &line[26..26+id_length];
        let id: u32 = id.parse().unwrap();
        Event::BeginShift(time, id)
    } else if indicator == "falls" {
        Event::FallAsleep(time)
    } else {
        Event::WakeUp(time)
    }
}

fn sort_events(events: &mut[Event]) {
    events.sort_by(|a, b| {
        let a_time = match a {
            Event::BeginShift(time, _) => time,
            Event::FallAsleep(time) => time,
            Event::WakeUp(time) => time,
        };

        let b_time = match b {
            Event::BeginShift(time, _) => time,
            Event::FallAsleep(time) => time,
            Event::WakeUp(time) => time,
        };

        a_time.year.cmp(&b_time.year)
            .then(a_time.month.cmp(&b_time.month))
            .then(a_time.day.cmp(&b_time.day))
            .then(a_time.hour.cmp(&b_time.hour))
            .then(a_time.minute.cmp(&b_time.minute))
    });
}

pub fn day4_1(lines: &[String]) -> u32 {
    let mut events = lines.iter().map(|line| parse_line(&line)).collect::<Vec<Event>>();

    sort_events(&mut events);

    // find which guard slept the most minutes
    let mut map = HashMap::new();
    let mut cur_id = 0;
    let mut sleep_start = 0;

    for event in events.iter() {
        if let Event::BeginShift(_, id) = event {
            cur_id = *id;
        }

        if let Event::FallAsleep(time) = event {
            sleep_start = time.minute;
        }

        if let Event::WakeUp(time) = event {
            let sleep_length = time.minute - sleep_start;
            let sleep_count = map.entry(cur_id).or_insert(0);
            *sleep_count += sleep_length;
        }
    }

    let select_id = *map.keys().max_by_key(|key| map.get(key).unwrap()).unwrap();

    // find what minute the selected guard was asleep the longest
    let mut sleep_minutes = vec![0; 60];

    for event in events.iter() {
        if let Event::BeginShift(_, id) = event {
            cur_id = *id;
        }

        if cur_id == select_id {
            if let Event::FallAsleep(time) = event {
                sleep_start = time.minute;
            }

            if let Event::WakeUp(time) = event {
                for i in sleep_start..time.minute {
                    sleep_minutes[i as usize] += 1;
                }
            }
        }
    }

    let select_minute = (0..60).max_by_key(|i| sleep_minutes[*i as usize]).unwrap();
    select_id * select_minute
}

pub fn day4_2(lines: &[String]) -> u32 {
    let mut events = lines.iter().map(|line| parse_line(&line)).collect::<Vec<Event>>();

    sort_events(&mut events);

    // find which guard slept the most minutes
    let mut map = HashMap::new();
    let mut cur_id = 0;
    let mut sleep_start = 0;
    
    let mut select_id = 0;
    let mut select_minute = 0;
    let mut max_count = 0;

    for event in events.iter() {
        if let Event::BeginShift(_, id) = event {
            cur_id = *id;
        }

        if let Event::FallAsleep(time) = event {
            sleep_start = time.minute;
        }

        if let Event::WakeUp(time) = event {
            let sleep_minutes = map.entry(cur_id).or_insert(vec![0; 60]);
            for i in sleep_start..time.minute {
                let i = i as usize;
                sleep_minutes[i] += 1;

                if sleep_minutes[i] > max_count {
                    select_id = cur_id;
                    select_minute = i as u32;
                    max_count = sleep_minutes[i];
                }
            }
        }
    }

    select_id * select_minute
}
