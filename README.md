# BackToSchool Rust Project

โปรเจกต์งานจบ BackToSchool ด้วย Rust รองรับทั้ง `CLI` และ `REST API` พร้อมระบบ `login + role` และ `export รายงานเป็น CSV/PDF`

## ฟีเจอร์หลัก

- จัดการข้อมูลนักเรียน/รายวิชา/การลงทะเบียน/คะแนน
- ระบบ login (Bearer token)
- Role-based authorization: `admin`, `teacher`
- REST API ด้วย `axum`
- Export รายงานรายนักเรียนและรายวิชาเป็น `CSV` และ `PDF`
- Unit test ครอบคลุม logic หลัก

## Quick Start

```bash
cargo run -- --help
```

## CLI ตัวอย่าง

```bash
cargo run -- add-student S001 "Somchai"
cargo run -- add-course CS101 "Intro to Rust"
cargo run -- enroll S001 CS101
cargo run -- grade S001 CS101 87.5
cargo run -- report-student S001
cargo run -- report-course CS101
```

## รัน REST API

```bash
cargo run -- serve
```

หรือกำหนด host/port เอง:

```bash
cargo run -- --addr 127.0.0.1:4000 serve
```

## Default Users

- `admin` / `admin123` (สิทธิ์ admin)
- `teacher` / `teacher123` (สิทธิ์ teacher)

## API ตัวอย่าง

1) Login เพื่อรับ token

```bash
curl -X POST http://127.0.0.1:3000/login \
  -H "Content-Type: application/json" \
  -d "{\"username\":\"teacher\",\"password\":\"teacher123\"}"
```

1) เรียก API ที่ต้อง auth

```bash
curl http://127.0.0.1:3000/students \
  -H "Authorization: Bearer <TOKEN>"
```

1) Export รายงาน

```bash
# CSV
curl -L "http://127.0.0.1:3000/reports/student/S001/csv" \
  -H "Authorization: Bearer <TOKEN>" \
  -o student_S001.csv

# PDF
curl -L "http://127.0.0.1:3000/reports/student/S001/pdf" \
  -H "Authorization: Bearer <TOKEN>" \
  -o student_S001.pdf
```

## Endpoints

- `GET /health`
- `POST /login`
- `GET/POST /students` (teacher/admin)
- `GET/POST /courses` (POST ต้อง admin)
- `POST /enroll` (teacher/admin)
- `POST /grade` (teacher/admin)
- `GET /reports/student/{student_id}`
- `GET /reports/course/{course_code}`
- `GET /reports/student/{student_id}/csv`
- `GET /reports/course/{course_code}/csv`
- `GET /reports/student/{student_id}/pdf`
- `GET /reports/course/{course_code}/pdf`

## ทดสอบ

```bash
cargo test
```

> ค่าเริ่มต้นไฟล์ฐานข้อมูลคือ `school_db.json`  
> เปลี่ยนได้ด้วย `--db` เช่น `cargo run -- --db my_school_data.json serve`
