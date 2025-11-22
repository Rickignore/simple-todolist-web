const api = "/api/todos";

// ----------------------------------------------------------------
// LOAD TODOS
// ----------------------------------------------------------------
async function loadTodos() {
    const res = await fetch(api);
    const data = await res.json();

    const list = document.getElementById("todo-list");
    list.innerHTML = "";

    data.forEach(todo => {
        const li = document.createElement("li");
        li.className = "todo-item";

        li.innerHTML = `
            <span class="todo-text ${todo.done ? "done" : ""}">
                ${todo.text}
            </span>
            <div class="todo-buttons">
                <button class="small done-btn" onclick="toggleDone('${todo.id}', ${todo.done})">
                    ${todo.done ? "Undo" : "Done"}
                </button>
                <button class="small delete-btn" onclick="deleteTodo('${todo.id}')">
                    Hapus
                </button>
            </div>
        `;

        list.appendChild(li);
    });
}

loadTodos();


// ----------------------------------------------------------------
// ADD TODO
// ----------------------------------------------------------------
async function addTodo() {
    const input = document.getElementById("todo-input");
    const text = input.value.trim();
    if (!text) return;

    await fetch(api, {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify({ text })
    });

    input.value = "";
    loadTodos();
}


// ----------------------------------------------------------------
// DELETE TODO
// ----------------------------------------------------------------
async function deleteTodo(id) {
    await fetch(`${api}/${id}`, { method: "DELETE" });
    loadTodos();
}


// ----------------------------------------------------------------
// TOGGLE DONE
// ----------------------------------------------------------------
async function toggleDone(id, current) {
    await fetch(`${api}/${id}`, {
        method: "PUT",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ done: !current })
    });

    loadTodos();
}