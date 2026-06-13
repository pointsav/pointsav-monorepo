# app-mediakit-shell

Chasis de cromo compartido para la familia de aplicaciones `os-mediakit`.

Este crate es para la familia mediakit lo que `app-console-keys` es para la
familia console: posee el cromo compartido y el vocabulario de componentes,
mientras que el binario del SO (`os-mediakit`) lanza las instancias de las
aplicaciones y cada crate de aplicación (`app-mediakit-marketing` y —
planificado — `-knowledge` / `-distributions`) depende de este. **No** es un
binario y **no** es el SO.

## Qué proporciona

1. **Cromo** (`shell`) — el marco persistente de encabezado / pie / página,
   portado a [maud](https://maud.lambda.xyz) desde las plantillas del shell de
   marketing de Woodfine. Parametrizado por inquilino mediante `Brand`
   (Woodfine, PointSav) para que un binario sirva varias instancias con la
   misma forma de cromo y distintas marcas/enlaces. El punto de entrada de
   renderizado es `shell::render_page`.

2. **Vocabulario de componentes** (`section`, `page`) — el conjunto tipado
   `Section` a partir del cual un autor de IA compone una página. Un manifiesto
   de página es un archivo YAML que enumera secciones tipadas; o bien se
   deserializa en estos tipos (el contrato) o se rechaza. No hay vía hacia HTML
   o CSS arbitrario. Subconjunto del andamiaje: `hero`, `prose`, `cta`.

3. **Tokens** (`tokens`) — carga de tokens de diseño DTCG. Los componentes
   solo referencian propiedades personalizadas de tokens; el paquete canónico
   de `pointsav-design-system` sobrescribe el respaldo integrado sin cambios en
   los componentes.

## Por qué el CSS vive aquí

Los componentes de sección poseen su CSS adaptable en `static/sections.css`.
Los manifiestos de contenido no llevan **nada** de CSS — una IA elige un tipo
de sección y vincula datos; nunca escribe una regla de estilo. La corrección
móvil es una propiedad probada de los componentes, no se regenera por página.
Así es como la plataforma "absorbe el CSS".

## Compilar

```
cd app-mediakit-shell
cargo test
```

## Estado

Andamiaje (P1). El catálogo completo de secciones, una sincronización de
tokens externos y la adopción por `app-mediakit-knowledge` / `-distributions`
son fases posteriores — ver `.agent/briefs/BRIEF-marketing-platform-master.md`.
