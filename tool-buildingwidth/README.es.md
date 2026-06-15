# tool-buildingwidth

**Calculadora de Ancho de Edificio** — Planes Clave Woodfine V3 (enero de 2026).

Calcula el ancho de planta baja de doble carga a partir de las profundidades de zona de los Planes Clave.

## Fórmula

```
ancho_total = 2 × (Z1 + Z2) + Z3
```

| Zona | Nombre | Definición |
|---|---|---|
| Z1 | Hábitat | Zona de estaciones de trabajo — todos los escritorios a menos de 6 m de la fachada (Estándar Europeo de Iluminación) |
| Z2 | Revista | Zona de almacenamiento más allá de Z1 |
| Z3 | Corredor | Corredor central compartido por ambos lados de la planta |

## Profundidades de zona — V3 enero 2026

| Categoría | Z1 (m) | Z2 (m) | Z3 (m) | Semiancho (m) | Ancho total (m) |
|---|---|---|---|---|---|
| private-office | 6.0 | 3.8 | 2.0 | 9.8 | 21.6 |
| medical | 7.2 | 4.9 | 2.9 | 12.1 | 27.1 |
| business | 6.0 | 7.3 | 2.7 | 13.3 | 29.3 |
| laboratory | 6.8 | 4.8 | 3.0 | 11.6 | 26.2 |
| academic | 4.7 | 3.0 | 0.0 | 7.7 | 15.4 |
| civic | 6.0 | 7.2 | 3.6 | 13.2 | 30.0 |

Fuente: `DISCOVERY_MCorp_Sketches_Key_Plans_Summary.pdf` (V3).

## Uso

```bash
# Modo categoría — usa la tabla de zonas V3
tool-buildingwidth --category private-office
tool-buildingwidth --category private-office --area 30.19

# Modo personalizado — profundidades de zona explícitas
tool-buildingwidth --z1 6.0 --z2 3.8 --z3 2.0 --area 43.20

# Salida JSON para uso en pipeline
tool-buildingwidth --category medical --area 223 --format json
```

## Compilación

```bash
cargo build -p tool-buildingwidth
```
