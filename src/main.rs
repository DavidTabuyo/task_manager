use std::{env, fs::{File, OpenOptions}, io::{self, BufRead, BufReader, Write}};

#[derive(Clone)]
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

fn get_tasks() -> io::Result<Vec<Task>>{
    let mut data: Vec<Task> = vec![];
    let file = File::open("/home/david/.task_manager/task_manager/src/to_do_data.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let task_name = line?;
        let task = Task::new(task_name, false, String::new()); // Puedes ajustar los valores según sea necesario
        data.push(task);
    }
    Ok(data)
}

fn check_tasks(data: &mut Vec<Task>){
    for index in 0..data.len() {
        println!("({}) {}",index,data[index].task_name);
    }
}
fn write_task(new_task:Task)-> io::Result<()>{
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("/home/david/.task_manager/task_manager/src/to_do_data.txt")?;
    file.write_all(new_task.task_name.as_bytes())?;
    Ok(())
}

fn add_tasks(data: &mut Vec<Task>){
    println!("Escribe la tarea nueva");
    let mut imput:String = String::new();
    io::stdin().read_line(&mut imput).expect("Error al leer la entrada");
    let new_task=Task::new(imput,false,"".to_string());
    let cloned_task = new_task.clone();
    data.push(new_task);
    write_task(cloned_task).unwrap();
    println!("Tarea escrita correctamente")
}


fn mark_as_done_task(index:usize){
    //mark as done task, remove from txt and add to other txt
}

fn mark_as_done(data: &mut Vec<Task>){
    println!("Elige la tarea que has acabado");
    check_tasks(data);
    let mut imput:String = String::new();
    io::stdin().read_line(&mut imput).expect("Error al leer la entrada");
    let imput:usize= imput.trim().parse().expect("Entrada no válida");
    if imput < data.len(){
        mark_as_done_task(imput);
        data.remove(imput);
    }else{
        print!("Entrada no válida");
    }
}

fn main() {
    //get data from txt
    let mut data:Vec<Task> = get_tasks().unwrap();
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
                    check_tasks(&mut data);
                    break;
                }
                "1" => {
                    add_tasks(&mut data);
                    break;
                }
                "2" => {
                    mark_as_done(&mut data);
                    break;
                }
                _ => {
                    println!("Entrada invalida, por favor, introduce otro número")
                }
            }
        }
    }else if args.len()==2{
        //arguments correct, we can execute only one part
        let chosen = args.get(1).map_or("".to_string(), |s| s.to_string());
        let chosen_str=chosen.as_str();
        match chosen_str {
            "0" => {
                check_tasks(&mut data)
            }
            "1" => {
                add_tasks(&mut data)
            }
            "2" => {
                mark_as_done(&mut data)
            }
            _ => {
                println!("Entrada invalida, por favor, introduce otro número")
            }
        }

    }else{
        //number of arguments are incorrect
        print!("El número de argumentos es incorrecto . . . ")
    }
}
