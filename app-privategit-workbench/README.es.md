# app-privategit-workbench

Entorno de desarrollo en navegador para Totebox Orchestration.

Sirve un IDE de tres columnas (árbol de archivos / visor / editor) a través
de un punto de acceso HTTP local. El servicio de escritura garantiza escrituras
atómicas, detección de conflictos por mtime, lista de extensiones permitidas y
contención de raíz. Alojado por `os-privategit`.

## Arquitectura

- `src/main.rs` — servidor HTTP Rust/axum (API GET/PUT de archivos, puerto 9210)
- `src/assets/index.html` — aplicación de página única (árbol / visor / editor)
- `config.toml` — declaraciones de raíces escribibles y dirección de enlace

## Licencia

Bajo la Licencia Apache, Versión 2.0. Consulte [LICENSE](LICENSE).
