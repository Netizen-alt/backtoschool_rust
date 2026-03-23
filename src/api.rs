use crate::models::{Course, Role, SchoolDb, Student};
use crate::report::{
    build_course_csv, build_course_pdf, build_course_report, build_student_csv, build_student_pdf,
    build_student_report, CourseReport, StudentReport,
};
use axum::extract::{Path as AxumPath, State};
use axum::http::header::{AUTHORIZATION, CONTENT_DISPOSITION, CONTENT_TYPE};
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Mutex<SchoolDb>>,
    pub db_path: PathBuf,
    pub sessions: Arc<Mutex<HashMap<String, SessionUser>>>,
}

#[derive(Clone)]
pub struct SessionUser {
    pub role: Role,
}

#[derive(Serialize)]
struct ApiMessage {
    message: String,
}

#[derive(Serialize)]
struct ApiError {
    error: String,
}

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    role: Role,
}

#[derive(Deserialize)]
struct AddStudentRequest {
    id: String,
    name: String,
}

#[derive(Deserialize)]
struct AddCourseRequest {
    code: String,
    title: String,
}

#[derive(Deserialize)]
struct EnrollRequest {
    student_id: String,
    course_code: String,
}

#[derive(Deserialize)]
struct GradeRequest {
    student_id: String,
    course_code: String,
    score: f32,
}

fn api_error(code: StatusCode, message: impl Into<String>) -> (StatusCode, Json<ApiError>) {
    (
        code,
        Json(ApiError {
            error: message.into(),
        }),
    )
}

fn require_auth(
    headers: &HeaderMap,
    state: &AppState,
) -> Result<SessionUser, (StatusCode, Json<ApiError>)> {
    let token = headers
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            api_error(
                StatusCode::UNAUTHORIZED,
                "Missing or invalid Authorization header",
            )
        })?;
    let sessions = state
        .sessions
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot read session data"))?;
    sessions
        .get(token)
        .cloned()
        .ok_or_else(|| api_error(StatusCode::UNAUTHORIZED, "Invalid token"))
}

fn require_role(user: &SessionUser, required: Role) -> Result<(), (StatusCode, Json<ApiError>)> {
    if user.role == required || user.role == Role::Admin {
        Ok(())
    } else {
        Err(api_error(StatusCode::FORBIDDEN, "Insufficient role"))
    }
}

fn save_db_locked(state: &AppState, db: &SchoolDb) -> Result<(), (StatusCode, Json<ApiError>)> {
    db.save(&state.db_path)
        .map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, e))
}

async fn health() -> Json<ApiMessage> {
    Json(ApiMessage {
        message: "ok".to_string(),
    })
}

async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ApiError>)> {
    let db = state
        .db
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot read db"))?;
    let user = db
        .users
        .get(&req.username)
        .ok_or_else(|| api_error(StatusCode::UNAUTHORIZED, "Invalid username/password"))?;
    if user.password != req.password {
        return Err(api_error(
            StatusCode::UNAUTHORIZED,
            "Invalid username/password",
        ));
    }
    let role = user.role;
    let token = Uuid::new_v4().to_string();
    drop(db);
    let mut sessions = state
        .sessions
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot write sessions"))?;
    sessions.insert(token.clone(), SessionUser { role });
    Ok(Json(LoginResponse { token, role }))
}

async fn add_student_api(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<AddStudentRequest>,
) -> Result<Json<ApiMessage>, (StatusCode, Json<ApiError>)> {
    let user = require_auth(&headers, &state)?;
    require_role(&user, Role::Teacher)?;
    let mut db = state
        .db
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot write db"))?;
    db.add_student(req.id, req.name)
        .map_err(|e| api_error(StatusCode::BAD_REQUEST, e))?;
    save_db_locked(&state, &db)?;
    Ok(Json(ApiMessage {
        message: "เพิ่มนักเรียนสำเร็จ".to_string(),
    }))
}

async fn add_course_api(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<AddCourseRequest>,
) -> Result<Json<ApiMessage>, (StatusCode, Json<ApiError>)> {
    let user = require_auth(&headers, &state)?;
    require_role(&user, Role::Admin)?;
    let mut db = state
        .db
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot write db"))?;
    db.add_course(req.code, req.title)
        .map_err(|e| api_error(StatusCode::BAD_REQUEST, e))?;
    save_db_locked(&state, &db)?;
    Ok(Json(ApiMessage {
        message: "เพิ่มรายวิชาสำเร็จ".to_string(),
    }))
}

async fn enroll_api(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<EnrollRequest>,
) -> Result<Json<ApiMessage>, (StatusCode, Json<ApiError>)> {
    let user = require_auth(&headers, &state)?;
    require_role(&user, Role::Teacher)?;
    let mut db = state
        .db
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot write db"))?;
    db.enroll(req.student_id, req.course_code)
        .map_err(|e| api_error(StatusCode::BAD_REQUEST, e))?;
    save_db_locked(&state, &db)?;
    Ok(Json(ApiMessage {
        message: "ลงทะเบียนสำเร็จ".to_string(),
    }))
}

async fn grade_api(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<GradeRequest>,
) -> Result<Json<ApiMessage>, (StatusCode, Json<ApiError>)> {
    let user = require_auth(&headers, &state)?;
    require_role(&user, Role::Teacher)?;
    let mut db = state
        .db
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot write db"))?;
    db.update_grade(req.student_id, req.course_code, req.score)
        .map_err(|e| api_error(StatusCode::BAD_REQUEST, e))?;
    save_db_locked(&state, &db)?;
    Ok(Json(ApiMessage {
        message: "อัปเดตคะแนนสำเร็จ".to_string(),
    }))
}

