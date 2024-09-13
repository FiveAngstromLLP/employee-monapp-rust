use rocket::{get, post, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::Instant;

#[derive(Serialize, Deserialize, Clone)]
struct Employee {
    name: String,
    code: String,
    start_time: Option<Instant>,
    elapsed_time: u64,
}

struct AppState {
    employees: Arc<Mutex<HashMap<String, Employee>>>,
}

#[post("/login", data = "<employee>")]
async fn login(employee: Json<Employee>, state: &State<AppState>) -> Json<bool> {
    let mut employees = state.employees.lock().unwrap();
    let employee_name = employee.name.clone();

    let new_employee = Employee {
        name: employee_name.clone(),
        code: employee.code.clone(),
        start_time: Some(Instant::now()),
        elapsed_time: 0,
    };

    employees.insert(employee_name, new_employee);
    Json(true)
}

#[post("/logout", data = "<employee>")]
async fn logout(employee: Json<Employee>, state: &State<AppState>) -> Json<bool> {
    let mut employees = state.employees.lock().unwrap();
    if let Some(emp) = employees.get_mut(&employee.name) {
        if let Some(start) = emp.start_time {
            emp.elapsed_time += start.elapsed().as_secs();
            emp.start_time = None;
        }
    }
    Json(true)
}

#[get("/elapsed_time/<name>")]
async fn get_elapsed_time(name: String, state: &State<AppState>) -> Json<u64> {
    let employees = state.employees.lock().unwrap();
    if let Some(emp) = employees.get(&name) {
        let mut elapsed = emp.elapsed_time;
        if let Some(start) = emp.start_time {
            elapsed += start.elapsed().as_secs();
        }
        return Json(elapsed);
    }
    Json(0)
}

#[launch]
fn rocket() -> _ {
    let app_state = AppState {
        employees: Arc::new(Mutex::new(HashMap::new())),
    };

    rocket::build()
        .manage(app_state)
        .mount("/", routes![login, logout, get_elapsed_time])
}