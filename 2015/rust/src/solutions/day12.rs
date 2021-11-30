use serde_json::Value;

pub fn parse(input: &str) -> Value {
    serde_json::from_str(input).unwrap()
}

pub fn part1(input: &Value) -> isize {
    part(input, false)
}

pub fn part2(input: &Value) -> isize {
    part(input, true)
}

pub fn part(input: &Value, red: bool) -> isize {
    if let Some(v) = input.as_object() {
        let mut val = 0;
        if red && v.contains_key("red") {
            return val;
        }
        for i in v.values() {
            if red && i == "red" {
                return 0;
            }
            val += part1(i);
        }
        return val;
    }
    if let Some(v) = input.as_array() {
        let mut val = 0;
        for i in v {
            val += part1(i);
        }
        return val;
    }
    if let Some(v) = input.as_i64() {
        return v as isize;
    }
    0
}
