use crate::Event::{Click, Error, Info};
use std::fmt::Formatter;
use std::io::stdin;

enum Event {
    Info(String),
    Error(i16),
    Click { x: i32, y: i32 },
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Info(message) => {
                write!(f, "Info: {}", message)
            }
            Error(code) => {
                write!(f, "Error: {}", code)
            }
            Click { x, y } => {
                write!(f, "Click: ({}, {})", x, y)
            }
        }
    }
}

impl Event {
    pub fn parse(line: &str) -> Option<Event> {
        let mut s = line.split(" ");
        let event_type = s.next().unwrap();
        if (event_type == "info") {
            Some(Info(s.next().unwrap().trim().to_string()))
        } else if (event_type == "error") {
            Some(Error(s.next().unwrap().trim().parse::<i16>().unwrap()))
        } else if (event_type == "click") {
            Some(Click {
                x: s.next().unwrap().trim().parse::<i32>().unwrap(),
                y: s.next().unwrap().trim().parse::<i32>().unwrap(),
            })
        } else {
            None
        }
    }
}

pub fn main() {
    let mut line = "".to_string();
    let mut events = Vec::new();
    loop {
        println!("Enter event (or 'exit' to quit):");
        line.clear();
        stdin().read_line(&mut line).unwrap();
        if line.trim().eq("exit") {
            statistic(&mut events);
            break;
        } else {
            let event = Event::parse(&line);
            if let Some(event) = event {
                events.push(event);
            }
        }
    }
}

pub fn statistic(events: &mut Vec<Event>) {
    /*let info_count = events
        .iter()
        .filter(|event| matches!(event, Event::Info(_)))
        .count();
    let error_count = events
        .iter()
        .filter(|event| matches!(event, Event::Error(_)))
        .count();
    let click_count = events
        .iter()
        .filter(|event| matches!(event, Event::Click { .. }))
        .count();*/

    let (info_count, error_count, click_count) =
        events
            .iter()
            .fold((0, 0, 0), |(info, err, click), event| match event {
                Info(_) => (info + 1, err, click),
                Error(_) => (info, err + 1, click),
                Click { .. } => (info, err, click + 1),
            });

    println!("Total events:{} ", events.len());
    for i in 0..events.len() {
        let event = events.get(i);
        println!("{:<6}{}", i, event.unwrap());
    }
    println!(
        "Event counts:[{},{},{}]",
        info_count, error_count, click_count
    );
}
