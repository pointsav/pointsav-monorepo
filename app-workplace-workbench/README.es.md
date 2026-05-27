# Workplace Workbench

Interfaz de escritorio nativa para el banco de trabajo de desarrollo privategit de PointSav.
Abre el servidor HTTP en ejecución local en una ventana de aplicación nativa de macOS.

**Plataforma:** macOS 10.13 High Sierra y versiones posteriores  
**Tecnología:** Tauri v1.7 (shell WebView — sin lógica de frontend integrada)  
**Licencia:** Apache-2.0

## Qué es esto

`app-workplace-workbench` es una interfaz Tauri WebView. Carga el servidor HTTP
`app-privategit-workbench` que se ejecuta en un puerto localhost configurable
(predeterminado: 3000) dentro de una ventana de aplicación nativa de macOS.

El servidor HTTP del banco de trabajo funciona como un proceso independiente.
Esta aplicación no inicia, detiene ni gestiona ese proceso; solo proporciona
una ventana nativa para acceder a él.
