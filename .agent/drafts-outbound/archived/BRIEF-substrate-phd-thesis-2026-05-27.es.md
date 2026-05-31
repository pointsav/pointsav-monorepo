---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: n/a
target_path: n/a
target_filename: BRIEF-substrate-phd-thesis-2026-05-27.es.md
audience: academic
bcsc_class: no-disclosure-implication
language_protocol: PROSE-RESEARCH
authored: 2026-05-28T00:00:00Z
authored_by: task-project-system (session phase-1c-b, panorama pass)
authored_with: claude-sonnet-4-6
references:
  - clones/project-system/.agent/briefs/BRIEF-substrate-phd-thesis-2026-05-27.md (documento fuente)
  - conventions/bcsc-disclosure-posture.md
  - conventions/language-protocol-substrate.md
  - system-core v1.0.0 (62 pruebas unitarias)
  - system-ledger v1.0.0 (47 pruebas de integración)
  - moonshot-toolkit v0.2.0 (30 pruebas)
notes_for_editor: |
  Panorama estratégico del BRIEF de tesis doctoral en inglés. No es una traducción:
  es una adaptación para lectores académicos hispanohablantes del mismo contenido
  intelectual. Profundidad panorámica: 3–5 oraciones por sección principal.

  Actualizaciones con respecto al BRIEF original (2026-05-27):
  - system-core v0.2.0 → v1.0.0 (62 pruebas; commit c2ae1e9, 2026-05-27)
  - system-ledger v0.2.1 → v1.0.0 (47 pruebas; commit c2ae1e9, 2026-05-27)
  - moonshot-toolkit v0.1.3 → v0.2.0 (30 pruebas; commit 34a1111, 2026-05-27);
    la función build ahora invoca el compilador cruzado real (Fase 1C.a completa).
  - La Fase 1C.b está completa: kernel.elf de seL4 para qemu-arm-virt construido.
  - La Fase 1C.c (arranque QEMU) sigue bloqueada pendiente del cargador elfloader.

  Postura BCSC: pruebas formales de seL4, código Rust comprometido, mediciones de
  referencia = HECHOS (sin calificativos). Despliegues de producción AArch64, estado
  Activo de os-*, paridad moonshot-kernel = PLANIFICADO/PREVISTO.

  Término prohibido: «soberano» en uso descriptivo. Usar «fiable» o «bajo control
  del cliente». No realizar comparaciones competitivas nominales.

  English-language source: BRIEF-substrate-phd-thesis-2026-05-27.md
  Bilingual pair: Yes — this .es.md is the companion to the English BRIEF.
---

# Composición de Sistemas Fiables a partir de Primitivas Verificadas: Una Arquitectura de Sustrato para Libros Mayores de Capacidades bajo Control del Cliente en una Pila de Sistema Operativo de Doble Base

**Woodfine Management Corp. — Vancouver, Columbia Británica, Canadá**
Autor de correspondencia: jmwoodfine@gmail.com

**Palabras clave:** sistemas de capacidades, registros de transparencia, seL4, NetBSD, Veriexec, libro mayor WORM, transferencia de titularidad, sistemas fiables, construcciones reproducibles

**Clasificación ACM CCS:** D.4.6 Seguridad y Protección · D.4.1 Gestión de Procesos · C.0 General · D.2.11 Arquitecturas de Software

---

## Resumen

Ningún sistema operativo en producción en 2026 hace visible el estado de capacidades ante un registro de transparencia ni lo consulta antes de atender una invocación, mientras admite simultáneamente una transferencia de titularidad atómica y anclada en el libro mayor. Este trabajo presenta una arquitectura de sustrato que compone tres primitivas maduras e independientes —tipos de capacidad del micronúcleo seL4, registros Merkle de transparencia conformes con RFC 9162 v2, y puntos de control con firma múltiple C2SP signed-note— en un sistema cuyo estado de ejecución es la materialización determinista de un libro mayor de capacidades bajo raíz del cliente. Un diseño de doble base combina una base nativa seL4 v15.0.0 sobre hardware AArch64 con una base de compatibilidad NetBSD que despliega el arranque de imagen verificada Veriexec y la construcción reproducible sin red `build.sh`, permitiendo que los mismos binarios de tiempo de ejecución os-* se ejecuten en cualquiera de las dos bases mediante una envoltura Rust delgada seleccionada por indicadores de característica de Cargo. La arquitectura está evaluada mediante una implementación funcional en Rust: `system-core` v1.0.0 (62 pruebas unitarias, conforme con RFC 9162), `system-ledger` v1.0.0 (47 pruebas de integración, 10 referencias Criterion), y `moonshot-toolkit` v0.2.0 (30 pruebas, orquestación de construcción reproducible con dirección de contenido).

