const BIG_ENOUGH_NUMBER: i64 = i64::MAX/2;

#[derive(Clone)]
struct MapRange {
    min: i64,
    max: i64,
    diff: i64,
}

pub struct Map {
    ranges: Vec<MapRange>
}

impl Map {
    pub fn clone(&self) -> Map {
        Map{ranges: self.ranges.clone()}
    }
}

impl Map {
    pub fn new() -> Map {
        Map{ ranges: vec![] }
    }
    pub fn insert(&mut self, dest: i64, src: i64, size: i64) {
        self.ranges.push(MapRange{
            min: src,
            max: src + size - 1,
            diff: dest - src,
        });
    }

    pub fn map(&self, val: i64) -> i64 {
        for range in self.ranges.iter() {
            if val >= range.min && val <= range.max {
                return val + range.diff;
            }
        }

        val
    }

    fn cut_range(&mut self, target_range: &MapRange) {
        let mut i = 0;
        while i < self.ranges.len() {
            if self.ranges[i].min <= target_range.max && target_range.min <= self.ranges[i].max {
                if target_range.max < self.ranges[i].max && target_range.min > self.ranges[i].min {
                    self.ranges.push(
                        MapRange{
                            min: target_range.max + 1, max: self.ranges[i].max, diff: self.ranges[i].diff
                        });
                    self.ranges[i].max = target_range.min - 1;
                } else if target_range.max < self.ranges[i].max {
                    self.ranges[i].min = target_range.max + 1;
                } else if target_range.min > self.ranges[i].min {
                    self.ranges[i].max = target_range.min - 1;
                } else if target_range.min == self.ranges[i].min && target_range.max == self.ranges[i].max {
                    self.ranges.remove(i);
                }
            }
            i += 1;
        }
    }

    fn fill_gaps(&mut self) {
        let mut identity_ranges = Map{ranges: vec![MapRange{diff:0, min: 0, max: BIG_ENOUGH_NUMBER}]};
        for range in &self.ranges {
            identity_ranges.cut_range(range);
        }
        self.ranges.append(&mut identity_ranges.ranges);
    }

    pub fn print(&self) -> String {
        let mut output = String::from("");
        for range in &self.ranges {
            output += &format!("\n{} {} {}", range.min + range.diff, range.min, range.max - range.min + 1);
        }

        output
    }
}

impl Map {
    pub fn input_to_maps(input: &str) -> Vec<Map> {
        let mut maps = vec![];
        let mut i = 1; // skip the seeds line
        while i < input.lines().count() {
            i += 2; // skip blank and title line
            let mut map = Map::new();
            while i < input.lines().count() && !input.lines().nth(i).unwrap().is_empty() {
                let values: Vec<i64> = input.lines().nth(i).unwrap().split(" ")
                    .map(|x| x.parse().unwrap())
                    .collect();

                map.insert(
                    values[0],
                    values[1],
                    values[2]
                );

                i += 1;
            }
            map.fill_gaps();
            maps.push(map);
        }

        maps
    }

    pub fn critical_intersections(a: &Map, b: &Map) -> Vec<i64> {
        let mut intersections = vec![];
        for range_a in &a.ranges {
            for range_b in &b.ranges {
                if range_a.min <= range_b.max
                    && range_b.min <= range_a.max {
                    intersections.push(
                        if range_a.min < range_b.min {range_b.min} else {range_a.min}
                    );
                    intersections.push(
                        if range_a.max > range_b.max {range_b.max} else {range_a.max}
                    );
                }
            }
        }

        intersections
    }

    pub fn add(a: &Map, b: &Map) -> Map {
        let mut sum = Map::new();
        for range_a in &a.ranges {
            let range_a = &MapRange{
                min: range_a.min + range_a.diff,
                max: range_a.max + range_a.diff,
                diff: range_a.diff,
            };
            for range_b in &b.ranges {
                if range_b.min <= range_a.max
                    && range_a.min <= range_b.max {
                    let overlap_range = MapRange {
                        min: if range_b.min > range_a.min { range_b.min } else { range_a.min } - range_a.diff,
                        max: if range_b.max < range_a.max { range_b.max } else { range_a.max } - range_a.diff,
                        diff: range_b.diff + range_a.diff
                    };
                    sum.ranges.push(overlap_range);
                }
            }
        }
        sum.fill_gaps();

        sum
    }
}