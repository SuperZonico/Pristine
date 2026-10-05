# ⚙️ 02. Arquitectura Técnica y Modelo de Seguridad de Pristine

> *"La solidez no se negocia: cero fallas de memoria, privilegios mínimos estrictos y tolerancia cero a vulnerabilidades."*

---

## 1. Visión General de la Arquitectura

Para que **Pristine** sea verdaderamente imposible de comprometer y no exponga la máquina a vectores de ataque de elevación de privilegios (LPE), se adopta un **Modelo de Doble Proceso con Separación de Privilegios** (*Privilege Separation Architecture*):

```
┌────────────────────────────────────────────────────────┐
│               PRISTINE UI (Medium Integrity)           │
│  - Renderizado visual (Mica / Dark & Light Theme)      │
│  - Gráficos de telemetría y métricas en tiempo real    │
│  - Interacción del usuario y selección de perfiles     │
│  - CERO permisos de administrador directos             │
└───────────────────────────┬────────────────────────────┘
                            │
                            │ IPC Seguro (Named Pipe con SDDL)
                            │ Validación criptográfica de comandos
                            ▼
┌────────────────────────────────────────────────────────┐
│             PRISTINE ENGINE (High Integrity)           │
│  - Ejecución de directivas de registro (Win32 Reg API) │
│  - Control de servicios de Windows (SCM API)           │
│  - Gestión de Tareas Programadas (Task Scheduler COM)  │
│  - Snapshots VSS y diffs de Rollback transaccional     │
│  - Solo acepta comandos autenticados del usuario local │
└────────────────────────────────────────────────────────┘
```

---

## 2. ¿Por Qué Separar Privilegios? (Seguridad Extrema)

La gran mayoría de optimizadores de Windows se ejecutan como un único proceso gigante con privilegios de **Administrador**. Esto es una falla de seguridad colosal:
- Si el motor de renderizado de la UI tiene una vulnerabilidad de desbordamiento, un atacante o software malicioso local podría inyectar código directamente con privilegios elevados.
- Al aislar la interfaz en **Medium Integrity** y el motor de operaciones en un worker de **High Integrity**, cualquier superficie visual queda completamente contenida.
- El canal IPC está protegido por descriptores de seguridad de Windows (SDDL) que impiden que otros procesos en el sistema envíen comandos al motor elevado.

---

## 3. Componentes del Workspace de Rust

Pristine se organiza como un **Cargo Workspace** modular y desacoplado:

```
pristine/
├── Cargo.toml                  # Workspace manifest
├── crates/
│   ├── pristine-core/          # Modelos de datos, definiciones de tweaks, reglas y estado
│   ├── pristine-winapi/        # Abstracción segura de APIs nativas de Windows (Win32/COM/Registry)
│   ├── pristine-engine/        # Lógica de aplicación, auditoría, diffs y rollback
│   ├── pristine-metrics/       # Colector de métricas de rendimiento (PDH, CPU, RAM, Disk, Net)
│   ├── pristine-ipc/           # Protocolo de comunicación seguro entre UI y Engine
│   └── pristine-app/           # Punto de entrada de la aplicación de usuario
```

### Detalle de Crates:

### `pristine-core`
- Define la estructura de cada `Tweak`:
  ```rust
  pub enum TweakCategory {
      Telemetry,
      Privacy,
      SystemCleanup,
      Performance,
      SecurityEnhancement,
  }

  pub enum RiskLevel {
      Safe,        // Verde: Sin impacto en funciones secundarias
      Moderate,    // Amarillo: Desactiva funciones que algunos usan (ej. historial portapapeles)
      Specific,    // Naranja: Afecta servicios como Xbox, Cortana, etc.
      Experimental,// Rojo: Solo para usuarios que saben exactamente qué hacen
  }

  pub struct TweakDefinition {
      pub id: &'static str,
      pub title: &'static str,
      pub description: &'static str,
      pub category: TweakCategory,
      pub risk: RiskLevel,
      pub impact_details: &'static str,
      pub reversible: bool,
  }
  ```

### `pristine-winapi`
- Encapsula llamadas a `windows` / `windows-sys`.
- Control seguro del Registro de Windows:
  - `RegOpenKeyExW`, `RegQueryValueExW`, `RegSetValueExW`, `RegDeleteValueW`.
- Control del Service Control Manager (SCM):
  - `OpenSCManagerW`, `OpenServiceW`, `QueryServiceStatusEx`, `ChangeServiceConfigW`.
- COM API de `ITaskService` para inspeccionar y deshabilitar tareas del Task Scheduler sin borrarlas.
- Creación de puntos de restauración del sistema mediante `SRSetRestorePointW`.

### `pristine-metrics`
- Monitoreo en tiempo real de bajo consumo usando **PDH (Performance Data Helper)** y APIs de kernel:
  - Uso de CPU por núcleo y porcentaje de interrupciones de hardware (DPC/ISR).
  - Memoria RAM física, standby list y modified list.
  - Tráfico de red activo y tasa de solicitudes bloqueadas hacia servidores de telemetría de Microsoft.
  - Temperatura y throttling si las interfaces WMI/ACPI están disponibles.

---

## 4. Selección del Framework de UI

Para la interfaz de usuario, se evaluaron tres opciones dentro del ecosistema Rust:

| Criterio | Tauri v2 (Recomendado) | Slint | Iced / Egui |
| :--- | :--- | :--- | :--- |
| **Integración Nativa Win 11** | Excelente (usa WebView2 nativo de Windows 11, sin descargas) | Buena (compilación a DirectX / Skia) | Básica (look and feel propio no-nativo) |
| **Efectos Mica & Acrylic** | Soporte directo vía DWM (`window-vibrancy`) | Requiere shaders personalizados | Muy complejo de sincronizar con Windows |
| **Tema Claro / Oscuro / Sistema** | Sincronización instantánea con CSS variables y `matchMedia` | Soporte manual en sintaxis Slint | Soporte manual |
| **Consumo de Memoria** | ~35 MB - 50 MB de RAM | ~25 MB - 40 MB de RAM | ~30 MB - 60 MB de RAM |
| **Estética y Animaciones** | Máxima libertad visual (Glassmorphism, SVG fluido, micro-interacciones) | Limitada a componentes nativos | Estética técnica/funcional |

> **Decisión Arquitectónica:** **Tauri v2** ofrece la combinación perfecta de rendimiento nativo en el backend (Rust con cero fugas de memoria) y una interfaz visualmente hipnótica, elegante y moderna, aprovechando que **todos los sistemas Windows 11 traen WebView2 preinstalado de fábrica**.

---

## 5. Garantía de Seguridad de Memoria y Resistencia a Ataques

1. **`#![forbid(unsafe_code)]`** en todos los crates de parsing, IPC y lógica de negocio.
2. Todo bloque `unsafe` en `pristine-winapi` debe:
   - Tener una precondición explícita validada (`assert!`).
   - Manejar códigos de retorno de error de Win32 (`GetLastError()`) convirtiéndolos a tipos `Result<T, PristineError>`.
   - Limpiar descriptores de Windows (`CloseHandle`, `RegCloseKey`, `CloseServiceHandle`) automáticamente mediante el patrón RAII (`Drop`).
3. **Cero inyección de comandos**: No se usa `std::process::Command` llamando a `powershell.exe` ni `cmd.exe`. Toda operación sobre el sistema se realiza de forma directa mediante las **APIs binarias del sistema operativo**, previniendo ataques de inyección de parámetros.
