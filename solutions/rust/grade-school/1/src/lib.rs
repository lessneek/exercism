use std::collections::BTreeMap;

pub struct School {
    grades: BTreeMap<u32, std::collections::BTreeSet<String>>,
}

#[allow(clippy::new_without_default)]
impl School {
    pub fn new() -> School {
        School {
            grades: Default::default(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades.entry(grade).or_insert(Default::default());
        self.grades.entry(grade).and_modify(|students| {
            students.insert(student.to_string());
        });
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().cloned().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades
            .get(&grade)
            .map_or(vec![], |x| x.iter().cloned().collect())
    }
}
