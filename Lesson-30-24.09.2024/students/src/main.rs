use std::{
    collections::{BTreeSet, HashMap},
    num::NonZeroU32,
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Student {
    name: String,
    age: u8,
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct StudentAgeOrdered(Student);

// impl PartialOrd for StudentAgeOrdered {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for StudentAgeOrdered {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         (self.0.age, &self.0.name).cmp(&(other.0.age, &other.0.name))
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct StudentWithGrade {
    avg_grade: Option<AvgGrade>,
    student: Student,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct AvgGrade {
    avg: f32,
    count: NonZeroU32,
}

impl Eq for AvgGrade {}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for AvgGrade {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Default)]
struct StudentStorage {
    students: BTreeSet<Rc<StudentWithGrade>>,
    name_to_student: HashMap<String, Rc<StudentWithGrade>>,
}

impl StudentStorage {
    /// O(logN)
    fn add_student(&mut self, student: Student) {
        let student = Rc::new(StudentWithGrade {
            student,
            avg_grade: None,
        });
        self.insert_inner(student);
    }

    /// O(logN)
    fn add_grade(&mut self, name: &str, grade: u8) -> bool {
        let Some(mut student) = self.name_to_student.remove(name) else {
            return false;
        };
        self.students.remove(&student);

        let student_mut = Rc::get_mut(&mut student).unwrap();
        student_mut.avg_grade = Some(student_mut.avg_grade.as_ref().map_or_else(
            || AvgGrade {
                avg: grade as f32,
                count: NonZeroU32::MIN,
            },
            |avg_grade| {
                let new_count = avg_grade.count.checked_add(1).unwrap();
                let new_avg = (avg_grade.avg * u32::from(avg_grade.count) as f32 + grade as f32)
                    / u32::from(new_count) as f32;
                AvgGrade {
                    avg: new_avg,
                    count: new_count,
                }
            },
        ));

        self.insert_inner(student);

        true
    }

    /// O(1)
    fn student_grades(&self) -> impl Iterator<Item = (Option<f32>, &Student)> {
        self.students.iter().map(|student_with_grade| {
            (
                student_with_grade.avg_grade.as_ref().map(|grade| grade.avg),
                &student_with_grade.student,
            )
        })
    }

    fn insert_inner(&mut self, student: Rc<StudentWithGrade>) {
        self.students.insert(Rc::clone(&student));
        self.name_to_student
            .insert(student.student.name.clone(), student);
    }
}

fn main() {
    let mut storage = StudentStorage::default();

    let john = Student {
        name: "John".to_owned(),
        age: 20,
    };
    storage.add_student(john);
    storage.add_grade("John", 5);
    storage.add_grade("John", 2);

    let alice = Student {
        name: "Alice".to_owned(),
        age: 22,
    };
    storage.add_student(alice);
    storage.add_grade("Alice", 3);

    let charlie = Student {
        name: "Charlie".to_owned(),
        age: 25,
    };
    storage.add_student(charlie);

    storage.add_grade("Alice", 5);

    for (avg_grade, student) in storage.student_grades() {
        println!(
            "Student {} has avg grade {}",
            student.name,
            avg_grade
                .map(|avg| avg.to_string())
                .unwrap_or("<no data>".to_owned())
        );
    }
}
