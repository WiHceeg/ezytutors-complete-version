function getCookie(name) {
    const cookieString = document.cookie;
    const cookies = cookieString.split(';');
    for (let i = 0; i < cookies.length; i++) {
        const cookie = cookies[i].trim();
        if (cookie.startsWith(name + '=')) {
            return cookie.substring(name.length + 1);
        }
    }
    return null;
}


const createCourseForm = document.getElementById('CreateCourse');

createCourseForm.addEventListener('submit', function (event) {
    event.preventDefault(); // 阻止浏览器默认动作（即不向服务器发送请求）

    // 创建空对象用来储存输入值
    const formDataJson = {};

    // 遍历所有name属性值非空的输入字段
    new FormData(this).forEach((value, key) => {
        if (key && value.trim()) {
            // 重点：针对数字字段特殊处理
            if (key === 'course_price') {
                formDataJson[key] = Number(value.trim()); // 字符串 → 数字
            } else {
                formDataJson[key] = value.trim();
            }
        }
    });


    console.log(formDataJson); // 打印出生成的json对象进行调试

    const tutor_id = getCookie('tutor_id');
    if (!tutor_id) {
        console.error('No tutor_id found in cookies');
        alert('tutor_id not found in cookies. Please log in.');
        return;
    }

    fetch(`/courses/new/${tutor_id}`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formDataJson)
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok ' + response.statusText);
            }
            return response.json();
        })
        .then(data => {
            if (confirm(`Insert course 「${data.course_name}」 success, Click Confirm to refresh the page and display.`)) {
                location.reload();   // 按了「确定」就刷新
            }
        })
        .catch(error => console.error('Error:', error));
});



const updateCourseForm = document.getElementById('UpdateCourse');

updateCourseForm.addEventListener('submit', function (event) {
    event.preventDefault(); // 阻止浏览器默认动作（即不向服务器发送请求）

    let course_id = null;
    // 创建空对象用来储存输入值
    const formDataJson = {};

    // 遍历所有name属性值非空的输入字段
    new FormData(this).forEach((value, key) => {
        if (key && value.trim()) {
            // 重点：针对数字字段特殊处理
            if (key === 'course_price') {
                formDataJson[key] = Number(value.trim()); // 字符串 → 数字
            } else if (key === 'course_id') {
                course_id = value.trim();
            } else {
                formDataJson[key] = value.trim();
            }
        }
    });


    console.log(formDataJson); // 打印出生成的json对象进行调试

    const tutor_id = getCookie('tutor_id');
    if (!tutor_id) {
        console.error('No tutor_id found in cookies');
        alert('tutor_id not found in cookies. Please log in.');
        return;
    }

    fetch(`/courses/${tutor_id}/${course_id}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formDataJson)
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok ' + response.statusText);
            }
            return response.json();
        })
        .then(data => {
            if (confirm(`Update course 「${data.course_name}」 success, Click Confirm to refresh the page and display.`)) {
                location.reload();   // 按了「确定」就刷新
            }
        })
        .catch(error => console.error('Error:', error));
});



const deleteCourseForm = document.getElementById('DeleteCourse');

deleteCourseForm.addEventListener('submit', function (event) {
    event.preventDefault(); // 阻止浏览器默认动作（即不向服务器发送请求）

    let course_id = document.getElementById('delete_course_id').value;
    console.log(course_id);

    const tutor_id = getCookie('tutor_id');
    if (!tutor_id) {
        console.error('No tutor_id found in cookies');
        alert('tutor_id not found in cookies. Please log in.');
        return;
    }

    fetch(`/courses/delete/${tutor_id}/${course_id}`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
    })
        .then(response => {
            if (!response.ok) {
                throw new Error('Network response was not ok ' + response.statusText);
            }
            return response;
        })
        .then(data => {
            if (confirm(`Delete course success, Click Confirm to refresh the page and display.`)) {
                location.reload();   // 按了「确定」就刷新
            }
        })
        .catch(error => console.error('Error:', error));
});
