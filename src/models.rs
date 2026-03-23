use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Teacher,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Student {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub code: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Enrollment {
    pub student_id: String,
    pub course_code: String,
    pub score: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct SchoolDb {
    pub students: HashMap<String, Student>,
    pub courses: HashMap<String, Course>,
    pub enrollments: Vec<Enrollment>,
    pub users: HashMap<String, User>,
}

