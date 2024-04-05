use std::{
    collections::{BTreeSet, HashMap},
    num::NonZeroU32,
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Student {
    age: u8,
    name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct StudentWithGrade {
    grade_info: Option<GradeInfo>,
    student: Student,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Grade(f32);

impl Ord for Grade {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl Eq for Grade {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct GradeInfo {
    avg: Grade,
    count: NonZeroU32,
}

#[derive(Default)]
struct StudentsStorage {
    students: BTreeSet<Rc<StudentWithGrade>>,
    name_to_student: HashMap<String, Rc<StudentWithGrade>>,
}

impl StudentsStorage {
    /// O(logN)
    fn add_student(&mut self, student: Student) {
        let name = student.name.clone();
        let student_with_grade = Rc::new(StudentWithGrade {
            student,
            grade_info: None,
        });
        self.students.insert(Rc::clone(&student_with_grade));
        self.name_to_student.insert(name, student_with_grade);
    }

    /// O(logN)
    fn add_grade(&mut self, name: &str, grade: u8) -> bool {
        let Some(mut student_with_grade_rc) = self.name_to_student.get_mut(name) else {
            return false;
        };
        self.students.remove(student_with_grade_rc);

        let student_with_grade = Rc::get_mut(&mut student_with_grade_rc).unwrap();
        student_with_grade.grade_info = Some(student_with_grade.grade_info.as_ref().map_or_else(
            || GradeInfo {
                avg: Grade(grade as f32),
                count: NonZeroU32::MIN,
            },
            |grade_info| {
                let new_count = grade_info.count.checked_add(1).unwrap();
                GradeInfo {
                    avg: Grade(
                        (grade_info.avg.0 * u32::from(grade_info.count) as f32 + grade as f32)
                            / u32::from(new_count) as f32,
                    ),
                    count: new_count,
                }
            },
        ));

        self.students.insert(Rc::clone(student_with_grade_rc));
        true
    }

    /// O(1)
    fn students(&self) -> impl Iterator<Item = (&Student, f32)> {
        self.students.iter().map(|student_with_grade| {
            (
                &student_with_grade.student,
                student_with_grade
                    .grade_info
                    .as_ref()
                    .map(|grade_info| grade_info.avg.0)
                    .unwrap_or_default(),
            )
        })
    }
}

fn main() {
    let mut storage = StudentsStorage::default();

    let john_name = "John".to_owned();
    let john = Student {
        name: john_name.clone(),
        age: 20,
    };

    let alice_name = "Alice".to_owned();
    let alice = Student {
        name: alice_name.clone(),
        age: 22,
    };

    let bob = Student {
        name: "Bob".to_owned(),
        age: 22,
    };

    let charlie_name = "Charlie".to_owned();
    let charlie = Student {
        name: charlie_name.clone(),
        age: 22,
    };

    storage.add_student(alice);
    storage.add_student(john);
    storage.add_student(charlie);
    storage.add_student(bob);

    assert!(storage.add_grade(&charlie_name, 1));
    assert!(storage.add_grade(&alice_name, 5));
    assert!(storage.add_grade(&charlie_name, 2));
    assert!(storage.add_grade(&john_name, 4));
    assert!(!storage.add_grade(&"Daniil", 2));
    assert!(storage.add_grade(&charlie_name, 3));
    assert!(storage.add_grade(&john_name, 5));

    for (student, avg_grade) in storage.students() {
        println!("Student: {student:?} with avg grade: {avg_grade}");
    }
}
