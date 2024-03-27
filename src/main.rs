use std::{env, io};


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
fn save_changes(){
    
}

fn main() {
    //get data from txt
    let data:Vec<Task> = get_tasks();
    //analize arguments
    let args: Vec<String> = env::args().collect();
    if args.len() == 1{
        //we execute the program normal
        loop{
            println!("(0) -> consultar tareas");
            println!("(1) -> Añadir tareas");
            println!("(2) -> Marcar como tarea hecha");
            let mut imput:String = String::new();
            io::stdin().read_line(&mut imput).expect("Error al leer la entrada");
            let imput= imput.trim();
            match imput {
                "0" => {
                    check_tasks()
                }
                "1" => {
                    add_tasks()
                }
                "2" => {
                    mark_as_done()
                }
                _ => {
                    println!("Entrada invalida, por favor, introduce otro número")
                }
            }
        }
    }else if args.len()==2{
        //arguments correct, we can execute only one part
        let chosen = args.get(1).map_or("".to_string(),|s| s.to_string()).as_str();
        match chosen {
            "0" => {
                check_tasks()
            }
            "1" => {
                add_tasks()
            }
            "2" => {
                mark_as_done()
            }
            _ => {
                println!("Entrada invalida, por favor, introduce otro número")
            }
        }

    }else{
        //number of arguments are incorrect
        print!("El número de argumentos es incorrecto . . . ")
    }
    //save changes
    save_changes();
}
