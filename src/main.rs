#![allow(warnings)]

#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}

impl Student {
    /// Returns if students succeeded or not
    /// ## Example
    /// ```rust
    /// let student = Student { name: "Foo".to_owned(), gpa: 2.5 };
    /// assert!(!student.is_succeeded());
    /// ```
    fn is_succeeded(&self) -> bool {
        // When you but succeeded statement in method
        // help you to easy update it later
        self.gpa >= 3.5
    }
}

fn main() {
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoliy 4.0"
    ];

    let good_students: Vec<Student> = students.iter()
        // `filter_map` help you to filter and map in same time
        .filter_map(|s| {
            let mut s = s.split(' ');
            let name = s.next()?.to_owned();
            let gpa = s.next()?.parse::<f32>().ok()?;
            let student = Student { name, gpa };
            if student.is_succeeded() {
                Some(student)
            } else {
                None
            }
        })
        .collect();

    for s in good_students {
        println!("{:?}", s);
    }

    // ----------------------------------

    let mut good_students = vec![];

    for s in students {
        let mut s = s.split(' ');
        let name = s.next();
        let gpa = s.next();

        if name.is_some() && gpa.is_some() {
            let name = name.unwrap().to_owned();
            let gpa = gpa.unwrap().to_owned();

            let gpa = gpa.parse::<f32>();

            if gpa.is_ok() {
                let gpa = gpa.unwrap();
                if gpa >= 3.5 {
                    good_students.push(Student { name, gpa });
                }
            }
        }
    }

    for s in good_students {
        println!("{:?}", s);
    }
}
