use std::collections::HashMap;

#[derive(Debug)]
struct Course {
    name_course: String,
    depend: u32,
}

fn fill_data() -> Vec<Course> {
    let data_courses: Vec<Course> = vec![
        Course { name_course: String::from("Math"), depend: 0 },
        Course { name_course: String::from("Diff Calculus"), depend: 1 },
        Course { name_course: String::from("Integral Calculus"), depend: 2 },
        Course { name_course: String::from("Vectors Calculus"), depend: 2 },
        Course { name_course: String::from("Vectors Calculus"), depend: 3 },
        Course { name_course: String::from("Integral Calculus"), depend: 0 },
        Course { name_course: String::from("Integral Calculus"), depend: 1 },
        Course { name_course: String::from("Sensorial Calculus"), depend: 4 },
    ];
    return data_courses;
}


pub fn group_by_name_course() {
    let mut data_dictionary: HashMap<String, Vec<u32>> = HashMap::new();
    let data_course = fill_data();
    for course in data_course.iter() {
        if !(data_dictionary.contains_key(&course.name_course)) {
            data_dictionary.insert((course.name_course).parse().unwrap(), vec![course.depend]);
        } else {
            data_dictionary.entry(course.name_course.clone()).and_modify(|c| c.push(course.depend));
        }
    }
    dbg!(data_dictionary);
}