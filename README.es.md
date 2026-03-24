<div align="center">

# PointSav Monorepo
### *Código Fuente e Ingeniería para Infraestructura Digital Independiente*

[ **Documentation Wiki** ](https://github.com/pointsav/content-wiki-documentation) | [ **Design System** ](https://github.com/pointsav/pointsav-design-system) | [ **Main Profile** ](https://github.com/pointsav)

*Despliegue Operativo:* [ **Woodfine Management Corp.** ](https://github.com/woodfine)

[ 🇬🇧 Read this document in English ](./README.md)

</div>

---

> [!NOTE]
> **POSTURA OPERATIVA [MARZO 2026]**
> **Fase:** Iteración de Producción 2 | **Enfoque:** Tubería de Datos Soberana y Libros de Construcción Criptográficos | **Estado:** Despliegue Activo de Servicios Totebox.

### 🚀 El Sistema Operativo de Prioridad Digital
PointSav proporciona la ingeniería fundamental para asegurar la empresa moderna y democratizar la propiedad de los datos. Construimos entornos operativos Unikernel descentralizados que evitan las vulnerabilidades de las aplicaciones web heredadas y la infraestructura comercial de hiperescaladores.

Al erradicar por completo el sistema operativo Linux genérico y multiinquilino, nuestra arquitectura aísla las cargas de trabajo críticas —como los sistemas de archivos criptográficos y el enrutamiento de IA cognitiva— en Unikernels dedicados y seguros en memoria. Garantizamos que los libros de contabilidad operativos permanezcan matemáticamente sellados contra la extracción externa o amenazas cibernéticas sistémicas.

> [!WARNING]
> **DECLARACIÓN DE LÍMITE DE SEGURIDAD**
> Este repositorio contiene el código fuente de ingeniería Rust `no_std` y las definiciones del Gestor Basado en Capacidades. **No contiene claves criptográficas activas, cargas útiles de red ni datos financieros de clientes.**

### ⚙️ Vía 1: Infraestructura (El Libro de Construcción Criptográfico)
Enfoque: Ejecución en hardware (bare-metal), puentes de virtualización y bloqueos matemáticos.
| Directorio de Componentes | Objetivo de Ingeniería | Estado |
| :--- | :--- | :--- |
| [`os-infrastructure`](./os-infrastructure) | Nodos de Entrega de Borde | 🟢 `Arquitectura Verificada` |
| [`os-network-admin`](./os-network-admin) | Pasarelas de Comando y Enrutamiento | 🟡 `Ingeniería Activa` |
| [`system-security`](./system-security) | Gestor Basado en Capacidades seL4 (Rust) | 🟢 `Verificado` |

### 🧠 Vía 2: Orquestación Totebox (Cajas de Arena de Servicios Activos)
Enfoque: Procesamiento de datos, aislamiento de identidad y enrutamiento de inteligencia determinista.
| Directorio de Componentes | Motor de Carga Útil y Mitigación de Riesgos | Estado |
| :--- | :--- | :--- |
| [`service-email`](./service-email) | Pasarela de Ingestión (MSFT Graph Harvester y MIME Splinter). | 🟢 `Verificado` |
| [`service-people`](./service-people) | Destilación de Señales de Personal (Motor ACS Soberano). | 🟢 `Verificado` |
| [`service-slm`](./service-slm) | Esclusa de Aire Lingüística (Cognitive Forge - SmolLM2 135M). | 🟢 `Verificado` |
| [`service-content`](./service-content) | Síntesis de Conocimiento (Compilador de Contenido, Libros Verificados y Wikis Autorreparables). | 🟢 `Verificado` |
| [`service-search`](./service-search) | Búsqueda Soberana (Índice Invertido de Archivo Plano Leapfrog 2050). | 🟡 `Ingeniería Activa` |

### 🖥️ Vía 3: Workplace (El Escritorio Soberano)
Enfoque: Entornos de operador diseñados para producir salidas sin dependencias.
| Directorio de Componentes | Salida de Archivo Determinista e Interfaces | Estado |
| :--- | :--- | :--- |
| [`os-console`](./os-console) | Vehículo de Entrega del Operador (Servidor HTTP Local). | 🟡 `Ingeniería Activa` |
| [`app-console-*`](./) | Interfaces de Administración e Inspector (Contenido, Correo, Personal, Entrada). | 🟡 `Ingeniería Activa` |
| [`app-workplace-editor`](./app-workplace-editor) | Estándares legibles por máquina: Markdown, YAML, CSV. | 🟡 `Ingeniería Activa` |
| [`app-workplace-comm`](./app-workplace-comm) | Almacenamiento Maildir asíncrono y localizado. | 🟡 `Ingeniería Activa` |

---
*© 2026 PointSav Digital Systems™*