---

## 1. Introducción

### 1.1 El Problema de Investigación

Toda pila de identidad digital nacional actual —Estonia X-Road, la Cartera EUDI de la UE, la e-ID suiza, Aadhaar de India— está enraizada en el Estado. Toda arquitectura de atestación de hiperscaler ancla la confianza en las claves del proveedor: AWS Nitro Enclaves acredita el aislamiento *ante Amazon*, en metal de Amazon; Apple Private Cloud Compute es la arquitectura publicada más rigurosa, aunque su silicio no está a la venta. Los vendedores de sistemas operativos en tiempo real propietarios (Green Hills INTEGRITY, Wind River VxWorks, BlackBerry QNX) no pueden publicar registros mediados por el núcleo porque el código fuente es propietario. Los certificados Common Criteria EAL están vinculados a un proveedor y a un objetivo de evaluación específicos; transferir un despliegue certificado a un nuevo operador invalida el certificado.

La consecuencia estructural es una brecha en el espacio de diseño: **la raíz criptográfica bajo control del cliente no tiene precedente a escala alguna**. Ningún sistema en producción agrupa `(fuente + pruebas de verificación + grafo de capacidades + libro mayor de auditoría + claves de firma)` bajo una única raíz de registro de transparencia anclada en el cliente, con una ceremonia de transferencia de titularidad simultáneamente atómica, anclada en el libro mayor e independiente del proveedor.

### 1.2 Alcance y Contribuciones

El trabajo realiza tres contribuciones. En primer lugar, especifica una arquitectura de *Sustrato de Libro Mayor de Capacidades* en la que el estado de capacidades del sustrato se define como un registro Merkle WORM (escritura única, lectura múltiple); cada invocación, concesión, revocación y extensión temporal de capacidad emite una entrada firmada a un registro bajo raíz del cliente cuyo apex es la clave de firma del cliente. En segundo lugar, introduce un diseño de *doble base*: una base nativa seL4 para despliegues de alta seguridad regulada sobre hardware AArch64, y una base de compatibilidad NetBSD para hardware de propósito general donde seL4 no puede alcanzar el metal; una envoltura Rust delgada seleccionada en tiempo de compilación presenta a todos los binarios os-* una interfaz de invocación de capacidades uniforme (`CapabilityInvoker`). En tercer lugar, presenta una *ceremonia de cofirma de apex N+3+* derivada de la semántica de firma múltiple de C2SP signed-note que hace que la transferencia de titularidad de un despliegue completo sea atómica, auditable y definitiva.

---

## 2. Antecedentes y Trabajos Relacionados

### 2.1 Sistemas de Capacidades del Núcleo

**seL4** (Klein et al. 2009) es el único núcleo de sistema operativo en producción cuyas propiedades de seguridad han sido verificadas formalmente mediante pruebas matemáticas comprobadas por máquina. La estructura de árbol de derivación de capacidades (CDT) del núcleo es el control central de acceso: todo recurso se nombra mediante un token de capacidad infalsificable que codifica el tipo de recurso y las operaciones permitidas; no existe autoridad ambiental, ni usuario raíz, ni vía de anulación de capacidades. **CHERIoT** extiende la ISA para que las capacidades se apliquen a nivel de hardware por palabra; se compone ortogonalmente con seL4. **Capsicum** (Watson et al. 2010) es el modelo de capacidades más cercano en un sistema operativo de propósito general —descriptores de archivo FreeBSD como capacidades infalsificables; modo capacidad— pero no produce un registro de transparencia ni admite extensión temporal ni ceremonia de transferencia. La brecha común: el estado de capacidades no se publica ante un auditor accesible de forma externa.

### 2.2 Registros de Transparencia

