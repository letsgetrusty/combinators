#![allow(warnings)]

#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}

fn main() {
    let s1 = "Bogdan 3.1";
    let s2 = "Wallace 2.3";
    let s3 = "Lidiya 3.5";
    let s4 = "Kyle 3.9";
    let s5 = "Anatoliy 4.0";

    let students = vec![s1, s2, s3, s4, s5];
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

    // let students: Vec<Student> = students.iter()
    //     .map(|s| {
    //         let s = s.split(' ');
    //         let name = s.next();
    //         let gpa = s.next();
            // if name.is_some() && gpa.is_some() {
            //     let name = name.unwrap().to_owned();
            //     let gpa = gpa.unwrap().parse::<f32>().unwrap();
            //     Some(Student {
            //         name,
            //         gpa
            //     })
            // } else {
            //     None
            // }
    //     })
    //     .filter_map(|s| s)
    //     .filter(|s| s.gpa >= 3.0)
    //     .collect();

    // let students: Vec<Student> = students.iter()
    //     .map(|s| {
    //         let mut s = s.to_owned().split(' ');
    //         let name = s.next()?.to_owned();
    //         let gpa = s.next()?.parse::<f32>().ok()?;

    //         Some(Student { name, gpa })
    //     })
    //     .filter_map(|s| s)
    //     .filter(|s| s.gpa >= 3.0)
    //     .collect();

    // Final example
    // let students: Vec<Student> = students.iter()
    //     .map(|s| {
    //         let mut s = s.split(' ');
    //         let name = s.next()?.to_owned();
    //         let gpa = s.next()?.parse::<f32>().ok()?;

    //         Some(Student { name, gpa })
    //     })
    //     .flatten()
    //     .filter(|s| s.gpa >= 3.5)
    //     .collect();

    // for s in students {
    //     println!("{:?}", s);
    // }

    // ok - convert result to option.
    // or, and, or_else, and_then
    // map, map_err, map_or, map_or_else
    // filter, collect
    // ok_or, ok_or_else
    // as_ref
    // as_mut
}
