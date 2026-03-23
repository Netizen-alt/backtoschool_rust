use crate::common::cli::{Cli, Commands};
use crate::modules::school::models::SchoolDb;
use crate::modules::school::report::{build_course_report, build_student_report};
use crate::presentation::http::server::run_server;
use clap::Parser;

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
    let report = build_student_report(db, student_id)?;
    println!("=== Student Report ===");
    println!("ID: {}", report.student_id);
    println!("Name: {}", report.student_name);
    println!("Courses:");
    if report.courses.is_empty() {
        println!("- ยังไม่มีการลงทะเบียน");
        return Ok(());
    }
    for item in report.courses {
        match item.score {
            Some(score) => println!(
                "- {} ({}) | score: {:.1} | grade: {}",
                item.key,
                item.label,
                score,
                item.grade.unwrap_or_else(|| "-".to_string())
            ),
            None => println!("- {} ({}) | score: N/A", item.key, item.label),
        }
    }
    Ok(())
}

fn print_course_report(db: &SchoolDb, course_code: &str) -> Result<(), String> {
    let report = build_course_report(db, course_code)?;
    println!("=== Course Report ===");
    println!("Code: {}", report.course_code);
    println!("Title: {}", report.course_title);
    println!("Students:");
    if report.students.is_empty() {
        println!("- ยังไม่มีนักเรียนลงทะเบียน");
        return Ok(());
    }
    for item in report.students {
        match item.score {
            Some(score) => println!(
                "- {} ({}) | score: {:.1} | grade: {}",
                item.key,
                item.label,
                score,
                item.grade.unwrap_or_else(|| "-".to_string())
            ),
            None => println!("- {} ({}) | score: N/A", item.key, item.label),
        }
    }
    Ok(())
}

pub async fn run() -> Result<(), String> {
    let cli = Cli::parse();
    let mut db = SchoolDb::load(&cli.db)?;
    db.ensure_default_users();
    db.save(&cli.db)?;

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
        Commands::Serve => {
            return run_server(cli.db, cli.addr).await;
        }
    }

    Ok(())
}

