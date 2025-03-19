# to_lcd

Convierte los dígitos de un texto a formato LCD usando caracteres Unicode.

## Descripción

Este programa en Rust recibe uno o varios argumentos de texto por línea de comandos. Internamente, combina todos los argumentos en una única cadena y luego busca los caracteres que son dígitos decimales (del 0 al 9). Cada dígito encontrado es reemplazado por su correspondiente carácter Unicode que simula un display LCD.

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

## Cómo usar

### Requisitos

### Compilación

1. Clona este repositorio (o copia el código fuente en un directorio local).
2. Abre una terminal en el directorio del proyecto (`~/rust/to_lcd`).
3. Ejecuta el siguiente comando para compilar el programa:

   ```bash
   cargo build --release
   ```

   El ejecutable se generará en el directorio `target/release`.

### Ejecución

Puedes ejecutar el programa pasando el texto que deseas convertir como argumentos en la línea de comandos.

```bash
target/release/to_lcd "Hola mundo 123" "y más texto 45"
```

Esto producirá la siguiente salida:

```
Hola mundo 🯱🯲🯳 y más texto 🯴🯵
```

Puedes pasar múltiples argumentos, y el programa los combinará con un espacio entre ellos antes de realizar la conversión:

```bash
target/release/to_lcd "El número es" 987
```

Salida:

```
El número es 🯹🯸🯷
```

Si no se proporciona ningún argumento, el programa mostrará un mensaje de uso:

```bash
target/release/to_lcd
```

Salida:

```
Uso: target/release/to_lcd <alfanum> <alfanum> ..
```

## Ejemplos

Aquí tienes algunos ejemplos de cómo puedes usar `to_lcd`:

- Convertir un número de teléfono:

  ```bash
  target/release/to_lcd "Mi número es 01115551234"
  ```

  Salida:

  ```
  Mi número es 🯰🯱🯱🯱🯵🯵🯵🯱🯲🯳🯴
  ```

- Mostrar un código:

  ```bash
  target/release/to_lcd "Código: 9991"
  ```

  Salida:

  ```
  Código: 🯹🯹🯹🯱
  ```

- Combinar varios argumentos:

  ```bash
  target/release/to_lcd "Resultado:" 100 "puntos"
  ```

  Salida:

  ```
  Resultado: 🯱🯰🯰 puntos
  ```

## Notas

- El programa solo reemplaza los caracteres numéricos del 0 al 9. Otros caracteres se mantienen sin cambios.
- Asegúrate de que la terminal o el visor de texto que utilices sea compatible con los caracteres Unicode utilizados para la representación LCD.

¡Espero que esto te sea útil! Si tienes alguna otra pregunta, no dudes en consultarme.
```

Puedes guardar este texto en un archivo llamado `README.md` en la raíz de tu proyecto. ¡Se verá muy bien en GitHub!
