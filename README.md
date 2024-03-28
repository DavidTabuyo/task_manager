Programa simple escrito en rust: EL programa se puede ejecutar con argumentos o sin argumentos, si se ejecuta con argumentos:
$cargo run
El programa te dará a eleigr entre varias opciones
0 -> Consultar tareas: Imprime por pantalla las tareas que haya en el fichero
1 -> Añadir tareas: Pide el nombre de la tarea que deseas hacer y la añade al fichero
2 -> Marcar como tarea hecha: Marca una tarea como hecha, la elimina del fichero y la añade a otro fichero con la fecha y hora de finalización
3 -> mostrar tareas hechas: Muestra las tareas que se encuentran como tareas finalizadas
Al ejecutar el programa con argumentos se ejecutará directamente una de las opciones.
ej: $cargo run 1 ejecutará Añadir tareas

El programa se puede usar como un alias en linux así se convierte en un comando que puede ser utilizado desde cualquier parte del sistema de forma instantánea
Añadir el path a los ficheros en el código.

Mejoras:
  - Añadir un máximo de 10 tareas hechas

---------------------------------------------------
Simple program written in Rust: The program can be executed with or without arguments. If executed with arguments:
$cargo run
The program will prompt you to choose from various options:
0 -> Check tasks: Prints the tasks stored in the file to the screen.
1 -> Add tasks: Prompts for the name of the task to be added and adds it to the file.
2 -> Mark task as done: Marks a task as completed, removes it from the file, and adds it to another file with the date and time of completion.
3 -> Show completed tasks: Displays tasks marked as completed.
When the program is executed with arguments, it will directly execute one of the options.
Example: $cargo run 1 will execute the "Add tasks" option.

The program can be used as an alias in Linux, turning it into a command that can be used from anywhere in the system instantly. Add the path to the files in the code.

Improvements:

Limit the number of completed tasks to 10.



