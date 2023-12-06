use crate::read_input::read_input;
use anyhow::Result;
use std::ops::Range;

#[derive(Eq, Debug)]
struct Ranges {
    source: Range<usize>,
    destination: Range<usize>,
}

impl Ranges {
    fn intersect(&self, other: &Range<usize>) -> Option<Range<usize>> {
        let start = other.start.max(self.source.start);
        let end = other.end.min(self.source.end);
        if start >= end {
            return None;
        }
        if self.destination.start >= self.source.start {
            let diff = self.destination.start - self.source.start;
            Some(Range {
                start: start + diff,
                end: end + diff,
            })
        } else {
            let diff = self.source.start - self.destination.start;
            Some(Range {
                start: start - diff,
                end: end - diff,
            })
        }
    }
}

impl From<&str> for Ranges {
    fn from(value: &str) -> Self {
        let mut parts = value.splitn(3, ' ').map(|p| p.parse().unwrap());
        let destination_start = parts.next().unwrap();
        let source_start = parts.next().unwrap();
        let length = parts.next().unwrap();
        Self {
            destination: Range {
                start: destination_start,
                end: destination_start + length,
            },
            source: Range {
                start: source_start,
                end: source_start + length,
            },
        }
    }
}

impl Ord for Ranges {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.source.start.cmp(&other.source.start)
    }
}

impl PartialOrd for Ranges {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.source.start.partial_cmp(&other.source.start)
    }
}

impl PartialEq for Ranges {
    fn eq(&self, other: &Self) -> bool {
        self.source.start.eq(&other.source.start)
    }
}

#[derive(Debug)]
struct Section(Vec<Ranges>);

impl Section {
    fn find_ranges(&self, target: &Range<usize>) -> Vec<Range<usize>> {
        let mut ranges = Vec::<Range<usize>>::new();
        let mut prev_range = Range {
            start: target.start,
            end: target.start,
        };
        for range in &self.0 {
            let source = &range.source;
            if prev_range.start < source.start
                && source.start < target.end
                && source.start >= target.start
            {
                ranges.push(Range {
                    start: prev_range.end,
                    end: source.start,
                });
            }
            if let Some(intersect) = range.intersect(target) {
                ranges.push(intersect);
            }
            prev_range = range.source.clone();
        }
        if prev_range.end < target.end && prev_range.start > target.start {
            ranges.push(Range {
                start: prev_range.end,
                end: target.end,
            });
        }
        if ranges.is_empty() {
            ranges.push(target.clone());
        }
        ranges
    }
}

impl<I: Iterator<Item = String>> From<&mut I> for Section {
    fn from(value: &mut I) -> Self {
        value.next();
        let mut ranges = Vec::<Ranges>::new();
        while let Some(line) = value.next() {
            if line.trim().is_empty() {
                break;
            }
            ranges.push(line.as_str().into())
        }
        ranges.sort();
        Self(ranges)
    }
}

struct Seeds(Vec<Range<usize>>);

impl Seeds {
    fn min_location<I: Iterator<Item = String>>(&self, mut lines: I) -> usize {
        let seeds_to_soil = Section::from(&mut lines);
        let soil_to_fertilizer = Section::from(&mut lines);
        let fertilizer_to_water = Section::from(&mut lines);
        let water_to_light = Section::from(&mut lines);
        let ligth_to_temperature = Section::from(&mut lines);
        let temperature_to_humidity = Section::from(&mut lines);
        let humidity_to_location = Section::from(&mut lines);

        self.0
            .iter()
            .map(|seeds| {
                let mut ranges = seeds_to_soil
                    .find_ranges(seeds)
                    .into_iter()
                    .flat_map(|soil| soil_to_fertilizer.find_ranges(&soil))
                    .flat_map(|fertilizer| fertilizer_to_water.find_ranges(&fertilizer))
                    .flat_map(|water| water_to_light.find_ranges(&water))
                    .flat_map(|light| ligth_to_temperature.find_ranges(&light))
                    .flat_map(|temperature| temperature_to_humidity.find_ranges(&temperature))
                    .flat_map(|humidity| humidity_to_location.find_ranges(&humidity))
                    .collect::<Vec<_>>();
                ranges.sort_by(|l, r| l.start.cmp(&r.start));
                ranges.first().unwrap().start
            })
            .min()
            .unwrap()
    }
}

pub fn first_part() -> Result<()> {
    let input = read_input("day5")?;
    let mut lines = input.map(|l| l.unwrap());
    let seeds_line = lines.next().unwrap();
    let seeds = seeds_line.split_once(": ").unwrap().1;
    let seeds = seeds
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .map(|s| Range {
            start: s,
            end: s + 1,
        })
        .collect::<Vec<_>>();

    let seeds = Seeds(seeds);
    lines.next();
    let min_location = seeds.min_location(lines);

    println!("Min location: {min_location}");
    Ok(())
}

pub fn second_part() -> Result<()> {
    let input = read_input("day5")?;
    let mut lines = input.map(|l| l.unwrap());
    let seeds_line = lines.next().unwrap();
    let mut seeds_nums = seeds_line
        .split_once(": ")
        .unwrap()
        .1
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());

    let mut seeds = Vec::new();
    while let Some(start) = seeds_nums.next() {
        let length = seeds_nums.next().unwrap();
        seeds.push(Range {
            start,
            end: start + length,
        });
    }
    let seeds = Seeds(seeds);

    lines.next();
    let min_location = seeds.min_location(lines);

    println!("Min location: {min_location}");
    Ok(())
}
