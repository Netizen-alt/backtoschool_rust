# BackToSchool Rust Project

โปรเจกต์ตัวอย่างสำหรับส่งงานจบ: ระบบจัดการนักเรียนและรายวิชาแบบ CLI ด้วย Rust

## คุณสมบัติ

- เพิ่มนักเรียน
- เพิ่มรายวิชา
- ลงทะเบียนเรียน
- บันทึกคะแนน
- รายงานผลรายนักเรียน/รายวิชา
- เก็บข้อมูลถาวรในไฟล์ JSON

## วิธีรัน

```bash
cargo run -- --help
```

## ตัวอย่างการใช้งาน

```bash
# เพิ่มนักเรียน
cargo run -- add-student S001 "Somchai"

# เพิ่มรายวิชา
cargo run -- add-course CS101 "Intro to Rust"

# ลงทะเบียน
cargo run -- enroll S001 CS101

# ใส่คะแนน
cargo run -- grade S001 CS101 87.5

# ดูรายงานนักเรียน
cargo run -- report-student S001

# ดูรายงานรายวิชา
cargo run -- report-course CS101
```

> ค่าเริ่มต้นไฟล์ฐานข้อมูลคือ `school_db.json`
> สามารถระบุไฟล์เองได้ เช่น:
>
> ```bash
> cargo run -- --db my_school_data.json list-students
> ```
