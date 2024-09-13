use leptos::*;
use serde::Serialize;

#[derive(Serialize)]
struct Employee {
    name: String,
    code: String,
}

#[component]
fn EmployeeApp(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, String::new());
    let (code, set_code) = create_signal(cx, String::new());
    let (elapsed_time, set_elapsed_time) = create_signal(cx, 0);

    let login = move |_| {
        let employee = Employee {
            name: name.get().to_string(),
            code: code.get().to_string(),
        };
        spawn_local(async move {
            let _ = leptos::post::<_, _, bool>("http://localhost:8000/login", &employee).await;
        });
    };

    let logout = move |_| {
        let employee = Employee {
            name: name.get().to_string(),
            code: code.get().to_string(),
        };
        spawn_local(async move {
            let _ = leptos::post::<_, _, bool>("http://localhost:8000/logout", &employee).await;
        });
    };

    view! { cx,
        <div>
            <h1>"Employee Monitoring"</h1>
            <div>
                <input type="text" placeholder="Employee Name" value=name.get() on:input=move |e| set_name(e.value) />
                <input type="text" placeholder="Employee Code" value=code.get() on:input=move |e| set_code(e.value) />
                <button on:click=login>"Login"</button>
                <button on:click=logout>"Logout"</button>
                <div>
                    <h2>"Elapsed Time: "{elapsed_time.get()}" seconds"</h2>
                </div>
            </div>
        </div>
    }
}