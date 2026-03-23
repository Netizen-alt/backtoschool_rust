pub mod api;
pub mod app;
pub mod cli;
pub mod db;
pub mod models;
pub mod report;

pub use app::run;

#[cfg(test)]
mod tests {
    use crate::models::SchoolDb;
    use crate::report::{build_student_csv, build_student_pdf, build_student_report};

    fn mock_db() -> SchoolDb {
        let mut db = SchoolDb::default();
        db.add_student("S001".to_string(), "Somchai".to_string())
            .expect("student");
        db.add_course("CS101".to_string(), "Intro Rust".to_string())
            .expect("course");
        db.enroll("S001".to_string(), "CS101".to_string())
            .expect("enroll");
        db
    }

    #[test]
    fn grade_mapping_should_work() {
        assert_eq!(SchoolDb::score_to_grade(88.0), "A");
        assert_eq!(SchoolDb::score_to_grade(72.0), "B");
        assert_eq!(SchoolDb::score_to_grade(40.0), "F");
    }

    #[test]
    fn duplicate_enrollment_should_fail() {
        let mut db = mock_db();
        let result = db.enroll("S001".to_string(), "CS101".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn csv_and_pdf_export_should_generate_data() {
        let mut db = mock_db();
        db.update_grade("S001".to_string(), "CS101".to_string(), 90.0)
            .expect("grade");

        let student_report = build_student_report(&db, "S001").expect("student report");
        let csv = build_student_csv(&student_report).expect("csv");
        let pdf = build_student_pdf(&student_report);

        assert!(csv.starts_with(b"student_id,student_name"));
        assert!(pdf.starts_with(b"%PDF-1.4"));
    }
}

