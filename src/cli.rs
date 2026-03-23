use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "backtoschool",
    version,
    about = "BackToSchool CLI: mini student management system"
)]
pub struct Cli {
    #[arg(long, default_value = "school_db.json")]
    pub db: PathBuf,
    #[arg(long, default_value = "127.0.0.1:3000")]
    pub addr: String,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
    Serve,
}