**Certificate Transparency v2** [rfc-9162] (RFC 9162, 2022) estandarizó la construcción del árbol Merkle con separación de dominio explícita: los hashes de hoja se calculan como `SHA-256(0x00 || datos_hoja)` y los hashes internos como `SHA-256(0x01 || izquierda || derecha)`, previniendo ataques de segunda preimagen. **C2SP signed-note** [c2sp-signed-note] es un formato de transferencia para puntos de control de registros Merkle que soporta múltiples firmas Ed25519 sobre el mismo cuerpo; la propiedad de firma múltiple es explotada directamente por la ceremonia de cofirma de apex: el punto de control de traspaso a la altura N+2 lleva las firmas del apex saliente y del entrante en un único registro C2SP. **Sigstore Rekor v2** [sigstore-rekor-v2] usa el formato C2SP tlog-tiles [c2sp-tlog-tiles] para su registro de transparencia en producción.

### 2.3 La Brecha Compositiva

La brecha es compositiva, no primitiva. Cada una de las primitivas —seL4, RFC 9162, C2SP signed-note, Ed25519— es madura, desplegada de forma independiente y bien analizada. Ningún sistema en 2026 las compone de modo que: (a) el núcleo consulte el registro antes de atender una invocación de capacidad; (b) la extensión temporal de una capacidad requiera tanto una firma criptográfica como una prueba de inclusión Merkle en el libro mayor; (c) la transferencia de titularidad sea un evento atómico del libro mayor mediante semántica de firma múltiple; y (d) el despliegue pueda reconstituirse a partir de una semilla impresa en papel en cualquier hardware que arranque la base de compatibilidad.

---

## 3. La Arquitectura del Sustrato de Libro Mayor de Capacidades

### 3.1 El Diseño de Doble Base

El sustrato sitúa dos bases bajo todo binario de tiempo de ejecución os-*. La base nativa combina seL4 v15.0.0 con `moonshot-kernel` (planificado, Rust no_std, AArch64 primero) y está destinada a despliegues regulados donde la verificación formal es significativa. La base de compatibilidad combina NetBSD con arranque de imagen verificada Veriexec y reproducibilidad fuera de red con `build.sh`, y alcanza hardware de propósito general donde seL4 no puede alcanzar el metal. Linux no es una base del sustrato: permanece como alternativa de nivel comunitario no respaldada, fuera de la cadena de confianza. La misma arquitectura permite al operador ejecutar un Archivo Totebox en un portátil arrendado hoy (compat NetBSD) y en un electrodoméstico verificado en producción mañana (nativo seL4), con el mismo libro mayor de capacidades, la misma clave de firma de apex y el mismo historial de auditoría.

### 3.2 El Sistema de Tipos de Capacidades (`system-core` v1.0.0)

El tipo fundamental del sustrato es `Capability`, implementado en `system-core/src/lib.rs`. Los cinco variantes de `CapabilityType` se corresponden directamente con los tipos de objetos del núcleo seL4. La novedad estructural reside en el triple `expiry_t` / `witness_pubkey` / `ledger_anchor`: las capacidades estándar de seL4 no tienen dimensión temporal y no se publican en ningún registro; los tres campos adicionales introducen extensión temporal mediante delegación de testigo, enlace al libro mayor en el momento de creación, e identidad con dirección de contenido mediante SHA-256. `Capability::hash()` es SHA-256 sobre la codificación serde-JSON del cuerpo, determinista entre ejecuciones; cambiar cualquier campo —`expiry_t`, `ledger_anchor`, `rights` o `witness_pubkey`— cambia el hash.

### 3.3 Puntos de Control C2SP con Firma Múltiple

El sustrato adopta el formato C2SP signed-note para todos los puntos de control del registro de transparencia. El prefijo de hash de clave de 4 bytes (`SHA-256("<nombre_firmante>\nED25519\n<clave_pública>")[..4]`) es una pista de enrutamiento, no un vínculo criptográfico. Cuatro primitivas de verificación compuestas están implementadas en `system-core/src/checkpoint.rs`: `verify_signer` (verificación de firma simple); `verify_apex_handover` (composición AND de dos llamadas a `verify_signer`); `verify_inclusion_proof` (tamaño del árbol → firma → inclusión); y `verify_consistency_proof` (cinco comprobaciones ordenadas). La regla de composición es estructuralmente necesaria: la verificación de la firma y la verificación Merkle se presentan como una única primitiva porque el modo de fallo de realizarlas por separado —verificar la inclusión contra una raíz no autenticada— es silencioso y catastrófico.

### 3.4 Mecanismo A: Capacidades con Límite Temporal

