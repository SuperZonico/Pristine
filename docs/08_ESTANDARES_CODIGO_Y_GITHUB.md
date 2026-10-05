/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         docs/08_ESTANDARES_CODIGO_Y_GITHUB.md
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Estándares de ingeniería, plantillas de cabecera de código,
 *               especificación de iconos vectoriales SVG hechos a mano y
 *               política de preparación para publicación pública en GitHub.
 * ============================================================================
 */

# 08. Estándares de Código, Iconografía Vectorial y Preparación GitHub

> *"Código impecable que hable por sí mismo: profesional, limpio, sin rastro de automatismos burdos y listo para ser admirado por la comunidad open-source mundial."*

---

## 1. Plantilla de Cabecera Obligatoria para Archivos de Código

Cada archivo fuente del proyecto (`.rs`, `.ts`, `.js`, `.css`, `.sql`, etc.) debe incluir en la parte superior un encabezado formal, técnico y conciso.

### Formato para Rust (`.rs`):
```rust
/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         crates/pristine-winapi/src/registry/mod.rs
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Type-safe transactional Windows Registry abstractions.
 * ============================================================================
 */
```

### Formato para Frontend (`.ts`, `.css`):
```typescript
/*
 * ============================================================================
 * Project:      Pristine — Windows 11 Optimization & Privacy Suite
 * File:         src/components/MetricCard.ts
 * Author:       SuperZonico
 * License:      MIT License
 * Purpose:      Real-time telemetry metric display card component.
 * ============================================================================
 */
```

### Reglas de Estilo de Comentarios:
- **Cero comentarios cliché o explicaciones triviales**: Prohibido añadir comentarios redundantes tipo `// modulo de limpieza`, `// funcion para sumar` o `// aqui creamos la variable`.
- **Enfoque en la Razón Técnica (*Why*, no *What*)**: Comentar únicamente decisiones de arquitectura no evidentes, invariantes de seguridad de bajo nivel, razones de directivas de registro de Windows o contratos de concurrencia.
- **Documentación Rustdoc canónica**: Utilizar `///` para funciones y estructuras públicas, describiendo argumentos, valores de retorno y condiciones de error (`# Errors`).

---

## 2. Iconografía y Logotipo: 100% SVG Vectorial Hecho a Mano

Queda **terminantemente prohibido** el uso de emojis dentro de la interfaz gráfica de la aplicación (los emojis rompen la seriedad del software, dependen de la fuente del sistema operativo y lucen inconsistentes entre plataformas).

Toda la iconografía y el imagotipo de **Pristine** se diseñan a nivel de código mediante vectores SVG limpios, optimizados y matemáticamente precisos.

### A. El Imagotipo de Pristine (Logotipo Vectorial de Alta Fidelidad)
El isotipo oficial representa un prisma de diamante angular tallado con precisión, simbolizando pureza, dureza indestructible y transparencia:

```xml
<svg viewBox="0 0 48 48" fill="none" xmlns="http://www.w3.org/2000/svg" class="pristine-logo">
  <defs>
    <linearGradient id="pristine-grad-primary" x1="4" y1="4" x2="44" y2="44" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#00f0a8" />
      <stop offset="50%" stop-color="#00c8ff" />
      <stop offset="100%" stop-color="#7000ff" />
    </linearGradient>
    <linearGradient id="pristine-grad-facet" x1="12" y1="8" x2="36" y2="40" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="rgba(255,255,255,0.4)" />
      <stop offset="100%" stop-color="rgba(255,255,255,0.02)" />
    </linearGradient>
  </defs>
  <!-- Prisma exterior tallado -->
  <polygon points="24,4 42,14 42,34 24,44 6,34 6,14" 
           stroke="url(#pristine-grad-primary)" stroke-width="2.5" stroke-linejoin="round" fill="none"/>
  <!-- Facetas interiores con reflejo traslúcido -->
  <polygon points="24,4 33,19 24,34 15,19" fill="url(#pristine-grad-facet)" />
  <line x1="24" y1="4" x2="24" y2="34" stroke="url(#pristine-grad-primary)" stroke-width="1.5" />
  <line x1="6" y1="14" x2="15" y2="19" stroke="url(#pristine-grad-primary)" stroke-width="1.5" />
  <line x1="42" y1="14" x2="33" y2="19" stroke="url(#pristine-grad-primary)" stroke-width="1.5" />
  <line x1="15" y1="19" x2="6" y2="34" stroke="url(#pristine-grad-primary)" stroke-width="1.5" />
  <line x1="33" y1="19" x2="42" y2="34" stroke="url(#pristine-grad-primary)" stroke-width="1.5" />
  <line x1="24" y1="34" x2="24" y2="44" stroke="url(#pristine-grad-primary)" stroke-width="1.5" />
</svg>
```

