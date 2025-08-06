# Contribuir a csvmd

¡Gracias por tu interés en contribuir a csvmd! 🎉 Este documento proporciona pautas e información para ayudarte a comenzar.

## Tabla de Contenidos

- [Primeros Pasos](#primeros-pasos)
- [Entorno de Desarrollo](#entorno-de-desarrollo)
- [Arquitectura del Proyecto](#arquitectura-del-proyecto)
- [Flujo de Trabajo de Desarrollo](#flujo-de-trabajo-de-desarrollo)
- [Estilo y Calidad del Código](#estilo-y-calidad-del-código)
- [Pruebas](#pruebas)
- [Envío de Cambios](#envío-de-cambios)
- [Proceso de Lanzamiento](#proceso-de-lanzamiento)
- [Obtener Ayuda](#obtener-ayuda)

## Primeros Pasos

csvmd es una herramienta CLI de Rust que convierte archivos CSV a tablas Markdown. Está diseñada para velocidad y eficiencia, soportando tanto modo estándar como de transmisión para manejar archivos grandes.

### Prerrequisitos

- [Rust](https://www.rust-lang.org/tools/install) (se recomienda la última versión estable)
- Git
- Un editor de texto o IDE con soporte para Rust

### Configuración Rápida

1. **Hacer fork y clonar el repositorio:**
   ```bash
   git clone https://github.com/TU_USUARIO/csvmd.git
   cd csvmd
   ```

2. **Compilar el proyecto:**
   ```bash
   cargo build
   ```

3. **Ejecutar pruebas para asegurar que todo funciona:**
   ```bash
   cargo test
   ```

4. **Probar la herramienta CLI:**
   ```bash
   # Crear un CSV de muestra
   echo "Nombre,Edad\nJuan,25\nJana,30" > muestra.csv
   
   # Convertirlo a Markdown
   cargo run -- muestra.csv
   ```

## Entorno de Desarrollo

### Herramientas Recomendadas

- **Rust Analyzer**: Para soporte de IDE
- **cargo-watch**: Para compilación/pruebas continuas
  ```bash
  cargo install cargo-watch
  cargo watch -x test
  ```

### Comandos Esenciales

```bash
# Compilar el proyecto
cargo build

# Compilar versión optimizada de lanzamiento
cargo build --release

# Ejecutar la herramienta CLI
cargo run -- [OPCIONES] [ARCHIVO]

# Formatear código (SIEMPRE ejecutar antes de commit)
cargo fmt

# Verificar formato (asegurar que el código esté formateado correctamente)
cargo fmt --check

# Lint del código
cargo clippy

# Ejecutar todas las pruebas (unitarias + integración)
cargo test

# Ejecutar solo pruebas unitarias (en src/lib.rs)
cargo test --lib

# Ejecutar solo pruebas de integración
cargo test --test integration_tests

# Ejecutar prueba específica
cargo test test_csv_with_pipes

# Ejecutar con salida
cargo test -- --nocapture
```

**Importante**: Siempre ejecuta `cargo fmt` antes de hacer commits para asegurar formato consistente del código.

## Arquitectura del Proyecto

csvmd sigue un patrón estándar de Rust biblioteca + binario:

```
src/
├── lib.rs          # Biblioteca principal con lógica de conversión
├── main.rs         # Interfaz CLI usando clap
└── error.rs        # Tipos de error personalizados con thiserror

tests/
├── integration_tests.rs  # Pruebas de funcionalidad CLI completa
└── edge_cases.rs         # Casos límite y pruebas de condiciones de error
```

### Componentes Clave

- **Biblioteca Principal** (`src/lib.rs`): Contiene dos funciones principales:
  - `csv_to_markdown()`: Carga todo el CSV en memoria, adecuado para archivos pequeños
  - `csv_to_markdown_streaming()`: Enfoque de transmisión de dos pasadas para archivos grandes
  
- **Interfaz CLI** (`src/main.rs`): Usa clap para análisis de argumentos y maneja entrada/salida

- **Manejo de Errores** (`src/error.rs`): Tipos de error personalizados para análisis CSV, IO y errores de formato

### Decisiones de Diseño Clave

- Usa el crate csv con análisis flexible para manejar recuentos de columnas desiguales
- Escapa caracteres especiales de Markdown: `|` → `\|`, `\n` → `<br>`
- Pre-asigna capacidad de string basada en tamaño estimado de salida
- El modo de transmisión usa enfoque de dos pasadas para asegurar formato correcto de tabla

## Flujo de Trabajo de Desarrollo

### Hacer Cambios

1. **Crear una rama de característica:**
   ```bash
   git checkout -b feature/nombre-de-tu-caracteristica
   ```

2. **Hacer tus cambios** siguiendo los estándares de codificación

3. **Probar tus cambios:**
   ```bash
   # Ejecutar formato
   cargo fmt
   
   # Ejecutar linting
   cargo clippy
   
   # Ejecutar todas las pruebas
   cargo test
   ```

4. **Probar la CLI manualmente:**
   ```bash
   # Probar funcionalidad básica
   echo "Nombre,Edad\nJuan,25" | cargo run
   
   # Probar con archivos
   cargo run -- datos_prueba.csv
   
   # Probar modo de transmisión
   cargo run -- --stream archivo_grande.csv
   
   # Probar diferentes alineaciones
   cargo run -- --align center datos.csv
   ```

### Pautas de Cambios de Código

- Hacer modificaciones mínimas - cambiar tan pocas líneas como sea posible
- No eliminar/remover código que funciona a menos que sea absolutamente necesario
- Siempre validar que los cambios no rompan el comportamiento existente
- Actualizar documentación si está directamente relacionada con tus cambios

## Estilo y Calidad del Código

### Formato

- **Siempre ejecuta `cargo fmt` antes de hacer commit**
- Usa la configuración predeterminada de rustfmt
- El código debe pasar `cargo fmt --check` en CI

### Linting

- El código debe pasar `cargo clippy` sin advertencias
- Seguir convenciones de nomenclatura e idiomas de Rust
- Usar nombres significativos para variables y funciones

### Documentación

- Agregar docstrings para funciones y tipos públicos
- Incluir ejemplos en documentación donde sea útil
- Actualizar README.md si se agregan nuevas características o se cambia la interfaz CLI

## Pruebas

csvmd tiene una estrategia de pruebas integral:

### Pruebas Unitarias

Ubicadas en `src/lib.rs`, estas prueban la lógica de conversión principal:

```bash
cargo test --lib
```

La cobertura incluye:
- Conversión básica de CSV
- Casos límite (celdas vacías, caracteres especiales, Unicode)
- Opciones de alineación de encabezados
- Delimitadores personalizados
- Condiciones de error

### Pruebas de Integración

Ubicadas en `tests/integration_tests.rs`, estas prueban la funcionalidad CLI completa:

```bash
cargo test --test integration_tests
```

La cobertura incluye:
- Análisis de argumentos de línea de comandos
- Entrada/salida de archivos
- Manejo de stdin/stdout
- Manejo y reporte de errores
- Compatibilidad multiplataforma

### Pruebas de Casos Límite

Ubicadas en `tests/edge_cases.rs`, estas prueban entradas inusuales:

```bash
cargo test --test edge_cases
```

### Agregar Nuevas Pruebas

Al agregar características:

1. **Agregar pruebas unitarias** para lógica principal en `src/lib.rs`
2. **Agregar pruebas de integración** para comportamiento CLI en `tests/integration_tests.rs`
3. **Considerar casos límite** y agregar pruebas en `tests/edge_cases.rs`

Ejemplo de prueba unitaria:
```rust
#[test]
fn test_tu_caracteristica() {
    let csv_data = "Nombre,Edad\nJuan,25";
    let input = Cursor::new(csv_data);
    let config = Config::default();
    let result = csv_to_markdown(input, config).unwrap();
    
    let expected = "| Nombre | Edad |\n| --- | --- |\n| Juan | 25 |\n";
    assert_eq!(result, expected);
}
```

## Envío de Cambios

### Pautas de Pull Request

1. **Asegurar que todas las pruebas pasen:**
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy
   ```

2. **Escribir una descripción clara del PR:**
   - Explicar qué cambios hiciste y por qué
   - Referenciar cualquier issue relacionado
   - Incluir ejemplos si agregas nuevas características

3. **Mantener PRs enfocados:**
   - Una característica o corrección por PR
   - Evitar mezclar cambios no relacionados

4. **Actualizar documentación** si tus cambios afectan:
   - Interfaz CLI
   - API pública
   - Instrucciones de instalación o uso

### Mensajes de Commit

- Usar mensajes de commit claros y descriptivos
- Comenzar con un verbo en tiempo presente ("Add", "Fix", "Update")
- Referenciar issues cuando sea aplicable ("Fixes #123")

### Requisitos de CI

Tu PR debe pasar todas las verificaciones de CI:

- ✅ Las pruebas pasan en todas las plataformas (Linux, macOS, Windows)
- ✅ El código está formateado correctamente (`cargo fmt --check`)
- ✅ Sin advertencias de linting (`cargo clippy`)
- ✅ Se compila exitosamente en modo release

## Proceso de Lanzamiento

csvmd usa un proceso de lanzamiento automatizado:

1. **Etiquetado de Versión**: Los lanzamientos se activan empujando etiquetas en el formato `vX.Y.Z`
2. **Compilaciones Multiplataforma**: CI automáticamente compila para múltiples plataformas
3. **Firma de Código**: Los binarios de macOS son firmados y notarizados
4. **Publicación**: Los lanzamientos se publican tanto en GitHub releases como en crates.io

Los contribuyentes no necesitan preocuparse por los lanzamientos - los mantenedores manejan este proceso.

## Obtener Ayuda

### Documentación

- [README.md](README.md) - Instrucciones de uso y ejemplos
- [The Rust Book](https://doc.rust-lang.org/book/) - Aprender Rust
- [Documentación de Clap](https://docs.rs/clap/) - Análisis de argumentos CLI
- [Documentación del Crate CSV](https://docs.rs/csv/) - Análisis CSV

### Comunicación

- **Issues**: Usar issues de GitHub para reportes de bugs y solicitudes de características
- **Discussions**: Usar discussions de GitHub para preguntas y discusión general
- **Seguridad**: Para issues de seguridad, por favor seguir divulgación responsable

### Problemas Comunes

**Problemas de Compilación:**
```bash
# Limpiar y recompilar
cargo clean
cargo build
```

**Fallas de Pruebas:**
```bash
# Ejecutar prueba específica con salida
cargo test nombre_prueba -- --nocapture

# Ejecutar pruebas una a la vez
cargo test -- --test-threads=1
```

**Problemas de Formato:**
```bash
# Auto-corregir formato
cargo fmt

# Verificar qué sería cambiado
cargo fmt -- --check
```

---

¡Gracias por contribuir a csvmd! Tus contribuciones ayudan a hacer mejor la conversión de CSV-a-Markdown para todos. 🚀