El Mecanismo A cierra la brecha de auditoría en la delegación de capacidades: una extensión de testigo firmada que no está en el registro de transparencia es indetectable para un auditor. El núcleo honra una extensión solo si se cumplen los cinco pasos en orden: validez del apex; revocación (la capacidad no debe estar revocada); no expiración (si `expiry_t.is_none() || now < t`, permitir inmediatamente); vía de testigo (presencia del testigo, `witness.capability_hash == cap.hash()`, monotonicidad `witness.new_expiry_t > prev_expiry`, inclusión Merkle verificada, firma verificada contra `witness_pubkey`); y veredicto `ExtendThenAllow`. El orden es óptimo en coste: las comprobaciones estructurales baratas preceden a la costosa verificación criptográfica.

---

## 4. La Pila del Libro Mayor WORM

### 4.1 Arquitectura de Cuatro Capas

La pila WORM sigue una arquitectura de cuatro capas: L4 Anclaje (sellado de tiempo externo mediante Sigstore Rekor v2 mensualmente y por cada incremento MENOR); L3 Protocolo de transferencia (puntos de conexión axum HTTP: `/v1/append`, `/v1/checkpoint`, `/v1/tile/N/M`; servidor MCP superpuesto); L2 API del Libro Mayor WORM (trait Rust `LedgerBackend`); y L1 Almacenamiento de tejas (formato C2SP tlog-tiles, los mismos bytes que usa Sigstore Rekor v2 internamente). La propiedad crítica en L1: el formato de teja utilizado internamente por `service-fs` es idéntico al que Sigstore Rekor v2 usa externamente; no se produce ninguna conversión de formato en el límite de anclaje L4.

### 4.2 `system-ledger` v1.0.0: La Máquina de Estados del Lado del Núcleo

`system-ledger` v1.0.0 implementa el consumidor del sustrato del libro mayor WORM. Su rol es distinto de `service-fs`: `service-fs` *produce* puntos de control firmados para registros del nivel de aplicación; `system-ledger` *consulta* puntos de control cuando el núcleo decide si atender una invocación de capacidad. El trait `LedgerConsumer` expone cuatro operaciones; `Verdict` tiene tres variantes: `Allow`, `Refuse(RefuseReason)` y `ExtendThenAllow { new_expiry_t }`. `RefuseReason` tiene seis variantes estructuradas, cada una con una respuesta operativa distinta; no se colapsan en un único tipo `Error` porque diferentes clases de fallo requieren diferentes respuestas del operador.

### 4.3 Revocación y el Invariante Post-Traspaso

`RevocationSet` mantiene un `HashSet<Hash256>` O(1) para comprobaciones de pertenencia y un `HashMap<Hash256, RevocationEvent>` para detalles de auditoría. `apply_revocation` es idempotente: reproducir un evento de revocación devuelve `false` y preserva los campos de auditoría originales `signed_by` y `revoked_at`; esta tolerancia a la reproducción es estructuralmente necesaria para la replicación de flujo de registro durante la recuperación. `ApexHistory` registra las entradas de apex como intervalos de altura `[effective_from, effective_until]`; a la altura de traspaso H, tanto el apex antiguo como el nuevo son simultáneamente válidos, realizando la ceremonia N+3+.

### 4.4 La Ceremonia de Cofirma de Apex N+3+

La ceremonia de transferencia de titularidad se desarrolla en cuatro alturas del libro mayor: en la altura N se produce el último punto de control operativo firmado por P-antiguo; en N+1 P-antiguo añade una entrada de revocación ("liberación a C-nuevo efectiva N+2"); en N+2 se produce el punto de control de traspaso cofirmado por P-antiguo y P-nuevo mediante firma múltiple C2SP signed-note; en N+3+ los puntos de control son firmados solo por P-nuevo, y cualquier punto de control firmado solo por P-antiguo produce `Verdict::Refuse(StaleApex)`. El nuevo apex C-nuevo hereda todo el estado de capacidades, todo el historial de auditoría, toda la identidad operativa y todas las pruebas de verificación formal —las pruebas son matemáticas y se transfieren con el código fuente.

---

## 5. La Capa de Compatibilidad y las Propiedades de Transferibilidad

### 5.1 Por Qué NetBSD como Base de Compatibilidad

