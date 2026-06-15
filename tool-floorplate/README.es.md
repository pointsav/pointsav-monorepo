# tool-floorplate

**Composición de Planta de Piso** — Álgebra de Tiles de Planes Clave Woodfine (enero 2026).

Calcula el desglose de la planta de piso a partir de la clase de desarrollo y el área bruta:
núcleo del edificio, área arrendable neta, conteo de tiles, fracciones de Planes Clave CO
y totales del edificio.

## Clases de desarrollo

| Clase | Pisos | Planta bruta |
|---|---|---|
| `professional-centre` | 3–5 | 19,000–23,000 SF |
| `suburban-office` | 6–9 | 17,000–21,000 SF |
| `retail-select` | 1 | Tile = Planta (T/M/G) |
| `tech-industrial` | 1 | Tile = Planta (M/G) |

Para Professional Centre y Suburban Office:
- Núcleo del edificio: 13% del área bruta
- Área arrendable neta: 87% del área bruta
- 1 Tile (CO-1/8) = 2,500 SF arrendables
- CO-FF = 20,000 SF (piso completo, 8 tiles)

## Fracciones de Planes Clave CO

| Etiqueta | Arrendable | Tiles |
|---|---|---|
| CO-FF | 20,000 SF | 8 tiles — piso arrendable completo |
| CO-1/2 | 10,000 SF | 4 tiles |
| CO-1/3 | 6,667 SF | 2–3 tiles |
| CO-1/4 | 5,000 SF | 2 tiles |
| CO-1/8 | 2,500 SF | 1 tile — unidad mínima |

## Álgebra de Tiles

```
T_Basic    = n × PO-S (325 SF) + p × PO-M (465 SF) + q × PO-L (685 SF)
T_Compound = T_Basic + Planes Clave de amenidades
T_Special  = T_Basic + Planes Clave de esquina / lobby de ascensores
Planta     = T_Basic + T_Compound + T_Special + Núcleo del Edificio
```

Fuente: `inputs/CONSTRUCTION_MCorp_2026_01_06_Tiles_Leasing Plan Efficiencies_FIN.docx`.

## Uso

```bash
# Punto medio de la clase (área predeterminada)
tool-floorplate --class professional-centre

# Área bruta específica
tool-floorplate --class professional-centre --area 21000

# Con conteo de pisos — agrega totales del edificio
tool-floorplate --class suburban-office --area 19500 --floors 7

# Tamaños de tile para Retail Select
tool-floorplate --class retail-select

# Salida JSON para uso en pipeline
tool-floorplate --class professional-centre --area 21000 --floors 4 --format json
```

## Compilación

```bash
cargo build -p tool-floorplate
```