### B. Sistema de Iconos Nativos SVG
Todos los iconos de la barra lateral, botones y estados siguen una cuadrícula de 20x20 o 24x24 px con grosor uniforme (`stroke-width="1.75"`), puntas redondeadas (`stroke-linecap="round"` `stroke-linejoin="round"`) y color heredado (`currentColor`):

1. **Dashboard / Resumen**: Cuadrícula de 4 cuadrantes modulares.
2. **Escudo de Privacidad**: Silueta de escudo con corte central limpio.
3. **Optimización y Limpieza**: Chispa de diamante y flujo dinámico.
4. **Servicios y Tareas**: Engranaje de 6 dientes simétricos.
5. **Historial y Rollback**: Flecha circular antihoraria con nodo central.
6. **Métricas de Rendimiento**: Pulso de frecuencia cardíaca / telemetría de bus.
7. **Modo Claro / Oscuro**: Sol radiante con rayos equidistantes y Luna creciente limpia.
8. **Insignias de Estado**:
   - *Seguro*: Checkmark circular minimalista.
   - *Advertencia*: Triángulo equilátero con vértice suavizado.
   - *Crítico*: Octágono regular de detención.

---

## 3. Preparación para el Lanzamiento Público en GitHub

Para que el repositorio en GitHub refleje el máximo estándar de la industria desde el commit inicial:

### Estructura de Archivos de Repositorio de Nivel Enterprise:
```
Pristine/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                 # Build de Rust, cargo test, clippy y fmt automáticos
│   │   └── release.yml            # Compilación de binarios firmados y checksums SHA-256
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md          # Plantilla estructurada para reportar fallos
│   │   └── feature_request.md     # Plantilla para solicitar nuevos módulos
│   └── pull_request_template.md   # Checklist de seguridad para contribuciones externas
├── docs/                          # El Libro Maestro de Pristine
├── .gitignore                     # Exclusión estricta de temporales, builds y credenciales
├── Cargo.toml                     # Workspace configuration con perfiles release agresivos
├── CONTRIBUTING.md                 # Guía para desarrolladores externos
├── LICENSE                        # Licencia MIT de código abierto
├── README.md                      # Portada pública para GitHub con badges y capturas
└── SECURITY.md                    # Política de divulgación responsable de vulnerabilidades
```

### Política de Seguridad de Datos del Creador:
- **Cero rutas locales absolutas en código**: Prohibido hardcodear rutas personales de Windows (`C:\Users\luist\...`). Toda ruta en tiempo de ejecución se resuelve de forma dinámica usando APIs del sistema (`CSIDL_LOCAL_APPDATA`, `FOLDERID_ProgramData`, o variables de entorno estándar `%SystemRoot%`, `%TEMP%`).
- **Cero tokens, certificados o secretos**: No se incluirán claves de firma privadas ni credenciales personales en el árbol de Git.
- **Atribución Pública Oficial**:
  - Autor y Arquitecto Principal: **SuperZonico**
  - Proyecto: **Pristine**
  - Repositorio Público: `https://github.com/SuperZonico/Pristine` (o la organización/usuario final elegido).
