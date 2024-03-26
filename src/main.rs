use std::io;

struct Task{
    task_name: String,
    due: bool,
    time: String,
}
impl Task {
    fn new (task_name:String, due:bool,time:String) -> Task{
        Task{
            task_name,
            due,
            time,
        }
    }
}
// let my_task = Task = Tak::new(ss,ss,ss)

fn get_tasks() -> Vec<Task>{
    
    return Task;
}

fn check_tasks(){

}
fn add_tasks(){

}
fn mark_as_done(){

}

fn main() {
    //analyze doc searching tasks


    loop{
        println!("(0) -> consultar tareas");
        println!("(1) -> Añadir tareas");
        println!("(2) -> Marcar como tarea hecha");
        let mut imput:String = String::new();
        io::stdin().read_line(&mut imput).expect("Error al leer la entrada");
        let imput= imput.trim();
        match imput {
            "0" | "1" | "2" => {
                break;
            }
            _ => {
                println!("Entrada invalida, por favor, introduce otro número")
            }
        }
    }


}