async fn list_students_api(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<Student>>, (StatusCode, Json<ApiError>)> {
    let _user = require_auth(&headers, &state)?;
    let db = state
        .db
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot read db"))?;
    Ok(Json(db.students.values().cloned().collect()))
}

async fn list_courses_api(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<Course>>, (StatusCode, Json<ApiError>)> {
    let _user = require_auth(&headers, &state)?;
    let db = state
        .db
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot read db"))?;
    Ok(Json(db.courses.values().cloned().collect()))
}

async fn report_student_api(
    State(state): State<AppState>,
    headers: HeaderMap,
    AxumPath(student_id): AxumPath<String>,
) -> Result<Json<StudentReport>, (StatusCode, Json<ApiError>)> {
    let _user = require_auth(&headers, &state)?;
    let db = state
        .db
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot read db"))?;
    let report =
        build_student_report(&db, &student_id).map_err(|e| api_error(StatusCode::NOT_FOUND, e))?;
    Ok(Json(report))
}

async fn report_course_api(
    State(state): State<AppState>,
    headers: HeaderMap,
    AxumPath(course_code): AxumPath<String>,
) -> Result<Json<CourseReport>, (StatusCode, Json<ApiError>)> {
    let _user = require_auth(&headers, &state)?;
    let db = state
        .db
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot read db"))?;
    let report =
        build_course_report(&db, &course_code).map_err(|e| api_error(StatusCode::NOT_FOUND, e))?;
    Ok(Json(report))
}

async fn export_student_csv_api(
    State(state): State<AppState>,
    headers: HeaderMap,
    AxumPath(student_id): AxumPath<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiError>)> {
    let _user = require_auth(&headers, &state)?;
    let db = state
        .db
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot read db"))?;
    let report =
        build_student_report(&db, &student_id).map_err(|e| api_error(StatusCode::NOT_FOUND, e))?;
    let bytes = build_student_csv(&report).map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, e))?;
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/csv; charset=utf-8"),
    );
    let file_name = format!("student_{student_id}.csv");
    headers.insert(
        CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{file_name}\""))
            .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Header error"))?,
    );
    Ok((headers, bytes))
}

async fn export_course_csv_api(
    State(state): State<AppState>,
    headers: HeaderMap,
    AxumPath(course_code): AxumPath<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiError>)> {
    let _user = require_auth(&headers, &state)?;
    let db = state
        .db
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot read db"))?;
    let report =
        build_course_report(&db, &course_code).map_err(|e| api_error(StatusCode::NOT_FOUND, e))?;
    let bytes = build_course_csv(&report).map_err(|e| api_error(StatusCode::INTERNAL_SERVER_ERROR, e))?;
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/csv; charset=utf-8"),
    );
    let file_name = format!("course_{course_code}.csv");
    headers.insert(
        CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{file_name}\""))
            .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Header error"))?,
    );
    Ok((headers, bytes))
}

async fn export_student_pdf_api(
    State(state): State<AppState>,
    headers: HeaderMap,
    AxumPath(student_id): AxumPath<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiError>)> {
    let _user = require_auth(&headers, &state)?;
    let db = state
        .db
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot read db"))?;
    let report =
        build_student_report(&db, &student_id).map_err(|e| api_error(StatusCode::NOT_FOUND, e))?;
    let bytes = build_student_pdf(&report);
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/pdf"));
    let file_name = format!("student_{student_id}.pdf");
    headers.insert(
        CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{file_name}\""))
            .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Header error"))?,
    );
    Ok((headers, bytes))
}

async fn export_course_pdf_api(
    State(state): State<AppState>,
    headers: HeaderMap,
    AxumPath(course_code): AxumPath<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<ApiError>)> {
    let _user = require_auth(&headers, &state)?;
    let db = state
        .db
        .lock()
        .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Cannot read db"))?;
    let report =
        build_course_report(&db, &course_code).map_err(|e| api_error(StatusCode::NOT_FOUND, e))?;
    let bytes = build_course_pdf(&report);
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/pdf"));
    let file_name = format!("course_{course_code}.pdf");
    headers.insert(
        CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{file_name}\""))
            .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Header error"))?,
    );
    Ok((headers, bytes))
}

pub async fn run_server(db_path: PathBuf, addr: String) -> Result<(), String> {
    let mut db = SchoolDb::load(&db_path)?;
    db.ensure_default_users();
    db.save(&db_path)?;

    let state = AppState {
        db: Arc::new(Mutex::new(db)),
        db_path,
        sessions: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/login", post(login))
        .route("/students", get(list_students_api).post(add_student_api))
        .route("/courses", get(list_courses_api).post(add_course_api))
        .route("/enroll", post(enroll_api))
        .route("/grade", post(grade_api))
        .route("/reports/student/{student_id}", get(report_student_api))
        .route("/reports/course/{course_code}", get(report_course_api))
        .route("/reports/student/{student_id}/csv", get(export_student_csv_api))
        .route("/reports/course/{course_code}/csv", get(export_course_csv_api))
        .route("/reports/student/{student_id}/pdf", get(export_student_pdf_api))
        .route("/reports/course/{course_code}/pdf", get(export_course_pdf_api))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| format!("bind {addr} ไม่สำเร็จ: {e}"))?;
    println!("REST API running at http://{addr}");
    println!("Default users: admin/admin123, teacher/teacher123");
    axum::serve(listener, app)
        .await
        .map_err(|e| format!("server error: {e}"))
}