NetBSD fue elegido sobre FreeBSD, OpenBSD o Linux como base de compatibilidad según seis criterios concretos: licencia BSD de 2 cláusulas (transferible sin fricciones de copyleft); Veriexec —verificación de huella digital de binarios aplicada por el núcleo en `exec(2)`, no en una capa de aplicación que podría desactivarse—; reproducibilidad fuera de red con `build.sh` (ningún otro sistema operativo de propósito general ofrece una historia equivalente de construcción de todo el mundo desde un snapshot USB); núcleos rump —los componentes del núcleo NetBSD se ejecutan como procesos de usuario, el mismo código de controlador ejecuta en espacio de usuario, en metal desnudo y en dominios de protección seL4—; 57 arquitecturas hardware oficiales —la mayor cobertura de hardware de cualquier sistema operativo de propósito general—; y la independencia de la NetBSD Foundation —una organización sin ánimo de lucro estadounidense 501(c) sin membresía corporativa de hiperscalares y sin participación accionarial en proveedores de nube.

### 5.2 Arranque de Imagen Verificada Veriexec (Modo Estricto 3)

Veriexec es un subsistema del núcleo NetBSD que mantiene una tabla en el núcleo de huellas digitales de archivos indexadas por tupla `(dispositivo, inodo)`. El sustrato apunta al **modo estricto 3**: modo 2 más tabla de huellas digitales inmutable después del arranque; no se aceptan nuevas entradas. El cliente puede reverificar la imagen ejecutando `./build.sh -m evbarm -a aarch64 -U MKREPRO=yes tools distribution release` sobre la etiqueta de fuente fijada, ejecutando `veriexecgen` sobre el rootfs resultante, firmando con la clave de apex del cliente y comparando huellas digitales con la referencia suministrada por el proveedor. Toda la cadena de reproducibilidad es ejecutable por el cliente e independiente del proveedor.

### 5.3 La Envoltura CapabilityInvoker

Una envoltura Rust delgada (prevista como `system-substrate-netbsd`, paralela a `system-substrate-broadcom` y `system-substrate-freebsd` existentes) implementa el trait `CapabilityInvoker` del que dependen los binarios os-*. El indicador de característica de Cargo selecciona la base en tiempo de compilación: `features = ["native"]` para la base nativa seL4 mediante `seL4_Call()` / `seL4_Send()` a través de rust-sel4; `features = ["compat"]` para la base de compatibilidad NetBSD mediante un canal de mensajes de capacidades basado en descriptores de archivo POSIX. Ambas variantes presentan la misma superficie del trait Rust al binario os-* superior; el tipo `Verdict` de `system-ledger::LedgerConsumer::consult_capability` tiene la misma semántica independientemente de qué base ejecutó la invocación.

### 5.4 Recuperación de Capacidades desde Cualquier Hardware (Mecanismo C)

Un Archivo Totebox es recuperable a partir de una semilla impresa en papel en cualquier hardware que pueda arrancar NetBSD. El cliente arranca la ISO del Archivo, introduce la semilla impresa (clave privada de apex mediante BIP-39 de 12–24 palabras más ancla del libro mayor de 32 bytes SHA-256), el sistema obtiene las tejas del registro de transparencia público desde cualquier testigo de cofirma y reprodice las entradas del libro mayor desde el origen, reconstituyendo el estado de capacidades. El despliegue es operativo bajo la clave de apex del cliente. El tamaño del papel de semilla cabe en una tarjeta de índice de 10×15 cm o una hoja A4; no se requiere ninguna llamada al portal de recuperación del proveedor, ningún viaje de ida y vuelta a un HSM, ninguna re-certificación.

---

## 6. Implementación y Evaluación

### 6.1 moonshot-toolkit v0.2.0: Orquestación de Construcción Reproducible

`moonshot-toolkit` es un orquestador de construcción seL4 exclusivamente en Rust que reemplaza la cadena de herramientas Python+CMake+Ninja+Make+shell del árbol upstream de seL4. La canalización tiene dos etapas con dirección de contenido: `SystemSpec → spec_hash` (SHA-256 de la representación TOML canónica de la especificación) y `BuildPlan → plan_hash` (SHA-256 de la representación JSON canónica de `(spec_hash, pasos)`). Como Fase 1C.a (completada el 2026-05-27), el subcomando `build` ahora invoca el compilador cruzado real `aarch64-linux-gnu-gcc` con indicadores de metal desnudo (`-nostdlib -nostartfiles -ffreestanding -static -no-pie -march=armv8-a`), produciendo un ELF estático AArch64 de metal desnudo verificado (`build/hello.elf`, punto de entrada 0x40010c). El paso `AssembleImage` (Fase 1C.d) devuelve un error procesable identificando la dependencia faltante mientras los binarios PD ya compilados están disponibles.

