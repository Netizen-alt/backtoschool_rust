use crate::modules::school::models::{Course, Enrollment, Role, SchoolDb, Student, User};
use std::fs;
use std::path::Path;

impl SchoolDb {
    pub fn load(path: &Path) -> Result<Self, String> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let text = fs::read_to_string(path).map_err(|e| format!("อ่านไฟล์ไม่สำเร็จ: {e}"))?;
        serde_json::from_str(&text).map_err(|e| format!("แปลง JSON ไม่สำเร็จ: {e}"))
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        let text = serde_json::to_string_pretty(self)
            .map_err(|e| format!("แปลงข้อมูลเป็น JSON ไม่สำเร็จ: {e}"))?;
        fs::write(path, text).map_err(|e| format!("บันทึกไฟล์ไม่สำเร็จ: {e}"))
    }

    pub fn ensure_default_users(&mut self) {
        if self.users.is_empty() {
            self.users.insert(
                "admin".to_string(),
                User {
                    username: "admin".to_string(),
                    password: "admin123".to_string(),
                    role: Role::Admin,
                },
            );
            self.users.insert(
                "teacher".to_string(),
                User {
                    username: "teacher".to_string(),
                    password: "teacher123".to_string(),
                    role: Role::Teacher,
                },
            );
        }
    }

    pub fn add_student(&mut self, id: String, name: String) -> Result<(), String> {
        if self.students.contains_key(&id) {
            return Err(format!("มี student id `{id}` อยู่แล้ว"));
        }
        self.students.insert(id.clone(), Student { id, name });
        Ok(())
    }

    pub fn add_course(&mut self, code: String, title: String) -> Result<(), String> {
        if self.courses.contains_key(&code) {
            return Err(format!("มี course code `{code}` อยู่แล้ว"));
        }
        self.courses.insert(code.clone(), Course { code, title });
        Ok(())
    }

    pub fn enroll(&mut self, student_id: String, course_code: String) -> Result<(), String> {
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

    pub fn update_grade(
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

    pub fn unenroll(&mut self, student_id: &str, course_code: &str) -> Result<(), String> {
        let initial_len = self.enrollments.len();
        self.enrollments.retain(|e| !(e.student_id == student_id && e.course_code == course_code));
        if self.enrollments.len() == initial_len {
            return Err("ไม่พบรายการลงทะเบียนนี้".to_string());
        }
        Ok(())
    }

    pub fn delete_student(&mut self, id: &str) -> Result<(), String> {
        if self.students.remove(id).is_none() {
            return Err("ไม่พบนักเรียนนี้".to_string());
        }
        self.enrollments.retain(|e| e.student_id != id);
        Ok(())
    }

    pub fn delete_course(&mut self, code: &str) -> Result<(), String> {
        if self.courses.remove(code).is_none() {
            return Err("ไม่พบรายวิชานี้".to_string());
        }
        self.enrollments.retain(|e| e.course_code != code);
        Ok(())
    }

    pub fn update_student(&mut self, id: &str, new_name: String) -> Result<(), String> {
        if let Some(student) = self.students.get_mut(id) {
            student.name = new_name;
            Ok(())
        } else {
            Err("ไม่พบนักเรียนนี้".to_string())
        }
    }

    pub fn update_course(&mut self, code: &str, new_title: String) -> Result<(), String> {
        if let Some(course) = self.courses.get_mut(code) {
            course.title = new_title;
            Ok(())
        } else {
            Err("ไม่พบรายวิชานี้".to_string())
        }
    }

    pub fn score_to_grade(score: f32) -> &'static str {
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

