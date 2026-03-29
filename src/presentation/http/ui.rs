use axum::response::Html;

pub async fn ui_page() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<html lang="th">
<head>
  <meta charset="UTF-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  <title>BackToSchool Management Dashbard</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap" rel="stylesheet">
  <style>
    body { font-family: 'Inter', sans-serif; }
    .nav-btn.active { background-color: #1e293b; font-weight: 600; color: #fff; }
    /* Toast styles */
    #toast-container { position: fixed; top: 1rem; right: 1rem; z-index: 50; display: flex; flex-direction: column; gap: 0.5rem; }
    .toast { padding: 1rem; border-radius: 0.5rem; color: white; opacity: 0; transition: opacity 0.3s; display: flex; align-items: center; justify-content: space-between; min-width: 250px;}
    .toast.show { opacity: 1; }
    .toast.success { background-color: #10b981; }
    .toast.error { background-color: #ef4444; }
    .loader { border: 2px solid #f3f3f3; border-top: 2px solid #3498db; border-radius: 50%; width: 14px; height: 14px; animation: spin 1s linear infinite; display: inline-block;}
    @keyframes spin { 0% { transform: rotate(0deg); } 100% { transform: rotate(360deg); } }
  </style>
</head>
<body class="bg-slate-50 text-slate-900">
  <div id="toast-container"></div>

  <div class="min-h-screen flex flex-col lg:flex-row">
    <!-- Sidebar -->
    <aside class="w-full lg:w-64 bg-slate-900 text-slate-300 flex-shrink-0">
      <div class="p-6 border-b border-slate-800">
        <h1 class="text-xl font-bold text-white tracking-wide flex items-center gap-2">
            <svg class="w-6 h-6 text-blue-500" fill="currentColor" viewBox="0 0 20 20"><path d="M10.394 2.08a1 1 0 00-.788 0l-7 3a1 1 0 000 1.84L5.25 8.051a.999.999 0 01.356-.257l4-1.714a1 1 0 11.788 1.838L7.667 9.088l1.94.831a1 1 0 00.787 0l7-3a1 1 0 000-1.838l-7-3zM3.31 9.397L5 10.12v4.102a8.969 8.969 0 00-1.05-.174 1 1 0 01-.89-.89 11.115 11.115 0 01.25-3.762zM9.3 16.573A9.026 9.026 0 007 14.935v-3.957l1.818.78a3 3 0 002.364 0l5.508-2.361a11.026 11.026 0 01.25 3.762 1 1 0 01-.89.89 8.968 8.968 0 00-5.35 2.524 1 1 0 01-1.4 0zM6 18a1 1 0 001-1v-2.065a8.935 8.935 0 00-2-.712V17a1 1 0 001 1z"></path></svg>
            BackToSchool
        </h1>
        <div class="mt-4 flex items-center justify-between text-xs">
            <span>Status: <span id="healthStatus" class="text-gray-400">Checking...</span></span>
        </div>
      </div>
      <nav class="p-4 space-y-1">
        <button onclick="showPanel('overview')" class="nav-btn active w-full flex items-center gap-3 rounded-md px-3 py-2.5 text-sm hover:bg-slate-800 hover:text-white transition-colors"><span>Dashboard</span></button>
        <button onclick="showPanel('students')" class="nav-btn w-full flex items-center gap-3 rounded-md px-3 py-2.5 text-sm hover:bg-slate-800 hover:text-white transition-colors"><span>Students</span></button>
        <button onclick="showPanel('courses')" class="nav-btn w-full flex items-center gap-3 rounded-md px-3 py-2.5 text-sm hover:bg-slate-800 hover:text-white transition-colors"><span>Courses</span></button>
        <button onclick="showPanel('enrollment')" class="nav-btn w-full flex items-center gap-3 rounded-md px-3 py-2.5 text-sm hover:bg-slate-800 hover:text-white transition-colors"><span>Enrollments</span></button>
        <button onclick="showPanel('reports')" class="nav-btn w-full flex items-center gap-3 rounded-md px-3 py-2.5 text-sm hover:bg-slate-800 hover:text-white transition-colors"><span>Reports & Export</span></button>
        <button onclick="showPanel('auth')" class="nav-btn w-full flex items-center gap-3 rounded-md px-3 py-2.5 text-sm hover:bg-slate-800 hover:text-white transition-colors mt-8"><span>Authentication</span></button>
      </nav>
      <div class="p-4 mt-auto border-t border-slate-800 text-xs text-slate-500">
        Logged in as: <span id="currentUserRole" class="text-blue-400">Guest</span>
      </div>
    </aside>

    <!-- Main Content -->
    <main class="flex-1 p-6 lg:p-10 lg:max-w-7xl mx-auto w-full">
      <header class="mb-8 flex justify-between items-center bg-white p-4 rounded-xl shadow-sm border border-slate-200">
        <div>
            <h2 id="panelTitle" class="text-2xl font-bold bg-clip-text text-transparent bg-gradient-to-r from-blue-600 to-indigo-600">Overview</h2>
            <p id="panelDesc" class="text-sm text-slate-500 mt-1">Manage school data simply and efficiently</p>
        </div>
        <div id="authIndicator" class="px-3 py-1 bg-red-100 text-red-700 text-xs font-semibold rounded-full border border-red-200">
            Unauthenticated
        </div>
      </header>

      <!-- Overview Panel -->
      <section id="panel-overview" class="panel block space-y-6">
        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          <div class="bg-white rounded-xl shadow-sm border border-slate-200 p-6 flex flex-col justify-between hover:shadow-md transition-shadow">
            <div class="flex justify-between items-start"><p class="text-slate-500 font-medium text-sm">Total Students</p><span class="p-2 bg-blue-50 rounded-lg text-blue-600"><svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z"></path></svg></span></div>
            <p id="statStudents" class="text-3xl font-bold mt-4">-</p>
          </div>
          <div class="bg-white rounded-xl shadow-sm border border-slate-200 p-6 flex flex-col justify-between hover:shadow-md transition-shadow">
            <div class="flex justify-between items-start"><p class="text-slate-500 font-medium text-sm">Total Courses</p><span class="p-2 bg-indigo-50 rounded-lg text-indigo-600"><svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"></path></svg></span></div>
            <p id="statCourses" class="text-3xl font-bold mt-4">-</p>
          </div>
        </div>
        <div class="bg-blue-50 p-6 rounded-xl border border-blue-100 text-blue-800">
            <h3 class="font-semibold text-lg mb-2">Welcome to BackToSchool</h3>
            <p class="text-sm opacity-90">Select a section from the sidebar to start managing students and courses. Make sure you are logged in to perform actions.</p>
        </div>
      </section>

      <!-- Auth Panel -->
      <section id="panel-auth" class="panel hidden">
        <div class="max-w-md mx-auto bg-white p-8 rounded-2xl shadow-sm border border-slate-200">
            <h3 class="text-xl font-bold mb-6 text-center">Login to Account</h3>
            <form onsubmit="handleLogin(event)" class="space-y-4">
                <div>
                    <label class="block text-sm font-medium text-slate-700 mb-1">Role / Username</label>
                    <select id="loginUsername" class="w-full rounded-lg border-slate-300 border px-4 py-2.5 focus:border-blue-500 focus:ring-blue-500 bg-slate-50 text-slate-900">
                        <option value="teacher">Teacher (Add/Edit Students, Enroll, Grade)</option>
                        <option value="admin">Admin (Add/Edit Courses + Everything)</option>
                    </select>
                </div>
                <div>
                    <label class="block text-sm font-medium text-slate-700 mb-1">Password</label>
                    <input id="loginPassword" type="password" class="w-full rounded-lg border-slate-300 border px-4 py-2.5 focus:border-blue-500 focus:ring-blue-500 bg-slate-50" placeholder="••••••••" value="teacher123">
                </div>
                <button type="submit" class="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2.5 rounded-lg transition-colors shadow-sm mt-4">Login</button>
            </form>
            <div class="mt-6 pt-6 border-t border-slate-100">
                <p class="text-sm text-slate-500 mb-2 font-medium">Current Token (Local Storage)</p>
                <div class="flex gap-2">
                    <input id="tokenInput" readonly class="flex-1 rounded-lg border-slate-200 border px-3 py-2 text-xs bg-slate-50 font-mono text-slate-500" placeholder="No token">
                    <button onclick="logout()" class="text-xs bg-slate-200 hover:bg-slate-300 px-3 py-2 rounded-lg font-medium transition-colors">Logout</button>
                </div>
            </div>
        </div>
      </section>

      <!-- Students Panel -->
      <section id="panel-students" class="panel hidden space-y-6">
        <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
            <div class="flex justify-between items-center mb-4">
                <h3 class="text-lg font-bold">Add Student</h3>
            </div>
            <form onsubmit="addStudent(event)" class="flex gap-4 items-end">
                <div class="flex-1">
                    <label class="block text-xs font-semibold text-slate-500 uppercase tracking-wide mb-1">Student ID</label>
                    <input id="newStudentId" required placeholder="e.g. S001" class="w-full rounded-lg border-slate-300 border px-3 py-2 focus:border-blue-500 focus:ring-1 focus:ring-blue-500">
                </div>
                <div class="flex-1">
                    <label class="block text-xs font-semibold text-slate-500 uppercase tracking-wide mb-1">Full Name</label>
                    <input id="newStudentName" required placeholder="e.g. Somchai" class="w-full rounded-lg border-slate-300 border px-3 py-2 focus:border-blue-500 focus:ring-1 focus:ring-blue-500">
                </div>
                <button type="submit" class="bg-blue-600 hover:bg-blue-700 text-white font-medium px-5 py-2.5 rounded-lg transition-colors whitespace-nowrap">Save Student</button>
            </form>
        </div>

        <div class="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
            <div class="p-4 border-b border-slate-200 bg-slate-50 flex justify-between items-center">
                <h3 class="font-bold text-slate-700">Student Directory</h3>
                <button onclick="loadStudents()" class="text-xs bg-white border border-slate-200 hover:bg-slate-50 px-3 py-1.5 rounded-md font-medium text-slate-600 shadow-sm flex items-center gap-1"><svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg> Refresh</button>
            </div>
            <div class="overflow-x-auto">
                <table class="w-full text-left text-sm text-slate-600">
                    <thead class="bg-slate-50/50 text-slate-500 border-b border-slate-200 text-xs font-semibold uppercase tracking-wider">
                        <tr><th class="px-6 py-4">ID</th><th class="px-6 py-4">Name</th><th class="px-6 py-4 text-right">Actions</th></tr>
                    </thead>
                    <tbody id="studentsTableBody" class="divide-y divide-slate-100">
                        <tr><td colspan="3" class="px-6 py-8 text-center text-slate-400">Loading data...</td></tr>
                    </tbody>
                </table>
            </div>
        </div>
      </section>

      <!-- Courses Panel -->
      <section id="panel-courses" class="panel hidden space-y-6">
        <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
            <form onsubmit="addCourse(event)" class="flex gap-4 items-end">
                <div class="flex-1">
                    <label class="block text-xs font-semibold text-slate-500 uppercase tracking-wide mb-1">Course Code</label>
                    <input id="newCourseCode" required placeholder="e.g. CS101" class="w-full rounded-lg border-slate-300 border px-3 py-2 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500">
                </div>
                <div class="flex-1">
                    <label class="block text-xs font-semibold text-slate-500 uppercase tracking-wide mb-1">Course Title</label>
                    <input id="newCourseTitle" required placeholder="e.g. Intro to Rust" class="w-full rounded-lg border-slate-300 border px-3 py-2 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500">
                </div>
                <button type="submit" class="bg-indigo-600 hover:bg-indigo-700 text-white font-medium px-5 py-2.5 rounded-lg transition-colors whitespace-nowrap">Save Course</button>
            </form>
        </div>

        <div class="bg-white rounded-xl shadow-sm border border-slate-200 overflow-hidden">
            <div class="p-4 border-b border-slate-200 bg-slate-50 flex justify-between items-center">
                <h3 class="font-bold text-slate-700">Course Catalog</h3>
                <button onclick="loadCourses()" class="text-xs bg-white border border-slate-200 hover:bg-slate-50 px-3 py-1.5 rounded-md font-medium text-slate-600 shadow-sm flex items-center gap-1"><svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/></svg> Refresh</button>
            </div>
            <div class="overflow-x-auto">
                <table class="w-full text-left text-sm text-slate-600">
                    <thead class="bg-slate-50/50 text-slate-500 border-b border-slate-200 text-xs font-semibold uppercase tracking-wider">
                        <tr><th class="px-6 py-4">Code</th><th class="px-6 py-4">Title</th><th class="px-6 py-4 text-right">Actions</th></tr>
                    </thead>
                    <tbody id="coursesTableBody" class="divide-y divide-slate-100">
                        <tr><td colspan="3" class="px-6 py-8 text-center text-slate-400">Loading data...</td></tr>
                    </tbody>
                </table>
            </div>
        </div>
      </section>

      <!-- Enrollment Panel -->
      <section id="panel-enrollment" class="panel hidden space-y-6">
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <!-- Enroll Form -->
            <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
                <h3 class="text-lg font-bold mb-4 text-slate-800 border-b pb-2">New Enrollment</h3>
                <form onsubmit="submitEnrollment(event)" class="space-y-4">
                    <div>
                        <label class="block text-sm font-medium text-slate-700 mb-1">Select Student</label>
                        <select id="enrollStudentSelect" required class="w-full rounded-lg border-slate-300 border px-3 py-2 bg-slate-50 focus:border-blue-500 focus:ring-1 focus:ring-blue-500"><option value="">-- Choose Student --</option></select>
                    </div>
                    <div>
                        <label class="block text-sm font-medium text-slate-700 mb-1">Select Course</label>
                        <select id="enrollCourseSelect" required class="w-full rounded-lg border-slate-300 border px-3 py-2 bg-slate-50 focus:border-blue-500 focus:ring-1 focus:ring-blue-500"><option value="">-- Choose Course --</option></select>
                    </div>
                    <button type="submit" class="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2.5 rounded-lg transition-colors mt-2">Enroll Student</button>
                </form>
            </div>

            <!-- Grade Form -->
            <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200 border-t-4 border-t-amber-500">
                <h3 class="text-lg font-bold mb-4 text-slate-800 border-b pb-2">Cut Grade</h3>
                <form onsubmit="submitGrade(event)" class="space-y-4">
                    <p class="text-xs text-slate-500 mb-2">First select a student and course, then input score (0-100).</p>
                    <div class="grid grid-cols-2 gap-3">
                        <div>
                            <label class="block text-xs font-semibold text-slate-500 uppercase tracking-wide mb-1">Student</label>
                            <select id="gradeStudentSelect" required class="w-full rounded-lg border-slate-300 border px-2 py-2 text-sm bg-slate-50 focus:border-amber-500 focus:ring-1 focus:ring-amber-500"><option value="">-- Student --</option></select>
                        </div>
                        <div>
                            <label class="block text-xs font-semibold text-slate-500 uppercase tracking-wide mb-1">Course</label>
                            <select id="gradeCourseSelect" required class="w-full rounded-lg border-slate-300 border px-2 py-2 text-sm bg-slate-50 focus:border-amber-500 focus:ring-1 focus:ring-amber-500"><option value="">-- Course --</option></select>
                        </div>
                    </div>
                    <div>
                        <label class="block text-xs font-semibold text-slate-500 uppercase tracking-wide mb-1">Score Result</label>
                        <input id="gradeScore" type="number" step="0.1" min="0" max="100" required placeholder="e.g. 85.5" class="w-full rounded-lg border-slate-300 border px-3 py-2 focus:border-amber-500 focus:ring-1 focus:ring-amber-500 text-lg font-bold text-amber-700">
                    </div>
                    <button type="submit" class="w-full bg-amber-500 hover:bg-amber-600 text-white font-medium py-2.5 rounded-lg transition-colors shadow-sm">Process Grade</button>
                </form>
            </div>
        </div>
      </section>

      <!-- Reports & Export -->
      <section id="panel-reports" class="panel hidden space-y-6">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
            <!-- Student Report -->
            <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
                <div class="w-12 h-12 rounded-full bg-blue-100 text-blue-600 flex items-center justify-center mb-4">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path></svg>
                </div>
                <h3 class="text-xl font-bold mb-1">Student Transcript</h3>
                <p class="text-sm text-slate-500 mb-5">Export full academic record for a specific student</p>
                <div class="space-y-3">
                    <select id="reportStudentSelect" class="w-full rounded-lg border-slate-300 border px-3 py-2 bg-slate-50 focus:border-blue-500 focus:ring-1 focus:ring-blue-500"><option value="">-- Select Student --</option></select>
                    <div class="flex gap-2">
                        <button onclick="downloadReport('student', 'csv')" class="flex-1 bg-white border border-slate-300 text-slate-700 hover:bg-slate-50 py-2 rounded-lg text-sm font-medium transition-colors shadow-sm inline-flex justify-center items-center gap-2"><svg class="w-4 h-4 text-green-600" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg> CSV</button>
                        <button onclick="downloadReport('student', 'pdf')" class="flex-1 bg-red-50 border border-red-200 text-red-700 hover:bg-red-100 py-2 rounded-lg text-sm font-medium transition-colors shadow-sm inline-flex justify-center items-center gap-2"><svg class="w-4 h-4 text-red-600" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4zm2 6a1 1 0 011-1h6a1 1 0 110 2H7a1 1 0 01-1-1zm1 3a1 1 0 100 2h6a1 1 0 100-2H7z" clip-rule="evenodd"></path></svg> PDF</button>
                    </div>
                </div>
            </div>

            <!-- Course Report -->
            <div class="bg-white p-6 rounded-xl shadow-sm border border-slate-200">
                <div class="w-12 h-12 rounded-full bg-indigo-100 text-indigo-600 flex items-center justify-center mb-4">
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10"></path></svg>
                </div>
                <h3 class="text-xl font-bold mb-1">Course Roster</h3>
                <p class="text-sm text-slate-500 mb-5">Export enrolled students and grades for a course</p>
                <div class="space-y-3">
                    <select id="reportCourseSelect" class="w-full rounded-lg border-slate-300 border px-3 py-2 bg-slate-50 focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"><option value="">-- Select Course --</option></select>
                    <div class="flex gap-2">
                        <button onclick="downloadReport('course', 'csv')" class="flex-1 bg-white border border-slate-300 text-slate-700 hover:bg-slate-50 py-2 rounded-lg text-sm font-medium transition-colors shadow-sm inline-flex justify-center items-center gap-2"><svg class="w-4 h-4 text-green-600" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"></path></svg> CSV</button>
                        <button onclick="downloadReport('course', 'pdf')" class="flex-1 bg-red-50 border border-red-200 text-red-700 hover:bg-red-100 py-2 rounded-lg text-sm font-medium transition-colors shadow-sm inline-flex justify-center items-center gap-2"><svg class="w-4 h-4 text-red-600" fill="currentColor" viewBox="0 0 20 20"><path fill-rule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4zm2 6a1 1 0 011-1h6a1 1 0 110 2H7a1 1 0 01-1-1zm1 3a1 1 0 100 2h6a1 1 0 100-2H7z" clip-rule="evenodd"></path></svg> PDF</button>
                    </div>
                </div>
            </div>
        </div>
      </section>

    </main>
  </div>

  <script>
    // --- State & Core ---
    const API = '';
    let globalMem = { students: [], courses: [] };

    function getToken() { return localStorage.getItem('bts_token') || ''; }
    function setToken(token, role) { 
        if(token) { localStorage.setItem('bts_token', token); localStorage.setItem('bts_role', role); }
        else { localStorage.removeItem('bts_token'); localStorage.removeItem('bts_role'); }
        updateAuthUI();
    }
    
    function getHeaders() {
        const t = getToken();
        return t ? { 'Authorization': `Bearer ${t}`, 'Content-Type': 'application/json' } : { 'Content-Type': 'application/json' };
    }

    // --- Toast Notifications ---
    function showToast(msg, type = 'success') {
        const c = document.getElementById('toast-container');
        const t = document.createElement('div');
        t.className = `toast ${type}`;
        t.innerHTML = `<span>${msg}</span><button onclick="this.parentElement.remove()" class="ml-4 opacity-70 hover:opacity-100">&times;</button>`;
        c.appendChild(t);
        setTimeout(() => t.classList.add('show'), 10);
        setTimeout(() => { t.classList.remove('show'); setTimeout(()=>t.remove(), 300); }, 4000);
    }

    // --- Fetch Wrapper ---
    async function apiCall(endpoint, method = 'GET', body = null) {
        try {
            const res = await fetch(API + endpoint, {
                method, headers: getHeaders(),
                body: body ? JSON.stringify(body) : undefined
            });
            const text = await res.text();
            let data = null;
            try { data = JSON.parse(text); } catch(e) {}
            if(!res.ok) throw new Error(data?.error || text || `Error ${res.status}`);
            return data;
        } catch (err) {
            showToast(err.message, 'error');
            throw err;
        }
    }

    // --- UI Navigation ---
    const panelMeta = {
        'overview': { title: 'Overview Dashboard', desc: 'Summary metrics and system health', onShow: loadOverview },
        'auth': { title: 'Authentication', desc: 'Login as Teacher or Admin', onShow: () => { document.getElementById('loginPassword').value = document.getElementById('loginUsername').value+'123'; } },
        'students': { title: 'Manage Students', desc: 'Add, edit, or remove students from the directory', onShow: loadStudents },
        'courses': { title: 'Manage Courses', desc: 'Administer course catalog subjects', onShow: loadCourses },
        'enrollment': { title: 'Enrollment & Grades', desc: 'Register students to courses and assign scores', onShow: loadDropdowns },
        'reports': { title: 'Data Exports', desc: 'Generate and download CSV / PDF files', onShow: loadDropdowns }
    };

    function showPanel(name) {
        document.querySelectorAll('.panel').forEach(p => p.classList.add('hidden'));
        document.getElementById(`panel-${name}`).classList.remove('hidden');
        document.querySelectorAll('.nav-btn').forEach(btn => btn.classList.remove('active'));
        document.querySelector(`.nav-btn[onclick="showPanel('${name}')"]`).classList.add('active');
        
        const m = panelMeta[name];
        document.getElementById('panelTitle').innerText = m.title;
        document.getElementById('panelDesc').innerText = m.desc;
        if(m.onShow) m.onShow();
    }

    function updateAuthUI() {
        const t = getToken();
        document.getElementById('tokenInput').value = t;
        const role = localStorage.getItem('bts_role');
        document.getElementById('currentUserRole').innerText = t ? role : 'Guest';
        const ind = document.getElementById('authIndicator');
        if(t){ ind.className = 'px-3 py-1 bg-green-100 text-green-700 text-xs font-semibold rounded-full border border-green-200 shadow-sm'; ind.innerText = 'Authenticated'; }
        else { ind.className = 'px-3 py-1 bg-red-100 text-red-700 text-xs font-semibold rounded-full border border-red-200 shadow-sm'; ind.innerText = 'Unauthenticated'; }
    }

    // --- Handlers ---
    async function handleLogin(e) {
        e.preventDefault();
        const username = document.getElementById('loginUsername').value;
        const password = document.getElementById('loginPassword').value;
        const btn = e.target.querySelector('button');
        btn.innerHTML = 'Connecting...'; btn.disabled = true;
        try {
            const data = await apiCall('/login', 'POST', {username, password});
            setToken(data.token, data.role);
            showToast('Login successful!');
            setTimeout(() => showPanel('overview'), 500);
        } finally {
            btn.innerHTML = 'Login'; btn.disabled = false;
        }
    }

    function logout() { setToken(null); showToast('Logged out', 'success'); }

    async function loadOverview() {
        try {
            fetch('/health').then(r=>r.json()).then(d => { document.getElementById('healthStatus').innerText = d.message; document.getElementById('healthStatus').className = "text-green-400 font-semibold"; }).catch(()=>null);
            if(getToken()){
                const [s, c] = await Promise.all([apiCall('/students').catch(()=>[]), apiCall('/courses').catch(()=>[])]);
                globalMem.students = s; globalMem.courses = c;
                document.getElementById('statStudents').innerText = s.length || 0;
                document.getElementById('statCourses').innerText = c.length || 0;
            }
        } catch(e) {}
    }

    function populateSelects(selectIds, dataArray, valKey, textKey) {
        const optionsHTML = '<option value="">-- Choose Option --</option>' + 
                            dataArray.map(item => `<option value="${item[valKey]}">${item[valKey]} - ${item[textKey]}</option>`).join('');
        selectIds.forEach(id => {
            const el = document.getElementById(id);
            if(el) el.innerHTML = optionsHTML;
        });
    }

    async function loadDropdowns() {
        if(!getToken()) return;
        if(globalMem.students.length === 0) await loadOverview();
        populateSelects(['enrollStudentSelect', 'gradeStudentSelect', 'reportStudentSelect'], globalMem.students, 'id', 'name');
        populateSelects(['enrollCourseSelect', 'gradeCourseSelect', 'reportCourseSelect'], globalMem.courses, 'code', 'title');
    }

    // -- Students Logic --
    async function loadStudents() {
        if(!getToken()) { document.getElementById('studentsTableBody').innerHTML = `<tr><td colspan="3" class="px-6 py-4 text-center text-red-500">Please login first</td></tr>`; return;}
        document.getElementById('studentsTableBody').innerHTML = `<tr><td colspan="3" class="px-6 py-4 text-center">Loading...</td></tr>`;
        try {
            const data = await apiCall('/students');
            globalMem.students = data;
            if(data.length === 0) {
                document.getElementById('studentsTableBody').innerHTML = `<tr><td colspan="3" class="px-6 py-4 text-center text-slate-500">No students found</td></tr>`; return;
            }
            document.getElementById('studentsTableBody').innerHTML = data.map(s => `
                <tr class="hover:bg-slate-50"><td class="px-6 py-4 font-mono text-xs">${s.id}</td><td class="px-6 py-4 font-medium">${s.name}</td>
                <td class="px-6 py-4 text-right space-x-2">
                    <button onclick="editStudent('${s.id}', '${s.name}')" class="text-blue-600 hover:underline">Edit</button>
                    <button onclick="deleteStudent('${s.id}')" class="text-red-600 hover:underline">Delete</button>
                </td></tr>`).join('');
        } catch(e) {}
    }

    async function addStudent(e) {
        e.preventDefault();
        const id = document.getElementById('newStudentId').value;
        const name = document.getElementById('newStudentName').value;
        const btn = e.target.querySelector('button'); btn.disabled=true;
        try { await apiCall('/students', 'POST', {id, name}); showToast('Student added successfully!'); e.target.reset(); loadStudents(); } finally { btn.disabled=false; }
    }
    
    async function editStudent(id, oldName) {
        const newName = prompt(`Edit name for ${id}:`, oldName);
        if(!newName || newName === oldName) return;
        try { await apiCall(`/students/${id}`, 'PUT', {name: newName}); showToast('Updated successfully!'); loadStudents(); } catch(e){}
    }

    async function deleteStudent(id) {
        if(!confirm(`Are you sure you want to delete student ${id}?`)) return;
        try { await apiCall(`/students/${id}`, 'DELETE'); showToast('Deleted successfully!'); loadStudents(); } catch(e){}
    }

    // -- Courses Logic --
    async function loadCourses() {
        if(!getToken()) { document.getElementById('coursesTableBody').innerHTML = `<tr><td colspan="3" class="px-6 py-4 text-center text-red-500">Please login first</td></tr>`; return;}
        document.getElementById('coursesTableBody').innerHTML = `<tr><td colspan="3" class="px-6 py-4 text-center">Loading...</td></tr>`;
        try {
            const data = await apiCall('/courses');
            globalMem.courses = data;
            if(data.length === 0) {
                document.getElementById('coursesTableBody').innerHTML = `<tr><td colspan="3" class="px-6 py-4 text-center text-slate-500">No courses found</td></tr>`; return;
            }
            document.getElementById('coursesTableBody').innerHTML = data.map(c => `
                <tr class="hover:bg-slate-50"><td class="px-6 py-4 font-mono text-xs">${c.code}</td><td class="px-6 py-4 font-medium">${c.title}</td>
                <td class="px-6 py-4 text-right space-x-2">
                    <button onclick="editCourse('${c.code}', '${c.title}')" class="text-blue-600 hover:underline">Edit</button>
                    <button onclick="deleteCourse('${c.code}')" class="text-red-600 hover:underline">Delete</button>
                </td></tr>`).join('');
        } catch(e) {}
    }

    async function addCourse(e) {
        e.preventDefault();
        const code = document.getElementById('newCourseCode').value;
        const title = document.getElementById('newCourseTitle').value;
        const btn = e.target.querySelector('button'); btn.disabled=true;
        try { await apiCall('/courses', 'POST', {code, title}); showToast('Course added successfully!'); e.target.reset(); loadCourses(); } finally { btn.disabled=false; }
    }
    
    async function editCourse(code, oldTitle) {
        const newTitle = prompt(`Edit title for course ${code}:`, oldTitle);
        if(!newTitle || newTitle === oldTitle) return;
        try { await apiCall(`/courses/${code}`, 'PUT', {title: newTitle}); showToast('Updated successfully!'); loadCourses(); } catch(e){}
    }

    async function deleteCourse(code) {
        if(!confirm(`Are you sure you want to delete course ${code}?`)) return;
        try { await apiCall(`/courses/${code}`, 'DELETE'); showToast('Deleted successfully!'); loadCourses(); } catch(e){}
    }

    // -- Enroll & Grade --
    async function submitEnrollment(e) {
        e.preventDefault();
        const sid = document.getElementById('enrollStudentSelect').value;
        const cid = document.getElementById('enrollCourseSelect').value;
        if(!sid || !cid) return showToast('Please select both', 'error');
        const btn = e.target.querySelector('button'); btn.disabled=true;
        try { await apiCall('/enroll', 'POST', {student_id: sid, course_code: cid}); showToast('Student Enrolled!'); e.target.reset(); } finally { btn.disabled=false; }
    }

    async function submitGrade(e) {
        e.preventDefault();
        const sid = document.getElementById('gradeStudentSelect').value;
        const cid = document.getElementById('gradeCourseSelect').value;
        const score = parseFloat(document.getElementById('gradeScore').value);
        if(!sid || !cid) return showToast('Please select both', 'error');
        const btn = e.target.querySelector('button'); btn.disabled=true;
        try { await apiCall('/grade', 'POST', {student_id: sid, course_code: cid, score}); showToast('Grade updated!'); e.target.reset(); } finally { btn.disabled=false; }
    }

    // -- Reports --
    function downloadReport(type, ext) {
        const val = document.getElementById(type === 'student' ? 'reportStudentSelect' : 'reportCourseSelect').value;
        if(!val) return showToast('Please select an option first', 'error');
        const url = `/reports/${type}/${encodeURIComponent(val)}/${ext}`;
        fetch(url, { headers: { 'Authorization': `Bearer ${getToken()}` } }).then(async res => {
            if(!res.ok) { showToast(await res.text(), 'error'); return; }
            const blob = await res.blob();
            const a = document.createElement('a');
            a.href = URL.createObjectURL(blob);
            a.download = `${type}_${val}.${ext}`;
            a.click();
            URL.revokeObjectURL(a.href);
            showToast('Download started');
        }).catch(err => showToast('Failed to download', 'error'));
    }

    // Init
    updateAuthUI();
    showPanel('overview');
    
    document.getElementById('loginUsername').addEventListener('change', e => {
        document.getElementById('loginPassword').value = e.target.value + '123';
    });
  </script>
</body>
</html>
"#
    )
}
