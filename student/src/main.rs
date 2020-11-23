
#[derive(Debug)]
struct Student {
    id: String,
    name: String,
    physics_marks: i64,
    math_marks: i64,
    english_marks: i64
}


fn main() {
    let mut no_of_students = String::new();

    println!("Enter total no. of students : ");

    std::io::stdin().read_line(&mut no_of_students).unwrap();
    let no_of_students = no_of_students.trim().parse::<i64>().unwrap();

    println!("Total no. of students : {}", no_of_students);

    let mut students: Vec<Student> = Vec::new();

    for i in 0..no_of_students {
        let mut id = String::new();
        let mut name = String::new();
        let mut physics_marks = String::new();
        let mut math_marks = String::new();
        let mut english_marks = String::new();

        println!("Enter ID of Student {} : ",i+1);
        std::io::stdin().read_line(&mut id).unwrap();
        println!("Enter name of student {} : ",i+1);
        std::io::stdin().read_line(&mut name).unwrap();
        println!("Enter physics marks of student {} : ",i+1);
        std::io::stdin().read_line(&mut physics_marks).unwrap();
        let physics_marks = physics_marks.trim().parse::<i64>().unwrap();
        println!("Enter maths marks of student {} : ",i+1);
        std::io::stdin().read_line(&mut math_marks).unwrap();
        let math_marks = math_marks.trim().parse::<i64>().unwrap();
        println!("Enter english marks of student {} : ",i+1);
        std::io::stdin().read_line(&mut english_marks).unwrap();
        let english_marks = english_marks.trim().parse::<i64>().unwrap();

        let student = Student{
            name: name,
            id: id,
            physics_marks: physics_marks,
            math_marks: math_marks,
            english_marks: english_marks,
        };

        students.push(student)
    }

    // println!("vector : {:#?}",students);

    loop {
        println!();
        println!("Order list by : ");
        println!("1. Sort by name");
        println!("2. Sort by marks");
        println!("3. Exit");
        println!("Enter your choice : ");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice);
        let choice = choice.trim().parse::<i32>().unwrap();

        match choice {
            1 => {
                students.sort_by_key(|student| student.name.clone());
            },
            2 => {
                students.sort_by_key(|student| {student.physics_marks + student.math_marks + student.english_marks});
                students.reverse()
            },
            3 => {
                break;
            },
            _ => {
            }
        }

        for student in students.iter() {
            let total_marks = student.physics_marks + student.math_marks + student.english_marks;
            let total_marks_f = total_marks as f32;
            let percentage = (total_marks_f/300.0) * 100.0;
            println!();
            println!("Student ID : {}",student.id);
            println!("Student Name : {}",student.name);
            println!("Total Marks : {}",total_marks);
            println!("Percentage : {}",percentage);
        }
    }
}
