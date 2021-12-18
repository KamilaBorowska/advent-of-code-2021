use crate::Solution;
use std::cell::Cell;
use std::collections::HashMap;
use std::error::Error;

#[derive(PartialEq, Eq, Clone, Copy)]
enum VisitState {
    Big,
    NotVisited,
    Visited,
}

struct Room {
    destinations: Vec<usize>,
    visit_state: Cell<VisitState>,
}

struct Rooms<'a> {
    name_map: HashMap<&'a str, usize>,
    rooms: Vec<Room>,
    end_room: usize,
    doubled: Cell<bool>,
}

impl<'a> Rooms<'a> {
    fn from_input(input: &'a str, can_double: bool) -> Result<Self, Box<dyn Error>> {
        let mut rooms = Rooms {
            name_map: HashMap::new(),
            rooms: Vec::new(),
            end_room: usize::MAX,
            doubled: Cell::new(!can_double),
        };
        for path in input.lines() {
            rooms.insert(path)?;
        }
        rooms.end_room = *rooms.name_map.get("end").ok_or("Missing end")?;
        Ok(rooms)
    }

    fn create_room(&mut self, room: &'a str) -> usize {
        let index = *self.name_map.entry(room).or_insert_with(|| {
            let index = self.rooms.len();
            self.rooms.push(Room {
                destinations: Vec::new(),
                visit_state: Cell::new(
                    if room.chars().next().filter(|c| c.is_lowercase()).is_some() {
                        VisitState::NotVisited
                    } else {
                        VisitState::Big
                    },
                ),
            });
            index
        });
        index
    }

    fn insert(&mut self, path: &'a str) -> Result<(), Box<dyn Error>> {
        let (from_room, to_room) = path.split_once('-').ok_or("Invalid path")?;
        let from = self.create_room(from_room);
        let to = self.create_room(to_room);
        if to_room != "start" {
            self.rooms[from].destinations.push(to);
        }
        if from_room != "start" {
            self.rooms[to].destinations.push(from);
        }
        Ok(())
    }

    fn search_paths(&self, current_room: usize) -> usize {
        let room = &self.rooms[current_room];
        let visit_state = room.visit_state.get();
        let doubled = self.doubled.get();
        match visit_state {
            VisitState::Big => {}
            VisitState::NotVisited => room.visit_state.set(VisitState::Visited),
            VisitState::Visited => {
                if doubled {
                    return 0;
                }
                self.doubled.set(true);
            }
        }
        let mut sum = 0;
        for &destination in &room.destinations {
            if destination == self.end_room {
                sum += 1;
            } else {
                sum += self.search_paths(destination);
            }
        }
        room.visit_state.set(visit_state);
        self.doubled.set(doubled);
        sum
    }
}

fn run(input: &str, can_double: bool) -> Result<String, Box<dyn Error>> {
    let rooms = Rooms::from_input(input, can_double)?;
    let paths = rooms.search_paths(*rooms.name_map.get("start").ok_or("Expected start room")?);
    Ok(paths.to_string())
}

pub(super) const DAY12: Solution = Solution {
    part1: |input| run(input, false),
    part2: |input| run(input, true),
};

#[cfg(test)]
mod test {
    use crate::test;
    test!(
        DAY12.part1,
        example1: lines!(
            "start-A"
            "start-b"
            "A-c"
            "A-b"
            "b-d"
            "A-end"
            "b-end"
        ) => 10,
        example2: lines!(
            "dc-end"
            "HN-start"
            "start-kj"
            "dc-start"
            "dc-HN"
            "LN-dc"
            "HN-end"
            "kj-sa"
            "kj-HN"
            "kj-dc"
        ) => 19,
        example3: lines!(
            "fs-end"
            "he-DX"
            "fs-he"
            "start-DX"
            "pj-DX"
            "end-zg"
            "zg-sl"
            "zg-pj"
            "pj-he"
            "RW-he"
            "fs-DX"
            "pj-RW"
            "zg-RW"
            "start-pj"
            "he-WI"
            "zg-he"
            "pj-fs"
            "start-RW"
        ) => 226,
        input: 4707,
    );
    test!(
        DAY12.part2,
        example1: lines!(
            "start-A"
            "start-b"
            "A-c"
            "A-b"
            "b-d"
            "A-end"
            "b-end"
        ) => 36,
        example2: lines!(
            "dc-end"
            "HN-start"
            "start-kj"
            "dc-start"
            "dc-HN"
            "LN-dc"
            "HN-end"
            "kj-sa"
            "kj-HN"
            "kj-dc"
        ) => 103,
        example3: lines!(
            "fs-end"
            "he-DX"
            "fs-he"
            "start-DX"
            "pj-DX"
            "end-zg"
            "zg-sl"
            "zg-pj"
            "pj-he"
            "RW-he"
            "fs-DX"
            "pj-RW"
            "zg-RW"
            "start-pj"
            "he-WI"
            "zg-he"
            "pj-fs"
            "start-RW"
        ) => 3509,
        input: 130493,
    );
}
