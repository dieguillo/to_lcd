# to_lcd

Convierte dígitos numéricos a formato LCD usando caracteres Unicode.


## Descripción

Este programa recibe uno o varios argumentos de texto por línea de comandos.

Internamente, combina todos los argumentos en una única cadena y luego busca y reemplaza cada dígito por su correspondiente carácter unicode de display LCD de 7 segmentos.


### Los caracteres Unicode utilizados para la conversión son:

- 0: 🯰
- 1: 🯱
- 2: 🯲
- 3: 🯳
- 4: 🯴
- 5: 🯵
- 6: 🯶
- 7: 🯷
- 8: 🯸
- 9: 🯹


## Cómo usarlo

- to_lcd 123
- to_lcd Ruta40
- to_lcd 27 + 3 = 30
- to_lcd $(date)
- to_lcd Código: 26283


## Notas

- El programa solo reemplaza los caracteres numéricos del 0 al 9. Otros caracteres se mantienen sin cambios.
- Asegúrate de que la terminal o el visor de texto que utilices sea compatible con los caracteres Unicode utilizados para la representación LCD.
Puedes guardar este texto en un archivo llamado `README.md` en la raíz de tu proyecto. ¡Se verá muy bien en GitHub!
