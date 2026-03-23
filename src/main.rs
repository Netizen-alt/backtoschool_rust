use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    name = "backtoschool",
    version,
    about = "BackToSchool CLI: mini student management system"
)]
struct Cli {
    #[arg(long, default_value = "school_db.json")]
    db: PathBuf,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    AddStudent { id: String, name: String },
    AddCourse { code: String, title: String },
    Enroll { student_id: String, course_code: String },
    Grade {
        student_id: String,
        course_code: String,
        score: f32,
    },
    ListStudents,
    ListCourses,
    ReportStudent { student_id: String },
    ReportCourse { course_code: String },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Student {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Course {
    code: String,
    title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Enrollment {
    student_id: String,
    course_code: String,
    score: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct SchoolDb {
    students: HashMap<String, Student>,
    courses: HashMap<String, Course>,
    enrollments: Vec<Enrollment>,
}

impl SchoolDb {
    fn load(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = fs::read_to_string(path).map_err(|e| format!("อ่านไฟล์ไม่สำเร็จ: {e}"))?;
        serde_json::from_str(&text).map_err(|e| format!("แปลง JSON ไม่สำเร็จ: {e}"))
    }

    fn save(&self, path: &Path) -> Result<(), String> {
        let text = serde_json::to_string_pretty(self)
            .map_err(|e| format!("แปลงข้อมูลเป็น JSON ไม่สำเร็จ: {e}"))?;
        fs::write(path, text).map_err(|e| format!("บันทึกไฟล์ไม่สำเร็จ: {e}"))
    }

    fn add_student(&mut self, id: String, name: String) -> Result<(), String> {
        if self.students.contains_key(&id) {
            return Err(format!("มี student id `{id}` อยู่แล้ว"));
        }
        self.students.insert(id.clone(), Student { id, name });
        Ok(())
    }

    fn add_course(&mut self, code: String, title: String) -> Result<(), String> {
        if self.courses.contains_key(&code) {
            return Err(format!("มี course code `{code}` อยู่แล้ว"));
        }
        self.courses.insert(code.clone(), Course { code, title });
        Ok(())
    }

    fn enroll(&mut self, student_id: String, course_code: String) -> Result<(), String> {
        if !self.students.contains_key(&student_id) {
            return Err(format!("ไม่พบ student id `{student_id}`"));
        }
        if !self.courses.contains_key(&course_code) {
            return Err(format!("ไม่พบ course code `{course_code}`"));
        }
        let exists = self
            .enrollments
            .iter()
            .any(|e| e.student_id == student_id && e.course_code == course_code);
        if exists {
            return Err("ลงทะเบียนซ้ำรายการเดิม".to_string());
        }
        self.enrollments.push(Enrollment {
            student_id,
            course_code,
            score: None,
        });
        Ok(())
    }

    fn update_grade(
        &mut self,
        student_id: String,
        course_code: String,
        score: f32,
    ) -> Result<(), String> {
        if !(0.0..=100.0).contains(&score) {
            return Err("คะแนนต้องอยู่ในช่วง 0 ถึง 100".to_string());
        }
        let enrollment = self
            .enrollments
            .iter_mut()
            .find(|e| e.student_id == student_id && e.course_code == course_code)
            .ok_or_else(|| "ไม่พบรายการลงทะเบียนของนักเรียนในรายวิชานี้".to_string())?;
        enrollment.score = Some(score);
        Ok(())
    }

    fn score_to_grade(score: f32) -> &'static str {
        if score >= 80.0 {
            "A"
        } else if score >= 75.0 {
            "B+"
        } else if score >= 70.0 {
            "B"
        } else if score >= 65.0 {
            "C+"
        } else if score >= 60.0 {
            "C"
        } else if score >= 55.0 {
            "D+"
        } else if score >= 50.0 {
            "D"
        } else {
            "F"
        }
    }
}

fn print_students(db: &SchoolDb) {
    if db.students.is_empty() {
        println!("ยังไม่มีข้อมูลนักเรียน");
        return;
    }
    println!("=== Students ===");
    for student in db.students.values() {
        println!("- {}: {}", student.id, student.name);
    }
}

fn print_courses(db: &SchoolDb) {
    if db.courses.is_empty() {
        println!("ยังไม่มีข้อมูลรายวิชา");
        return;
    }
    println!("=== Courses ===");
    for course in db.courses.values() {
        println!("- {}: {}", course.code, course.title);
    }
}

fn print_student_report(db: &SchoolDb, student_id: &str) -> Result<(), String> {
    let student = db
        .students
        .get(student_id)
        .ok_or_else(|| format!("ไม่พบ student id `{student_id}`"))?;

    println!("=== Student Report ===");
    println!("ID: {}", student.id);
    println!("Name: {}", student.name);
    println!("Courses:");

    let mut found = false;
    for e in &db.enrollments {
        if e.student_id == student.id {
            found = true;
            let title = db
                .courses
                .get(&e.course_code)
                .map(|c| c.title.as_str())
                .unwrap_or("Unknown");
            match e.score {
                Some(score) => println!(
                    "- {} ({}) | score: {:.1} | grade: {}",
                    e.course_code,
                    title,
                    score,
                    SchoolDb::score_to_grade(score)
                ),
                None => println!("- {} ({}) | score: N/A", e.course_code, title),
            }
        }
    }
    if !found {
        println!("- ยังไม่มีการลงทะเบียน");
    }
    Ok(())
}

fn print_course_report(db: &SchoolDb, course_code: &str) -> Result<(), String> {
    let course = db
        .courses
        .get(course_code)
        .ok_or_else(|| format!("ไม่พบ course code `{course_code}`"))?;

    println!("=== Course Report ===");
    println!("Code: {}", course.code);
    println!("Title: {}", course.title);
    println!("Students:");

    let mut found = false;
    for e in &db.enrollments {
        if e.course_code == course.code {
            found = true;
            let name = db
                .students
                .get(&e.student_id)
                .map(|s| s.name.as_str())
                .unwrap_or("Unknown");
            match e.score {
                Some(score) => println!(
                    "- {} ({}) | score: {:.1} | grade: {}",
                    e.student_id,
                    name,
                    score,
                    SchoolDb::score_to_grade(score)
                ),
                None => println!("- {} ({}) | score: N/A", e.student_id, name),
            }
        }
    }
    if !found {
        println!("- ยังไม่มีนักเรียนลงทะเบียน");
    }
    Ok(())
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();
    let mut db = SchoolDb::load(&cli.db)?;

    match cli.command {
        Commands::AddStudent { id, name } => {
            db.add_student(id, name)?;
            db.save(&cli.db)?;
            println!("เพิ่มนักเรียนสำเร็จ");
        }
        Commands::AddCourse { code, title } => {
            db.add_course(code, title)?;
            db.save(&cli.db)?;
            println!("เพิ่มรายวิชาสำเร็จ");
        }
        Commands::Enroll {
            student_id,
            course_code,
        } => {
            db.enroll(student_id, course_code)?;
            db.save(&cli.db)?;
            println!("ลงทะเบียนสำเร็จ");
        }
        Commands::Grade {
            student_id,
            course_code,
            score,
        } => {
            db.update_grade(student_id, course_code, score)?;
            db.save(&cli.db)?;
            println!("อัปเดตคะแนนสำเร็จ");
        }
        Commands::ListStudents => print_students(&db),
        Commands::ListCourses => print_courses(&db),
        Commands::ReportStudent { student_id } => print_student_report(&db, &student_id)?,
        Commands::ReportCourse { course_code } => print_course_report(&db, &course_code)?,
    }

    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}
