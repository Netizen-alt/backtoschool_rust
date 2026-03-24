use axum::response::Html;

pub async fn ui_page() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html lang="th">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>BackToSchool Test UI</title>
  <style>
    body { font-family: Arial, sans-serif; margin: 20px; background: #f6f8fb; }
    h1 { margin-bottom: 4px; }
    .muted { color: #666; margin-bottom: 16px; }
    .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 12px; }
    .card { background: #fff; border: 1px solid #dfe4ea; border-radius: 10px; padding: 12px; }
    .card h3 { margin-top: 0; }
    input, button, textarea, select { width: 100%; padding: 8px; margin: 4px 0; box-sizing: border-box; }
    button { cursor: pointer; border: 0; border-radius: 6px; background: #2f6feb; color: #fff; }
    button.secondary { background: #475569; }
    pre { background: #0f172a; color: #e2e8f0; padding: 10px; border-radius: 8px; overflow: auto; min-height: 120px; }
    .row { display: grid; grid-template-columns: 1fr 1fr; gap: 8px; }
  </style>
</head>
<body>
  <h1>BackToSchool Test UI</h1>
  <div class="muted">ใช้หน้านี้ทดสอบ API ได้ทันที (login, CRUD พื้นฐาน, report/export)</div>

  <div class="grid">
    <div class="card">
      <h3>Login</h3>
      <select id="username">
        <option value="teacher">teacher</option>
        <option value="admin">admin</option>
      </select>
      <input id="password" type="password" value="teacher123" />
      <button onclick="login()">Login</button>
      <input id="token" placeholder="Bearer token" />
      <small>ระบบจะใส่ token ให้หลัง login</small>
    </div>

    <div class="card">
      <h3>Add Student</h3>
      <input id="studentId" placeholder="S001" />
      <input id="studentName" placeholder="Somchai" />
      <button onclick="addStudent()">POST /students</button>
    </div>

    <div class="card">
      <h3>Add Course (admin)</h3>
      <input id="courseCode" placeholder="CS101" />
      <input id="courseTitle" placeholder="Intro to Rust" />
      <button onclick="addCourse()">POST /courses</button>
    </div>

    <div class="card">
      <h3>Enroll</h3>
      <div class="row">
        <input id="enrollStudentId" placeholder="S001" />
        <input id="enrollCourseCode" placeholder="CS101" />
      </div>
      <button onclick="enroll()">POST /enroll</button>
    </div>

    <div class="card">
      <h3>Grade</h3>
      <div class="row">
        <input id="gradeStudentId" placeholder="S001" />
        <input id="gradeCourseCode" placeholder="CS101" />
      </div>
      <input id="score" type="number" placeholder="88.5" />
      <button onclick="grade()">POST /grade</button>
    </div>

    <div class="card">
      <h3>Query</h3>
      <button onclick="listStudents()">GET /students</button>
      <button onclick="listCourses()">GET /courses</button>
      <input id="reportStudentId" placeholder="Student id for report/export" />
      <button onclick="studentReport()">GET /reports/student/{id}</button>
      <button class="secondary" onclick="openExport('student','csv')">Export Student CSV</button>
      <button class="secondary" onclick="openExport('student','pdf')">Export Student PDF</button>
      <input id="reportCourseCode" placeholder="Course code for report/export" />
      <button onclick="courseReport()">GET /reports/course/{code}</button>
      <button class="secondary" onclick="openExport('course','csv')">Export Course CSV</button>
      <button class="secondary" onclick="openExport('course','pdf')">Export Course PDF</button>
    </div>
  </div>

  <h3>Response</h3>
  <pre id="out">Ready.</pre>

  <script>
    function authHeaders() {
      const token = document.getElementById('token').value.trim();
      return token ? { 'Authorization': `Bearer ${token}` } : {};
    }

    async function request(path, method = 'GET', body = null) {
      const headers = { ...authHeaders() };
      if (body) headers['Content-Type'] = 'application/json';
      const res = await fetch(path, { method, headers, body: body ? JSON.stringify(body) : undefined });
      const text = await res.text();
      let data = text;
      try { data = JSON.parse(text); } catch (_) {}
      document.getElementById('out').textContent = JSON.stringify({ status: res.status, data }, null, 2);
      return { res, data };
    }

    async function login() {
      const username = document.getElementById('username').value;
      const password = document.getElementById('password').value;
      const { res, data } = await request('/login', 'POST', { username, password });
      if (res.ok && data.token) document.getElementById('token').value = data.token;
    }
    async function addStudent() {
      await request('/students', 'POST', {
        id: document.getElementById('studentId').value,
        name: document.getElementById('studentName').value
      });
    }
    async function addCourse() {
      await request('/courses', 'POST', {
        code: document.getElementById('courseCode').value,
        title: document.getElementById('courseTitle').value
      });
    }
    async function enroll() {
      await request('/enroll', 'POST', {
        student_id: document.getElementById('enrollStudentId').value,
        course_code: document.getElementById('enrollCourseCode').value
      });
    }
    async function grade() {
      await request('/grade', 'POST', {
        student_id: document.getElementById('gradeStudentId').value,
        course_code: document.getElementById('gradeCourseCode').value,
        score: Number(document.getElementById('score').value)
      });
    }
    async function listStudents() { await request('/students'); }
    async function listCourses() { await request('/courses'); }
    async function studentReport() {
      const id = document.getElementById('reportStudentId').value;
      await request(`/reports/student/${encodeURIComponent(id)}`);
    }
    async function courseReport() {
      const code = document.getElementById('reportCourseCode').value;
      await request(`/reports/course/${encodeURIComponent(code)}`);
    }
    function openExport(kind, ext) {
      const key = kind === 'student'
        ? document.getElementById('reportStudentId').value
        : document.getElementById('reportCourseCode').value;
      const token = document.getElementById('token').value.trim();
      if (!token) {
        document.getElementById('out').textContent = 'Please login first.';
        return;
      }
      const base = kind === 'student' ? '/reports/student' : '/reports/course';
      fetch(`${base}/${encodeURIComponent(key)}/${ext}`, { headers: { Authorization: `Bearer ${token}` } })
        .then(async (res) => {
          if (!res.ok) {
            const txt = await res.text();
            document.getElementById('out').textContent = txt;
            return;
          }
          const blob = await res.blob();
          const a = document.createElement('a');
          a.href = URL.createObjectURL(blob);
          a.download = `${kind}_${key}.${ext}`;
          a.click();
          URL.revokeObjectURL(a.href);
        });
    }
  </script>
</body>
</html>
"#,
    )
}