### 6.2 Características de Rendimiento

Todas las medidas utilizan el arnés Criterion 0.5 en una VM GCP de clase n2 (Intel Xeon a 2,20 GHz, 4 vCPUs, 15 GiB RAM). La proporción de 358.000× entre el acierto de caché (11,2 ns, referencia 4) y la verificación Ed25519 completa (4,01 ms, referencia 2) hace que la caché de puntos de control sea estructuralmente necesaria en lugar de una optimización de rendimiento. En operación en estado estable, el núcleo publica nuevos puntos de control con poca frecuencia (solo en eventos de escritura: revocaciones, aplicaciones de registros de testigo, traspasos de apex); entre publicaciones, cada invocación de capacidad acierta la caché. En hardware ARM Cortex-A (el objetivo de producción seL4 AArch64), la verificación Ed25519 es aproximadamente 10–50× más lenta que en Intel Xeon, lo que estrecha la proporción de caché de 358.000× a aproximadamente 10.000–35.000× y hace que la caché sea obligatoria en lugar de muy útil.

### 6.3 Propiedades Formales Verificadas por la Implementación

La implementación no introduce nuevas afirmaciones criptográficas; verifica propiedades formales de las primitivas que compone. La conformidad con RFC 9162 v2 se verifica contra un oráculo construido independientemente sobre la cuadrícula completa `(antiguo, nuevo)` para `1 ≤ antiguo ≤ nuevo ≤ 8` (36 pares). El aislamiento de reproducción entre espacios de nombres (la firma producida bajo el espacio de nombres de firma de commit `git` falla la verificación bajo `capability-witness-v1`) está explícitamente probado. La monotonía del testigo, el enlace del testigo, el invariante post-traspaso y la idempotencia de la revocación están cada uno cubiertos por pruebas de integración end-to-end. No está formalmente verificado: silicio, microcódigo, firmware (Boot Guard/ME/PSP/SMM) ni mitigaciones de hardware de Spectre y Rowhammer; estos quedan fuera del alcance del sustrato.

---

## 7. Discusión

### 7.1 La Composición como Contribución

Las primitivas criptográficas de esta arquitectura no son invenciones novedosas: RFC 9162 Certificate Transparency v2.0 es un estándar IETF maduro; SHA-256 es FIPS 180-4; Ed25519 es RFC 8032; C2SP signed-note es una especificación publicada y estable; el trabajo de verificación formal de seL4 abarca quince años y múltiples certificaciones de prueba independientes. **La contribución es la composición.** Ningún sistema en producción en 2026 conecta un tipo de capacidad del núcleo a un registro RFC 9162 bajo raíz del cliente mediante puntos de control de firma múltiple C2SP signed-note, con gating de prueba de inclusión en la validez del lado de escritura, gating de prueba de consistencia en la seguridad de la replicación, un diseño de doble base que habilita los mismos binarios en seL4 y NetBSD, y una ceremonia de cofirma de apex atómica que hace que la transferencia de titularidad sea un evento del libro mayor en lugar de un proyecto de migración de identidad.

### 7.2 Posicionamiento Estructural

El sustrato evalúa honestamente qué posee y qué no: el núcleo, la capa del sistema, las aplicaciones, el libro mayor de capacidades, la identidad, el historial de auditoría, la procedencia de la construcción y los artefactos de verificación formal son propiedad del sustrato; el silicio y el microcódigo no. La limitación estructural a las arquitecturas existentes es compositiva, no técnica: la economía de SaaS multiteniente de hiperscalares requiere raíces de confianza del proveedor compartidas que la raíz de libro mayor de capacidades por cliente rompe; la gestión de identidad del proveedor (Okta, Microsoft Entra, Auth0) ancla la autenticación en claves del proveedor; los vendedores de RTOS propietarios no pueden publicar registros mediados por el núcleo porque el código fuente es propietario; los certificados Common Criteria EAL están vinculados a un proveedor específico y a un objetivo de evaluación.

### 7.3 Limitaciones

