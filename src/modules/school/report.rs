use crate::modules::school::models::SchoolDb;
use csv::WriterBuilder;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ReportItem {
    pub key: String,
    pub label: String,
    pub score: Option<f32>,
    pub grade: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct StudentReport {
    pub student_id: String,
    pub student_name: String,
    pub courses: Vec<ReportItem>,
}

#[derive(Serialize, Clone)]
pub struct CourseReport {
    pub course_code: String,
    pub course_title: String,
    pub students: Vec<ReportItem>,
}

pub fn build_student_report(db: &SchoolDb, student_id: &str) -> Result<StudentReport, String> {
    let student = db
        .students
        .get(student_id)
        .ok_or_else(|| format!("ไม่พบ student id `{student_id}`"))?;
    let mut courses = Vec::new();
    for e in &db.enrollments {
        if e.student_id == student.id {
            let title = db
                .courses
                .get(&e.course_code)
                .map(|c| c.title.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            courses.push(ReportItem {
                key: e.course_code.clone(),
                label: title,
                score: e.score,
                grade: e.score.map(|s| SchoolDb::score_to_grade(s).to_string()),
            });
        }
    }
    Ok(StudentReport {
        student_id: student.id.clone(),
        student_name: student.name.clone(),
        courses,
    })
}

pub fn build_course_report(db: &SchoolDb, course_code: &str) -> Result<CourseReport, String> {
    let course = db
        .courses
        .get(course_code)
        .ok_or_else(|| format!("ไม่พบ course code `{course_code}`"))?;
    let mut students = Vec::new();
    for e in &db.enrollments {
        if e.course_code == course.code {
            let name = db
                .students
                .get(&e.student_id)
                .map(|s| s.name.clone())
                .unwrap_or_else(|| "Unknown".to_string());
            students.push(ReportItem {
                key: e.student_id.clone(),
                label: name,
                score: e.score,
                grade: e.score.map(|s| SchoolDb::score_to_grade(s).to_string()),
            });
        }
    }
    Ok(CourseReport {
        course_code: course.code.clone(),
        course_title: course.title.clone(),
        students,
    })
}

pub fn build_student_csv(report: &StudentReport) -> Result<Vec<u8>, String> {
    let mut wtr = WriterBuilder::new().from_writer(vec![]);
    wtr.write_record([
        "student_id",
        "student_name",
        "course_code",
        "course_title",
        "score",
        "grade",
    ])
    .map_err(|e| e.to_string())?;
    for item in &report.courses {
        let score = item.score.map(|s| format!("{s:.1}")).unwrap_or_default();
        let grade = item.grade.clone().unwrap_or_default();
        wtr.write_record([
            report.student_id.as_str(),
            report.student_name.as_str(),
            item.key.as_str(),
            item.label.as_str(),
            score.as_str(),
            grade.as_str(),
        ])
        .map_err(|e| e.to_string())?;
    }
    wtr.into_inner().map_err(|e| e.to_string())
}

pub fn build_course_csv(report: &CourseReport) -> Result<Vec<u8>, String> {
    let mut wtr = WriterBuilder::new().from_writer(vec![]);
    wtr.write_record([
        "course_code",
        "course_title",
        "student_id",
        "student_name",
        "score",
        "grade",
    ])
    .map_err(|e| e.to_string())?;
    for item in &report.students {
        let score = item.score.map(|s| format!("{s:.1}")).unwrap_or_default();
        let grade = item.grade.clone().unwrap_or_default();
        wtr.write_record([
            report.course_code.as_str(),
            report.course_title.as_str(),
            item.key.as_str(),
            item.label.as_str(),
            score.as_str(),
            grade.as_str(),
        ])
        .map_err(|e| e.to_string())?;
    }
    wtr.into_inner().map_err(|e| e.to_string())
}

fn escape_pdf_text(input: &str) -> String {
    input
        .replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
}

fn build_simple_pdf(title: &str, lines: &[String]) -> Vec<u8> {
    let mut content = format!(
        "BT\n/F1 16 Tf\n50 800 Td\n({}) Tj\n/F1 12 Tf\n",
        escape_pdf_text(title)
    );
    for line in lines {
        content.push_str(&format!("0 -18 Td\n({}) Tj\n", escape_pdf_text(line)));
    }
    content.push_str("ET\n");

    let stream = format!(
        "4 0 obj\n<< /Length {} >>\nstream\n{}endstream\nendobj\n",
        content.len(),
        content
    );
    let obj1 = "1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n";
    let obj2 = "2 0 obj\n<< /Type /Pages /Kids [3 0 R] /Count 1 >>\nendobj\n";
    let obj3 = "3 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 595 842] /Contents 4 0 R /Resources << /Font << /F1 5 0 R >> >> >>\nendobj\n";
    let obj5 = "5 0 obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\nendobj\n";

    let mut pdf = Vec::new();
    pdf.extend_from_slice(b"%PDF-1.4\n");
    let mut offsets = vec![0usize];

    offsets.push(pdf.len());
    pdf.extend_from_slice(obj1.as_bytes());
    offsets.push(pdf.len());
    pdf.extend_from_slice(obj2.as_bytes());
    offsets.push(pdf.len());
    pdf.extend_from_slice(obj3.as_bytes());
    offsets.push(pdf.len());
    pdf.extend_from_slice(stream.as_bytes());
    offsets.push(pdf.len());
    pdf.extend_from_slice(obj5.as_bytes());

    let xref_start = pdf.len();
    pdf.extend_from_slice(b"xref\n0 6\n");
    pdf.extend_from_slice(b"0000000000 65535 f \n");
    for offset in offsets.iter().skip(1) {
        pdf.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    pdf.extend_from_slice(b"trailer\n<< /Size 6 /Root 1 0 R >>\nstartxref\n");
    pdf.extend_from_slice(format!("{xref_start}\n").as_bytes());
    pdf.extend_from_slice(b"%%EOF");
    pdf
}

pub fn build_student_pdf(report: &StudentReport) -> Vec<u8> {
    let mut lines = vec![format!(
        "Student: {} ({})",
        report.student_name, report.student_id
    )];
    for item in &report.courses {
        let score = item
            .score
            .map(|s| format!("{s:.1}"))
            .unwrap_or_else(|| "N/A".to_string());
        let grade = item.grade.clone().unwrap_or_else(|| "-".to_string());
        lines.push(format!(
            "{} {} | score={} grade={}",
            item.key, item.label, score, grade
        ));
    }
    build_simple_pdf("Student Report", &lines)
}

pub fn build_course_pdf(report: &CourseReport) -> Vec<u8> {
    let mut lines = vec![format!(
        "Course: {} ({})",
        report.course_title, report.course_code
    )];
    for item in &report.students {
        let score = item
            .score
            .map(|s| format!("{s:.1}"))
            .unwrap_or_else(|| "N/A".to_string());
        let grade = item.grade.clone().unwrap_or_else(|| "-".to_string());
        lines.push(format!(
            "{} {} | score={} grade={}",
            item.key, item.label, score, grade
        ));
    }
    build_simple_pdf("Course Report", &lines)
}

