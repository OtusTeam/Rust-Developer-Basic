use std::{collections::BTreeMap, num::NonZeroU32};

#[derive(Debug, Clone)]
pub struct Student {
    pub first_name: String,
    pub last_name: String,
    pub age: u32,
}

impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.age.eq(&other.age)
    }
}

impl Eq for Student {}

impl PartialOrd for Student {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.age.partial_cmp(&other.age)
    }
}

impl Ord for Student {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.age.cmp(&other.age)
    }
}

#[derive(Clone, Copy)]
struct GradeData {
    pub avg_grade: f32,
    pub count: NonZeroU32,
}

#[derive(Default)]
pub struct StudentStorage {
    students_collection: BTreeMap<Student, Option<GradeData>>,
}

impl StudentStorage {
    pub fn insert(&mut self, student: Student) {
        self.students_collection.insert(student, None);
    }

    pub fn students_by_avg_grade(&self) -> impl Iterator<Item = (&Student, f32)> {
        self.students_collection.iter().map(|(key, value)| {
            (
                key,
                value
                    .map(|grade_data| grade_data.avg_grade)
                    .unwrap_or_default(),
            )
        })
    }

    pub fn add_grade(&mut self, student: &Student, grade: u32) {
        match self.students_collection.get_mut(student) {
            Some(grade_data_opt) => match grade_data_opt {
                Some(ref mut grade_data) => {
                    let count = grade_data.count.checked_add(1).unwrap();
                    let avg_grade = ((grade_data.avg_grade * u32::from(grade_data.count) as f32)
                        + grade as f32)
                        / u32::from(count) as f32;
                    grade_data.avg_grade = avg_grade;
                    grade_data.count = count;
                }
                None => {
                    let _ = grade_data_opt.insert(GradeData {
                        avg_grade: grade as f32,
                        count: NonZeroU32::try_from(1).unwrap(),
                    });
                }
            },
            None => {
                self.students_collection.insert(
                    student.clone(),
                    Some(GradeData {
                        avg_grade: grade as f32,
                        count: NonZeroU32::try_from(1).unwrap(),
                    }),
                );
            }
        };
    }
}

fn main() {
    let mut storage = StudentStorage::default();

    let alice = Student {
        first_name: "Alice".to_owned(),
        last_name: "1".to_owned(),
        age: 22,
    };
    let mathew = Student {
        first_name: "Mathew".to_owned(),
        last_name: "2".to_owned(),
        age: 20,
    };
    storage.insert(alice.clone());
    storage.insert(mathew.clone());

    storage.add_grade(&alice, 5);
    storage.add_grade(&alice, 4);

    storage.add_grade(&mathew, 3);
    storage.add_grade(&mathew, 3);

    for (student, avg_grade) in storage.students_by_avg_grade() {
        println!("{student:?}: {avg_grade}")
    }
}
