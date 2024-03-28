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
