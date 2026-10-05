# 📡 03. Catálogo Exhaustivo de Telemetría y Servicios en Windows 11

> *"Conocer al adversario al milímetro: mapeo exhaustivo de cada sonda, cada servicio y cada tarea silenciosa que vigila a Windows 11."*

---

## 1. Niveles de Telemetría Oficiales de Windows 11

Windows 11 organiza internamente su telemetría a través del componente **UTC (Universal Telemetry Client)**:

```
[ Nivel 0: Security (Solo Enterprise/IoT) ] ──> Mínimo absoluto (datos de seguridad de Defender)
[ Nivel 1: Required / Basic ]               ──> Especificaciones de hardware, calidad, compatibilidad
[ Nivel 2: Enhanced (Deprecado en Win 11) ] ──> Métricas intermedias de uso
[ Nivel 3: Optional / Full ]                ──> Páginas web visitadas, dumps de memoria completos, uso de apps
```

En ediciones Windows 11 Home y Pro, la interfaz de configuración **no permite** bajar del nivel "Basic" (Required). Sin embargo, a través de directivas de grupo en el registro, **Pristine** puede forzar el nivel mínimo o desarticular las sondas secundarias de recopilación.

---

## 2. Mapa Detallado de Servicios de Telemetría

| Servicio (Nombre Técnico) | Nombre para Mostrar | Función Real de Microsoft | Impacto de Desactivación | Recomendación Pristine |
| :--- | :--- | :--- | :--- | :--- |
| **`DiagTrack`** | Connected User Experiences and Telemetry | Recopila eventos de uso del sistema, caídas de apps y los transmite periódicamente a Microsoft. Consume I/O de disco y red. | Se desactivan las analíticas enviadas a MS. No afecta la búsqueda local ni apps básicas. | **Deshabilitar (Seguro / Verde)** |
| **`dmwappushservice`** | Device Management Wireless Application Protocol (WAP) Push Routing Service | Canal de telemetría y mensajes push de diagnóstico empresarial y sincronización MDM. | Nulo para usuarios domésticos y gaming. | **Deshabilitar (Seguro / Verde)** |
| **`WerSvc`** | Windows Error Reporting Service | Genera y transmite volcados de memoria y registros de choque a Microsoft tras un BSOD o error de aplicación. | Si se deshabilita por completo, no se pueden enviar reportes a Microsoft. (Pristine permite conservar minidumps locales sin subirlos). | **Modo Híbrido (Conservar local, no subir) (Seguro / Verde)** |
| **`diagnosticshub.standardcollector.service`** | Microsoft (R) Diagnostics Hub Standard Collector Service | Recolector de eventos para diagnóstico en tiempo real (utilizado a menudo por perfiles de desarrollo). | Ahorro de memoria. | **Manual / Deshabilitar (Seguro / Verde)** |
| **`DPS`** | Diagnostic Policy Service | Servicio de políticas de diagnóstico que detecta problemas de componentes. | **¡ADVERTENCIA!** Si se desactiva, el solucionador de problemas de red y audio deja de operar. | **Conservar en Automático (Riesgo Alto / Rojo)** |

---

## 3. Telemetría de Inteligencia Artificial y Nuevas Tecnologías (Win 11 23H2 / 24H2)

Con las últimas actualizaciones de Windows 11, Microsoft introdujo nuevas capas de rastreo basadas en IA:

### A. Windows Recall (Snapshots Locales y OCR)
- **Qué hace:** Toma capturas de pantalla continuas de la actividad del usuario cada pocos segundos, las almacena en una base de datos SQLite local y aplica OCR y modelos neuronales locales para permitir búsquedas.
- **Riesgo:** Si bien los datos residen localmente, es un vector de ataque gravísimo si malware o infostealers acceden a la base de datos de capturas.
- **Intervención Pristine:** Desactivación de las directivas de Recall mediante Group Policy:
  - `HKLM\SOFTWARE\Policies\Microsoft\Windows\WindowsAI\DisableAIDataAnalysis` = `1`
  - `HKCU\Software\Policies\Microsoft\Windows\WindowsCopilot\TurnOffWindowsCopilot` = `1`

