# 06. Sistema de Diseño, Interfaz y Experiencia de Usuario (UI/UX)

> *"Una herramienta técnica no tiene por qué ser tosca ni fría. Pristine está diseñada para enamorar la vista: fluida, moderna, hipnótica y adaptativa al milisegundo."*

---

## 1. Filosofía Visual: Seducción y Claridad Absoluta

Pristine rompe con la estética tradicional de "consola negra aburrida" o "ventanas grises noventeras". Su diseño abraza la filosofía **Windows 11 Fluent Design**:
- **Fondos Traslúcidos (Mica Alt & Acrylic)**: Integración orgánica con el papel tapiz del escritorio mediante llamadas a la API DWM (`DwmSetWindowAttribute`).
- **Bordes Suaves y Cristalinos**: Sombras sutiles, micro-gradientes y bordes de 1 píxel con transparencia (`rgba(255, 255, 255, 0.08)` en oscuro / `rgba(0, 0, 0, 0.06)` en claro).
- **Tipografía Moderna**: Uso de fuentes de sistema optimizadas para legibilidad de alta densidad (**Segoe UI Variable**, **Inter**).

---

## 2. Paleta de Colores y Temas Dinámicos

Pristine incluye tres modos de visualización seleccionables al instante:

```
[ Modo Claro (Frost White) ]   ───┐
[ Modo Oscuro (Obsidian Glow) ] ──┼──> [ Sincronización Automática con Windows 11 ]
[ Modo Sistema (Sync) ]        ───┘
```

### Tabla de Tokens de Color:

| Token | Modo Oscuro (Obsidian) | Modo Claro (Frost Pure) | Propósito |
| :--- | :--- | :--- | :--- |
| `--bg-base` | `#0b0f17` (Obsidiana Profunda) | `#f8fafc` (Nieve Cristalina) | Fondo raíz de ventana con Mica |
| `--bg-surface` | `#131b26` (Medianoche) | `#ffffff` (Blanco Puro) | Tarjetas, contenedores de opciones |
| `--bg-hover` | `#1b2636` | `#f1f5f9` | Efecto hover interactivo |
| `--text-primary` | `#f1f5f9` (Blanco suave) | `#0f172a` (Pizarra oscuro) | Títulos y etiquetas clave |
| `--text-muted` | `#94a3b8` (Gris azulado) | `#64748b` (Gris neutro) | Descripciones y notas técnicas |
| `--accent-emerald` | `#10b981` (Esmeralda Neón) | `#059669` (Esmeralda Intenso) | Estado seguro, privacidad óptima, éxito |
| `--accent-amber` | `#f59e0b` (Ámbar Calibrado) | `#d97706` (Ámbar) | Nivel moderado, atención requerida |
| `--accent-rose` | `#f43f5e` (Rosa Carmesí) | `#e11d48` (Carmesí) | Telemetría activa detectada, advertencias |
| `--accent-cyan` | `#06b6d4` (Cian Eléctrico) | `#0284c7` (Zafiro) | Métricas de rendimiento, gráficos en vivo |

### Sincronización en Tiempo Real con el Sistema:
Pristine escucha activamente la clave de registro:
`HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize\AppsUseLightTheme`
y el evento CSS `window.matchMedia('(prefers-color-scheme: dark)')`.
Si el usuario tiene programado que Windows cambie de claro a oscuro al atardecer, Pristine transiciona de forma sedosa (transición CSS de 300ms) sin necesidad de reiniciar la app.

---

## 3. Arquitectura de Pantallas y Navegación

La interfaz se divide en 5 secciones accesibles desde un menú lateral estilizado (*Collapsible Sidebar*):

```
┌────────────┬────────────────────────────────────────────────────────┐
│  PRISTINE  │  PANEL PRINCIPAL: PRIVACIDAD Y RENDIMIENTO             │
│  [LogoSVG] ├────────────────────────────────────────────────────────┤
│  [Icon-Dash]  Dashboard    │  [ Puntuación de Privacidad: 94% ]     │
│  [Icon-Priv]  Privacidad   │  [ Telemetría: INACTIVA ]              │
│  [Icon-Tune]  Optimización │                                        │
│  [Icon-Serv]  Servicios    │  [ Métricas de Hardware en Tiempo Real]│
│  [Icon-Hist]  Rollback     │  CPU: 2.1%  |  RAM: 18%  | Bloqueos: 142│
│  [Icon-Gear]  Ajustes      │                                        │
└────────────┴────────────────────────────────────────────────────────┘
```

### 1. Panel de Control (Dashboard)
- **Privacy Score Meter**: Un anillo circular dinámico que muestra el grado de exposición del sistema (0 a 100%).
- **Contador en Vivo de Telemetría Bloqueada**: Muestra el número de paquetes e intentos de conexión abortados hacia servidores de Microsoft en las últimas 24h.
- **Gráficos de Hardware Ligeros**: Sparklines de uso de CPU, RAM liberada y estado de latencia de red.

### 2. Privacidad y Telemetría
- Interruptores modernos con animación fluida agrupados por categorías:
  - Telemetría de Diagnóstico (DiagTrack, CEIP, WER).
  - Telemetría de Explorador e Inicio (Bing Search, Anuncios, Sugerencias).
  - IA y Nuevas Tecnologías (Copilot, Recall, Inking & Typing).
  - Telemetría de Navegadores (Edge Telemetry, Pre-carga).
- Cada interruptor cuenta con un botón desplegable **"¿Qué hace esto?"** que expande la ruta exacta en el registro, el impacto y el comando de reversión.

### 3. Limpieza y Optimización
- Selector granular de limpieza:
  - Temporales seguros (>24h).
  - Reducción del Almacén WinSxS (botón guiado de DISM).
  - Purgado de Caché P2P de Windows Update.
  - Caché de Shaders (DirectX/GPU).
- Toggles de latencia DPC, mitigación de throttling multimedia y CompactOS.

### 4. Gestor de Servicios y Tareas
- Tabla ordenada y filtrable de servicios y tareas programadas.
- Insignia de riesgo: **Verde (Safe to Disable)**, **Amarillo (Review)**, **Rojo (Critical System Dependency)**.
- Búsqueda en vivo instantánea con filtrado por nombre y descripción.

### 5. Historial y Rollback
- Línea de tiempo visual que registra cada modificación efectuada con fecha, hora y lista de cambios.
- Botón individual para deshacer una sesión específica o restaurar el estado completo de fábrica.

---

## 4. Eficiencia Extrema de Renderizado (Cero Desperdicio de CPU)

A diferencia de aplicaciones como Discord o Spotify que consumen 2-5% de CPU solo para mantener la ventana abierta:
1. **Pausado en Segundo Plano**: Cuando Pristine se minimiza o pierde el foco (`document.hidden`), el bucle de animación de gráficos y las consultas de métricas se reducen a un intervalo de 5 segundos o se suspenden por completo.
2. **Uso de CSS Transform y GPU**: Todas las animaciones (toggles, modales, transiciones de pantalla) utilizan propiedades aceleradas por hardware (`transform`, `opacity`) para garantizar **60/120/144 FPS estables** sin elevar la temperatura del procesador.
3. **Consumo en Reposo**: **0.0% de uso de CPU** cuando la ventana está inactiva.
