/**
Struct for parsing and containing a pair of u8 passed as a string like "0/100".
Normally correlates to a (current_value, target/max_value)
*/
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Pair {
    pub current: u16,
    pub target: u16,
}
#[derive(Debug, PartialEq, Eq)]
pub struct PairParseError;

impl std::str::FromStr for Pair {
    type Err = PairParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let slash_split: Vec<&str> = s.split("/").collect();
        if slash_split.len() == 2 {
            return Ok(Pair {
                current: slash_split[0].parse::<u16>().unwrap_or(0),
                target: slash_split[1].parse::<u16>().unwrap_or(0),
            });
        }
        Ok(Pair::default())
    }
}

/**
Struct for parsing and containing a triple of (u64, u64, bool) passed as a string like "0/100/1".
This will normally correlate to (current_file_position, max_file_position, paused)
*/
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Triple {
    pub current_file_position: u64,
    pub max_file_position: u64,
    pub paused: bool,
}
#[derive(Debug, PartialEq, Eq)]
pub struct TripleParseError;

impl std::str::FromStr for Triple {
    type Err = TripleParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let slash_split: Vec<&str> = s.split("/").collect();
        if slash_split.len() == 3 {
            return Ok(Triple {
                current_file_position: slash_split[0].parse::<u64>().unwrap_or(0),
                max_file_position: slash_split[1].parse::<u64>().unwrap_or(0),
                paused: match slash_split[2] {
                    "0" => false,
                    "1" => true,
                    _ => false,
                },
            });
        }
        Ok(Triple::default())
    }
}

/// Struct to normalize the data coming from a printer when a "M4000" gcode is sent.
/// If a particular key is not in the data a default will be used.
/// If the data can not be parsed for any reason the default will be returned.
///
/// * (b, e1, e2) are all temps and not currently used.
/// * (x, y, z) are the current position of the printer, only z should be a real value.
/// * (f) is the fan speed.
/// * (d) contains the current file position, the max file size, and a (0 | 1) on if the printer is paused.
/// * (t) is the time elapsed since the print was started.
///
/// # Examples
///
/// ```
/// let state = PrinterState::from_str("ok B:0/0 X:0.000 Y:0.000 Z:50.000 F:256/256 D:0/0/1");
/// let compared = PrinterState {
///     b: Pair{current: 0, target: 0},
///     e1: Pair{current: 0, target: 0},
///     e2: Pair{current: 0, target: 0},
///     x: 0.000,
///     y: 0.000,
///     z: 50.000,
///     f: Pair{current: 256, target: 256},
///     d: Triple{current_file_position: 0, max_file_position: 0, paused: true},
///     ..Default::default()
/// };
/// assert_eq!(state.unwrap(), compared);
/// ```
#[derive(Debug, Default, PartialEq)]
pub struct PrinterState {
    pub b: Pair,
    pub e1: Pair,
    pub e2: Pair,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub f: Pair,
    pub d: Triple,
    pub t: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PrinterStateParseError;
impl std::str::FromStr for PrinterState {
    type Err = PrinterStateParseError;
    fn from_str(s: &str) -> Result<Self, PrinterStateParseError> {
        let mut state = Self::default();
        for p in s.split_whitespace() {
            let colon_split: Vec<&str> = p.split(":").collect();
            if colon_split.len() != 2 || colon_split[1].is_empty() {
                continue;
            }
            match colon_split[0] {
                "B" => state.b = colon_split[1].parse().unwrap(),
                "E1" => state.e1 = colon_split[1].parse().unwrap(),
                "E2" => state.e2 = colon_split[1].parse().unwrap(),
                "X" => state.x = colon_split[1].parse().unwrap(),
                "Y" => state.y = colon_split[1].parse().unwrap(),
                "Z" => state.z = colon_split[1].parse().unwrap(),
                "F" => state.f = colon_split[1].parse().unwrap(),
                "D" => state.d = colon_split[1].parse().unwrap(),
                "T" => state.t = colon_split[1].parse().unwrap(),
                _ => {}
            }
        }
        Ok(state)
    }
}

#[test]
fn test_parse_valid_state() {
    let status_string = "B:1/2 E1:3/4 X:1.5 Y:2.0 Z:3.14 D:50/100/0 T:10";
    let expected_state = PrinterState {
        b: Pair {
            current: 1,
            target: 2,
        },
        e1: Pair {
            current: 3,
            target: 4,
        },
        x: 1.5,
        y: 2.0,
        z: 3.14,
        d: Triple {
            current_file_position: 50,
            max_file_position: 100,
            paused: false,
        },
        t: 10,
        ..Default::default()
    };
    let actual_state: PrinterState = status_string.parse().unwrap();
    assert_eq!(actual_state, expected_state);
}

#[test]
fn test_parse_empty_string() {
    let status_string = "";
    let expected_state = PrinterState::default();
    let actual_state: PrinterState = status_string.parse().unwrap();
    assert_eq!(actual_state, expected_state);
}

#[test]
fn test_parse_missing_colon() {
    let status_string = "B 1/2 E1 3/4";
    let actual_result: PrinterState = status_string.parse().unwrap();
    assert_eq!(actual_result, PrinterState::default());
}

#[test]
fn test_parse_invalid_key() {
    let status_string = "X: 1.5 Y: 2.0 Z: 3.14 U: invalid";
    let actual_result: PrinterState = status_string.parse().unwrap();
    assert_eq!(actual_result, PrinterState::default());
}

#[test]
fn test_parse_invalid_value() {
    let status_string = "B: 1/two E1: 3/4";
    let actual_result: PrinterState = status_string.parse().unwrap();
    assert_eq!(actual_result, PrinterState::default());
}

#[test]
fn test_parse_valid_pair() {
    let pair_str = "1/2";
    let expected_pair = Pair {
        current: 1,
        target: 2,
    };
    let actual_pair: Pair = pair_str.parse().unwrap();
    assert_eq!(actual_pair, expected_pair);
}

#[test]
fn test_parse_single_number_no_delimiter() {
    let pair_str = "1";
    let actual_pair: Pair = pair_str.parse().unwrap();
    assert_eq!(actual_pair, Pair::default());
}

#[test]
fn test_parse_invalid_number() {
    let pair_str = "one/two";
    let actual_result: Pair = pair_str.parse().unwrap();
    assert_eq!(actual_result, Pair::default());
}
