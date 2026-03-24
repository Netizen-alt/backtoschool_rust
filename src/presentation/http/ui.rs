use axum::response::Html;

pub async fn ui_page() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html lang="th">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>BackToSchool Dashboard</title>
  <script src="https://cdn.tailwindcss.com"></script>
</head>
<body class="bg-slate-100 text-slate-900">
  <div class="min-h-screen lg:flex">
    <aside class="w-full border-b border-slate-200 bg-slate-900 text-slate-100 lg:w-72 lg:border-b-0 lg:border-r">
      <div class="p-5">
        <h1 class="text-2xl font-bold">BackToSchool</h1>
        <p class="mt-1 text-sm text-slate-300">Test Dashboard</p>
      </div>
      <nav class="space-y-1 px-3 pb-5">
        <button onclick="showPanel('overview')" class="nav-btn w-full rounded-lg px-3 py-2 text-left text-sm hover:bg-slate-800">Overview</button>
        <button onclick="showPanel('auth')" class="nav-btn w-full rounded-lg px-3 py-2 text-left text-sm hover:bg-slate-800">Auth</button>
        <button onclick="showPanel('students')" class="nav-btn w-full rounded-lg px-3 py-2 text-left text-sm hover:bg-slate-800">Students</button>
        <button onclick="showPanel('courses')" class="nav-btn w-full rounded-lg px-3 py-2 text-left text-sm hover:bg-slate-800">Courses</button>
        <button onclick="showPanel('enrollment')" class="nav-btn w-full rounded-lg px-3 py-2 text-left text-sm hover:bg-slate-800">Enrollment & Grade</button>
        <button onclick="showPanel('reports')" class="nav-btn w-full rounded-lg px-3 py-2 text-left text-sm hover:bg-slate-800">Reports & Export</button>
      </nav>
    </aside>

    <main class="flex-1 p-4 sm:p-6 lg:p-8">
      <header class="mb-5">
        <h2 class="text-2xl font-semibold">Dashboard</h2>
        <p class="text-sm text-slate-600">แยกเมนูให้ใช้งานเป็นส่วน ๆ เหมือน dashboard</p>
      </header>

      <section id="panel-overview" class="panel">
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 xl:grid-cols-4">
          <div class="rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
            <p class="text-sm text-slate-500">Health</p>
            <p id="healthStatus" class="mt-1 text-lg font-semibold">-</p>
          </div>
          <div class="rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
            <p class="text-sm text-slate-500">Students</p>
            <p id="studentCount" class="mt-1 text-lg font-semibold">-</p>
          </div>
          <div class="rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
            <p class="text-sm text-slate-500">Courses</p>
            <p id="courseCount" class="mt-1 text-lg font-semibold">-</p>
          </div>
          <div class="rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
            <p class="text-sm text-slate-500">Token</p>
            <p id="tokenState" class="mt-1 text-lg font-semibold">Not set</p>
          </div>
        </div>
        <button onclick="refreshOverview()" class="mt-4 rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700">Refresh Overview</button>
      </section>

      <section id="panel-auth" class="panel hidden rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
        <h3 class="mb-3 text-lg font-semibold">Auth</h3>
        <div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
          <select id="username" class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm">
            <option value="teacher">teacher</option>
            <option value="admin">admin</option>
          </select>
          <input id="password" type="password" value="teacher123" class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
        </div>
        <button onclick="login()" class="mt-2 rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700">Login</button>
        <input id="token" placeholder="Bearer token" class="mt-3 w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
      </section>

      <section id="panel-students" class="panel hidden rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
        <h3 class="mb-3 text-lg font-semibold">Students</h3>
        <div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
          <input id="studentId" placeholder="S001" class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
          <input id="studentName" placeholder="Somchai" class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
        </div>
        <div class="mt-3 flex gap-2">
          <button onclick="addStudent()" class="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700">Add Student</button>
          <button onclick="listStudents()" class="rounded-md bg-slate-700 px-4 py-2 text-sm font-medium text-white hover:bg-slate-800">List Students</button>
        </div>
      </section>

      <section id="panel-courses" class="panel hidden rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
        <h3 class="mb-3 text-lg font-semibold">Courses</h3>
        <div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
          <input id="courseCode" placeholder="CS101" class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
          <input id="courseTitle" placeholder="Intro to Rust" class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
        </div>
        <div class="mt-3 flex gap-2">
          <button onclick="addCourse()" class="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700">Add Course (admin)</button>
          <button onclick="listCourses()" class="rounded-md bg-slate-700 px-4 py-2 text-sm font-medium text-white hover:bg-slate-800">List Courses</button>
        </div>
      </section>

      <section id="panel-enrollment" class="panel hidden rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
        <h3 class="mb-3 text-lg font-semibold">Enrollment & Grade</h3>
        <div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
          <input id="enrollStudentId" placeholder="Student ID (S001)" class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
          <input id="enrollCourseCode" placeholder="Course Code (CS101)" class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
        </div>
        <button onclick="enroll()" class="mt-2 rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700">Enroll</button>
        <div class="mt-4 grid grid-cols-1 gap-2 sm:grid-cols-3">
          <input id="gradeStudentId" placeholder="S001" class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
          <input id="gradeCourseCode" placeholder="CS101" class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
          <input id="score" type="number" placeholder="88.5" class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
        </div>
        <button onclick="grade()" class="mt-2 rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700">Update Grade</button>
      </section>

      <section id="panel-reports" class="panel hidden rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
        <h3 class="mb-3 text-lg font-semibold">Reports & Export</h3>
        <div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
          <input id="reportStudentId" placeholder="Student id for report/export" class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
          <input id="reportCourseCode" placeholder="Course code for report/export" class="w-full rounded-md border border-slate-300 px-3 py-2 text-sm" />
        </div>
        <div class="mt-3 grid grid-cols-1 gap-2 md:grid-cols-2 xl:grid-cols-3">
          <button onclick="studentReport()" class="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700">Student Report</button>
          <button onclick="openExport('student','csv')" class="rounded-md bg-slate-700 px-4 py-2 text-sm font-medium text-white hover:bg-slate-800">Student CSV</button>
          <button onclick="openExport('student','pdf')" class="rounded-md bg-slate-700 px-4 py-2 text-sm font-medium text-white hover:bg-slate-800">Student PDF</button>
          <button onclick="courseReport()" class="rounded-md bg-blue-600 px-4 py-2 text-sm font-medium text-white hover:bg-blue-700">Course Report</button>
          <button onclick="openExport('course','csv')" class="rounded-md bg-slate-700 px-4 py-2 text-sm font-medium text-white hover:bg-slate-800">Course CSV</button>
          <button onclick="openExport('course','pdf')" class="rounded-md bg-slate-700 px-4 py-2 text-sm font-medium text-white hover:bg-slate-800">Course PDF</button>
        </div>
      </section>

      <section class="mt-6">
        <h3 class="mb-2 text-lg font-semibold">Response</h3>
        <pre id="out" class="min-h-44 overflow-auto rounded-xl bg-slate-900 p-4 text-sm text-slate-100">Ready.</pre>
      </section>
    </main>
  </div>

  <script>
    function setOutput(payload) {
      document.getElementById('out').textContent = typeof payload === 'string'
        ? payload
        : JSON.stringify(payload, null, 2);
    }

    function showPanel(name) {
      const panels = document.querySelectorAll('.panel');
      panels.forEach((el) => el.classList.add('hidden'));
      document.getElementById(`panel-${name}`).classList.remove('hidden');
      const nav = document.querySelectorAll('.nav-btn');
      nav.forEach((el) => el.classList.remove('bg-slate-800', 'font-semibold'));
      const idx = ['overview','auth','students','courses','enrollment','reports'].indexOf(name);
      if (idx >= 0) nav[idx].classList.add('bg-slate-800', 'font-semibold');
    }

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
      setOutput({ status: res.status, data });
      return { res, data };
    }

    async function login() {
      const username = document.getElementById('username').value;
      const password = document.getElementById('password').value;
      const { res, data } = await request('/login', 'POST', { username, password });
      if (res.ok && data.token) {
        document.getElementById('token').value = data.token;
        document.getElementById('tokenState').textContent = 'Set';
      }
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
        setOutput('Please login first.');
        return;
      }
      const base = kind === 'student' ? '/reports/student' : '/reports/course';
      fetch(`${base}/${encodeURIComponent(key)}/${ext}`, { headers: { Authorization: `Bearer ${token}` } })
        .then(async (res) => {
          if (!res.ok) {
            const txt = await res.text();
            setOutput(txt);
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

    async function refreshOverview() {
      const health = await fetch('/health').then(r => r.json()).catch(() => ({ message: 'down' }));
      document.getElementById('healthStatus').textContent = health.message || 'unknown';
      document.getElementById('tokenState').textContent = document.getElementById('token').value.trim() ? 'Set' : 'Not set';

      const headers = authHeaders();
      const students = await fetch('/students', { headers }).then(async (r) => r.ok ? r.json() : []).catch(() => []);
      const courses = await fetch('/courses', { headers }).then(async (r) => r.ok ? r.json() : []).catch(() => []);
      document.getElementById('studentCount').textContent = Array.isArray(students) ? students.length : '-';
      document.getElementById('courseCount').textContent = Array.isArray(courses) ? courses.length : '-';
    }

    showPanel('overview');
    refreshOverview();
  </script>
</body>
</html>
"#,
    )
}