Ocho directorios os-* en `pointsav-monorepo` están actualmente en estado de código de andamiaje; las afirmaciones de producción para la capa os-* son prospectivas. La construcción cruzada de seL4 AArch64 mediante `moonshot-toolkit` está planificada como entregable de la Fase 1C: la Fase 1C.a (CompilePd compilando ELF de metal desnudo AArch64 verificado) y la Fase 1C.b (kernel.elf de seL4 para qemu-arm-virt) están completas; la Fase 1C.c (arranque QEMU) y la Fase 1C.d (AssembleImage) están bloqueadas. La verificación multicore de seL4 es un problema de investigación abierto; el sustrato apunta a configuraciones de un solo núcleo o en espera de multinúcleo hasta que la verificación multicore de seL4 se complete. La envoltura `system-substrate-netbsd` no existe aún. La referencia 9 (`verify_inclusion_proof` compuesta, árbol de 1.024 hojas) tiene 22 valores atípicos y ±11% CI —sensible a la carga y no de calidad de publicación; está planificada una nueva ejecución en VM tranquila (promedio de carga < 1,0).

### 7.4 Hipótesis Formales y Programa de Falsificación

**H₁ (Principal — Transferibilidad):** Un cliente que posea únicamente su clave privada de apex y el ancla del libro mayor de 32 bytes puede reconstituir completamente un Archivo Totebox —estado operativo de capacidades, historial de auditoría completo e identidad— en cualquier hardware capaz de arrancar el núcleo NetBSD `GENERIC64`, sin intervención del proveedor, sin re-certificación y sin migración de estado. **H₀ (Nula):** La reconstitución requiere infraestructura del proveedor, reclave de al menos una capacidad interna o re-certificación del despliegue con el nuevo hardware. **H₂ (Semántica Idéntica):** El mismo binario os-*, compilado con `features = ["native"]` para la base seL4 y con `features = ["compat"]` para la base NetBSD, produce semánticas de evento de libro mayor de capacidades idénticas dado el mismo estado de capacidades y los mismos registros de testigo. H₁ se falsifica si cualquier paso del flujo de recuperación del Mecanismo C requiere un recurso de red controlado por el proveedor. H₂ se falsifica si las variantes de indicador de característica `native` y `compat` producen valores de `Verdict` diferentes, hashes de capacidad diferentes o cargas de entrada del libro mayor diferentes dadas secuencias de entrada idénticas.

---

## 8. Conclusión

Este trabajo ha presentado una arquitectura de sustrato para despliegues de sistemas operativos fiables bajo control del cliente. Las tres contribuciones son: el Sustrato de Libro Mayor de Capacidades (Doctrina, afirmación 33), en el que el estado de capacidades del sustrato ES el libro mayor WORM y cuya implementación en Rust está evaluada con 62+47 = 109 casos de prueba y 10 referencias Criterion; el diseño de doble base del sistema operativo (Doctrina, afirmación 34), que consigue alcance de hardware sin comprometer las propiedades de sistemas fiables mediante una envoltura Rust delgada que permite que los mismos binarios os-* se ejecuten tanto en seL4 como en NetBSD; y la ceremonia de cofirma de apex N+3+, que hace que la transferencia de titularidad del despliegue sea atómica (un único evento del libro mayor a la altura N+2), auditable (las firmas de ambas partes en un registro de transparencia público) y definitiva (P-antiguo rechazado por el núcleo a las alturas N+3+). El nuevo apex hereda todo el estado de capacidades, historial de auditoría, identidad operativa y pruebas de verificación formal: las pruebas son matemáticas y se transfieren con el código fuente.

---

## Referencias

*(Formato Chicago autor-fecha; las citas en línea utilizan la sintaxis [citation-id] donde existen IDs estables en citations.yaml.)*

*(Las referencias son las mismas que el documento en inglés: Apple Security Research 2024; AWS 2025; Birgisson et al. 2014; European Commission 2025; ETSI 2024; Heiser y Klein 2010; Klein et al. 2009; Klein et al. 2014; Laurie et al. 2013; Murray et al. 2013; NetBSD Project 2026; Newman, Meyers y Torres-Arias 2022; Sewell et al. 2011; IETF RFC 9162; IETF RFC 8032; C2SP 2024 signed-note; C2SP 2024 tlog-tiles; US SEC Rule 17a-4(f); Watson et al. 2010.)*