### B. Búsqueda con Bing en el Menú Inicio
- **Qué hace:** Cada letra pulsada en la barra de búsqueda del menú Inicio envía una solicitud HTTP a los servidores de Bing con la pulsación del usuario y la IP, devolviendo resultados web irrelevantes cuando el usuario solo busca un archivo local.
- **Intervención Pristine:**
  - `HKCU\Software\Policies\Microsoft\Windows\Explorer\DisableSearchBoxSuggestions` = `1`
  - `HKCU\Software\Microsoft\Windows\CurrentVersion\Search\BingSearchEnabled` = `0`
  - **Beneficio:** Menú inicio instantáneo (latencia 0 ms) y cero tráfico saliente a Bing.

### C. Inking & Typing Personalization (Keylogger Diagnóstico)
- **Qué hace:** Recopila patrones de escritura en teclado físico y táctil para "mejorar el diccionario y reconocimiento de escritura".
- **Intervención Pristine:**
  - `HKCU\Software\Microsoft\InputPersonalization\RestrictImplicitInkCollection` = `1`
  - `HKCU\Software\Microsoft\InputPersonalization\RestrictImplicitTextCollection` = `1`
  - `HKCU\Software\Microsoft\Personalization\Settings\AcceptedPrivacyPolicy` = `0`

---

## 4. Tareas Programadas Silenciosas (Task Scheduler)

Windows 11 ejecuta periódicamente decenas de tareas en segundo plano que disparan picos de uso de CPU y disco:

```
Ruta en Task Scheduler:
\Microsoft\Windows\
  ├── Application Experience\
  │   ├── Microsoft Compatibility Appraiser   ──> Escanea todo el software instalado y compatibilidad
  │   ├── ProgramDataUpdater                  ──> Rastrea cambios en programas instalados
  │   └── StartupAppTask                      ──> Evalúa tiempos de inicio de apps
  ├── Customer Experience Improvement Program\
  │   ├── Consolidator                        ──> Agrupa métricas de uso y las envía
  │   └── UsbCeip                             ──> Registra dispositivos USB conectados
  ├── Autochk\
  │   └── Proxy                               ──> Envía métricas de integridad de disco a SQM
  ├── DiskDiagnostic\
  │   └── Microsoft-Windows-DiskDiagnosticDataCollector
  └── Feedback\
      └── Siuf\
          └── DmClient                        ──> Notificaciones de encuestas de satisfacción
```

> **Técnica de Pristine:** En lugar de borrar la tarea (lo cual causaría errores en `sfc /scannow`), Pristine invoca el método COM `ITaskFolder::GetTask()` y establece `ITaskDefinition::put_Enabled(VARIANT_FALSE)`. La tarea queda **deshabilitada** de forma limpia y transparente, lista para reactivarse si el usuario lo decide.

---

## 5. Bloqueo de Red y Firewall (Defensa en Profundidad)

Para garantizar que ningún paquete de telemetría escape incluso si un servicio se reactiva tras un parche menor:

### Endpoints Oficiales de Telemetría Bloqueados:
- `v10.events.data.microsoft.com`
- `v20.events.data.microsoft.com`
- `telemetry.microsoft.com`
- `watson.telemetry.microsoft.com`
- `feedback.microsoft.com`
- `diagnostics.support.microsoft.com`
- `activity.windows.com`

**Estrategia de Pristine:**
1. **Reglas de Windows Defender Firewall nativo**: Creación de reglas salientes (*Outbound Block Rules*) dirigidas a los ejecutables de diagnóstico (`%SystemRoot%\System32\diagtrack.dll`, `compattelrunner.exe`), sin necesidad de alterar el archivo `hosts` si el usuario no lo desea, manteniendo el DNS limpio.
2. Opción complementaria de filtrado en archivo `hosts` redirigiendo a `0.0.0.0` (opcional con advertencia).

---

## 6. Matriz Técnica de las 20 Directivas Implementadas en Pristine Core

A continuación se lista la especificación técnica de las 20 directivas implementadas en el catálogo de producción (`crates/pristine-core/src/catalog.rs`), con sus claves de registro, tareas asociadas, nivel de riesgo y comportamiento recomendado:

