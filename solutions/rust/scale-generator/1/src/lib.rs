// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub struct Error;

pub struct Scale {
    pitches: Vec<String>,
}

const CHROMATIC_SCALE_SHARP: [&str; 12] = [
    "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
];
const CHROMATIC_SCALE_FLAT: [&str; 12] = [
    "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
];

const USE_SHARPS: [&str; 14] = [
    "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#", "C", "a",
];
const USE_FLATS: [&str; 12] = [
    "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
];

fn get_chromatic_pitches(tonic: &str) -> Result<Vec<String>, Error> {
    if let Some(chromatic_scale) = if USE_SHARPS.contains(&tonic) {
        Some(CHROMATIC_SCALE_SHARP)
    } else if USE_FLATS.contains(&tonic) {
        Some(CHROMATIC_SCALE_FLAT)
    } else {
        None
    } {
        let tonic = tonic[..1].to_uppercase().to_string() + &tonic[1..];
        if let Some(i) = chromatic_scale.iter().position(|&x| x == tonic) {
            let pitches = chromatic_scale
                .iter()
                .cycle()
                .skip(i)
                .take(13)
                .map(|x| x.to_string())
                .collect();
            return Ok(pitches);
        }
    }
    Err(Error)
}

impl Scale {
    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        let intervals = format!("_{intervals}");
        let pitches = get_chromatic_pitches(tonic)?;

        let result = intervals
            .chars()
            .scan(pitches.iter().cloned(), |pit, c| {
                pit.skip(match c {
                    'A' => 2,
                    'M' => 1,
                    'm' | _ => 0,
                })
                .next()
            })
            .collect::<Vec<_>>();
        Ok(Scale { pitches: result })
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        Ok(Scale {
            pitches: get_chromatic_pitches(tonic)?,
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.pitches.clone()
    }
}
