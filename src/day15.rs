use advent_of_code_2022::read_file_to_string;
use sscanf::scanf;
use std::{collections::HashSet, str::FromStr};

const MAX_COORD: i64 = 4_000_000;
const MAX_HALF_COORD: i64 = MAX_COORD / 2;

type Coords = (i64, i64);

#[derive(Clone, Debug)]
struct Interval {
    start: i64,
    end: i64,
}

enum MergeResult {
    Disjointed,
    Merged(Interval),
}

impl Interval {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    fn len(&self) -> u64 {
        (self.end - self.start + 1) as u64
    }

    fn clamped(&self, min: i64, max: i64) -> Interval {
        Self::new(self.start.max(min), self.end.min(max))
    }

    fn merge(&self, rhs: &Interval) -> MergeResult {
        // case when `self` completely absorbs `rhs`
        if self.start <= rhs.start && rhs.end <= self.end {
            return MergeResult::Merged(self.clone());
        }

        // case when `rhs` completely absorbs `self`
        if rhs.start <= self.start && self.end <= rhs.end {
            return MergeResult::Merged(rhs.clone());
        }

        // case when `self` overlaps `rhs` from the left
        if self.start <= rhs.start && rhs.start <= self.end {
            return MergeResult::Merged(Self::new(self.start, rhs.end));
        }

        // case when `rhs` overlaps `self` from the left
        if rhs.start <= self.start && self.start <= rhs.end {
            return MergeResult::Merged(Self::new(rhs.start, self.end));
        }

        // case when the 2 intervals are disjointed
        MergeResult::Disjointed
    }
}

fn manhattan_distance(a: &Coords, b: &Coords) -> u64 {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs()) as u64
}

struct SensorWithClosestBeacon {
    sensor_pos: Coords,
    beacon_pos: Coords,
    distance: u64,
}

impl FromStr for SensorWithClosestBeacon {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s_x, s_y, b_x, b_y): (i64, i64, i64, i64) = scanf!(
            s,
            "Sensor at x={i64}, y={i64}: closest beacon is at x={i64}, y={i64}"
        )
        .map_err(|_| "Invalid format".to_string())?;

        let sensor_pos = (s_x, s_y);
        let beacon_pos = (b_x, b_y);
        let distance = manhattan_distance(&sensor_pos, &beacon_pos);

        Ok(Self {
            sensor_pos,
            beacon_pos,
            distance,
        })
    }
}

impl SensorWithClosestBeacon {
    fn get_covered_interval_in_in_row(&self, row: i64) -> Option<Interval> {
        let diff = (self.sensor_pos.1 - row).abs() as u64;
        if diff > self.distance {
            return None;
        }

        let half_len = (self.distance - diff) as i64;
        Some(Interval::new(
            self.sensor_pos.0 - half_len,
            self.sensor_pos.0 + half_len,
        ))
    }
}

struct TunnelSystem {
    sensors_with_closest_beacons: Vec<SensorWithClosestBeacon>,
    known_beacon_coordinates: HashSet<Coords>,
}

impl FromStr for TunnelSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sensors_with_closest_beacons = s
            .lines()
            .map(|l| SensorWithClosestBeacon::from_str(l))
            .collect::<Result<Vec<_>, _>>()?;

        let known_beacon_coordinates = sensors_with_closest_beacons
            .iter()
            .map(|s| s.beacon_pos)
            .collect();

        Ok(Self {
            sensors_with_closest_beacons,
            known_beacon_coordinates,
        })
    }
}

impl TunnelSystem {
    fn count_mapped_area_size_in_row(&self, row: i64) -> u64 {
        let intervals = self
            .sensors_with_closest_beacons
            .iter()
            .filter_map(|s| s.get_covered_interval_in_in_row(row))
            .collect::<Vec<_>>();
        let merged_intervals = Self::merge_intervals(&intervals);

        let number_of_covered_coordinates = merged_intervals.iter().map(|i| i.len()).sum::<u64>();
        let coordinates_where_a_beacon_cannot_be =
            number_of_covered_coordinates - self.count_num_of_beacons_in_row(row);

        coordinates_where_a_beacon_cannot_be
    }

    fn get_merged_intervals_in_row_clamped(&self, row: i64) -> Vec<Interval> {
        let intervals = self
            .sensors_with_closest_beacons
            .iter()
            .filter_map(|s| s.get_covered_interval_in_in_row(row))
            .collect::<Vec<_>>();
        let merged_intervals = Self::merge_intervals(&intervals);
        let clamped_intervals = merged_intervals
            .iter()
            .map(|i| i.clamped(0, MAX_COORD))
            .collect();
        clamped_intervals
    }

    fn merge_intervals(intervals: &Vec<Interval>) -> Vec<Interval> {
        let mut intervals = intervals.clone();

        let mut changed = true;
        'outer: while changed {
            changed = false;
            for i in 0..intervals.len() - 1 {
                for j in (i + 1)..intervals.len() {
                    let lhs = &intervals[i];
                    let rhs = &intervals[j];

                    match lhs.merge(rhs) {
                        MergeResult::Disjointed => continue,
                        MergeResult::Merged(merged) => {
                            intervals.remove(j);
                            intervals.remove(i);
                            intervals.push(merged);
                            changed = true;
                            continue 'outer;
                        }
                    }
                }
            }
        }

        intervals
    }

    fn count_num_of_beacons_in_row(&self, row: i64) -> u64 {
        self.known_beacon_coordinates
            .iter()
            .filter(|b| b.1 == row)
            .count() as u64
    }
}

fn calculate_tuning_frequency(position: &Coords) -> u64 {
    (position.0 * MAX_COORD + position.1) as u64
}

fn count_number_of_positions_which_cannot_contain_a_beacon_in_row_2_000_000(
    tunnel: &TunnelSystem,
) -> u64 {
    tunnel.count_mapped_area_size_in_row(MAX_HALF_COORD)
}

fn determine_tuning_frequency_of_distress_beacon(tunnel: &TunnelSystem) -> Option<u64> {
    for y in 0..=MAX_COORD {
        let intervals = tunnel.get_merged_intervals_in_row_clamped(y);
        if intervals.len() == 2 {
            let i1 = &intervals[0];
            let i2 = &intervals[1];
            let x = if i1.start - i2.end == 2 {
                i1.start - 1
            } else if i2.start - i1.end == 2 {
                i2.start - 1
            } else {
                return None;
            };
            return Some(calculate_tuning_frequency(&(x, y)));
        }
    }
    None
}

fn main() {
    let input = read_file_to_string("input/day15.txt");
    let tunnel = TunnelSystem::from_str(&input).unwrap();
    let number_of_positions_which_cannot_contain_a_beacon_in_row_2_000_000 =
        count_number_of_positions_which_cannot_contain_a_beacon_in_row_2_000_000(&tunnel);
    println!(
        "The number of positions that cannot contain a beacon in row 2_000_000 is {number_of_positions_which_cannot_contain_a_beacon_in_row_2_000_000}"
    );

    determine_tuning_frequency_of_distress_beacon(&tunnel);

    let tuning_frequency_of_distress_beacon =
        determine_tuning_frequency_of_distress_beacon(&tunnel).unwrap();
    println!(
        "The tuning frequency of the distress beacon is {tuning_frequency_of_distress_beacon}"
    );
}