| # | ID de Directiva | Título | Categoría | Riesgo | Recomendado | Mecanismo Primario |
| :- | :--- | :--- | :--- | :--- | :--- | :--- |
| 1 | `diagtrack_utc` | Connected User Experiences (DiagTrack) | Telemetría | Seguro | Sí | SCM Service `DiagTrack` (Disabled) + `AllowTelemetry` = 0 |
| 2 | `ceip_tasks` | Customer Experience Improvement (CEIP) | Telemetría | Seguro | Sí | Task Scheduler `Consolidator`, `UsbCeip` |
| 3 | `advertising_id` | Identificador de Publicidad | Privacidad | Seguro | Sí | `HKCU\...\AdvertisingInfo\Enabled` = 0 |
| 4 | `bing_start_search` | Búsqueda Web con Bing en Inicio | Privacidad | Seguro | Sí | `DisableSearchBoxSuggestions` = 1, `BingSearchEnabled` = 0 |
| 5 | `windows_recall_ai` | Windows Recall y Capturas de IA | Privacidad | Seguro | Sí | `DisableAIDataAnalysis` = 1 |
| 6 | `copilot_ai` | Integración de Windows Copilot | Privacidad | Moderado | No | `TurnOffWindowsCopilot` = 1 |
| 7 | `inking_typing_telemetry`| Personalización de Escritura / Keylogger | Telemetría | Seguro | Sí | `RestrictImplicitTextCollection` = 1, `AcceptedPrivacyPolicy` = 0 |
| 8 | `delivery_optimization_p2p`| Optimización de Entrega P2P | Rendimiento | Seguro | Sí | `DODownloadMode` = 1 (LAN Only) |
| 9 | `multimedia_network_throttling`| Throttling Multimedia de Red | Rendimiento | Seguro | Sí | `NetworkThrottlingIndex` = 0xFFFFFFFF, `SystemResponsiveness` = 0 |
| 10| `wer_hybrid_error_reporting`| Reporte de Errores Híbrido | Telemetría | Seguro | Sí | `Disabled` = 1, `DontSendAdditionalData` = 1 |
| 11| `edge_telemetry` | Telemetría y Modo Segundo Plano de Edge | Telemetría | Seguro | Sí | `MetricsReportingEnabled` = 0, `StartupBoostEnabled` = 0 |
| 12| `cortana_speech_telemetry`| Reconocimiento de Voz en Nube y Cortana | Telemetría | Seguro | Sí | `AllowCortana` = 0, `HasAccepted` = 0 |
| 13| `activity_history_sync`| Historial de Actividades y Timeline Cloud| Privacidad | Seguro | Sí | `PublishUserActivities` = 0, `UploadUserActivities` = 0 |
| 14| `location_sensor_service`| Servicio de Sensores de Ubicación (lfsvc)| Privacidad | Moderado | No | SCM Service `lfsvc` (Disabled) + `SensorPermissionState` = 0 |
| 15| `tailored_experiences` | Experiencias Personalizadas y Consejos | Privacidad | Seguro | Sí | `TailoredExperiencesWithDiagnosticDataEnabled` = 0 |
| 16| `game_dvr_background` | Grabación de Fondo GameDVR y Latencia | Rendimiento | Seguro | Sí | `AppCaptureEnabled` = 0, `HistoricalCaptureEnabled` = 0 |
| 17| `cloud_clipboard_sync` | Portapapeles en la Nube (Cloud Sync) | Seguridad | Seguro | Sí | `AllowCrossDeviceClipboard` = 0 |
| 18| `app_diagnostics_access`| Acceso de Apps a Diagnósticos de Terceros| Seguridad | Seguro | Sí | `Value` = "Deny" en AppPrivacy |
| 19| `wifi_sense_sharing` | Wi-Fi Sense y Credenciales Compartidas | Seguridad | Seguro | Sí | `AutoConnectAllowedOEM` = 0 |
| 20| `remote_assistance_solicited`| Asistencia Remota No Solicitada | Seguridad | Seguro | Sí | `fAllowToGetHelp` = 0 |

---
*Documento mantenido bajo la dirección de SuperZonico para el ecosistema de código abierto Pristine.*
