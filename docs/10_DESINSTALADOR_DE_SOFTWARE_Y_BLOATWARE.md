# 📦 10. Desinstalador de Software, Bloatware y Escudo de Red

> *"A diferencia de los desinstaladores comerciales cargados de adware y procesos zombies en segundo plano, Pristine ofrece una gestión de software pura, nativa en Rust, 100% transparente y orientada a la soberanía del usuario."*

---

## 1. Filosofía: Por qué Pristine supera a herramientas como IObit Uninstaller

Herramientas tradicionales como IObit Uninstaller o Revo Uninstaller suelen arrastrar problemas graves en entornos modernos:
- **Procesos en segundo plano:** Servicios residentes permanentes que monitorean instalaciones y devoran RAM.
- **Telemetría propia y publicidad:** Envío de estadísticas de uso y banners para vender licencias "Pro".
- **Falsos positivos de limpieza:** Borrado imprudente de entradas compartidas en el registro que corrompen otras aplicaciones.

### El Estándar Pristine:
1. **0% Residencia en Memoria:** Cuando cierras Pristine, no queda ni un solo servicio, proceso ni driver en ejecución.
2. **Código Abierto y Transparente:** Cada llamada al registro o comando de desinstalación es auditable.
3. **Punto de Restauración Previo (VSS):** Antes de desinstalar aplicaciones críticas, se genera un snapshot de protección.
4. **Protección de Componentes del Sistema:** Identificación estricta de runtimes esenciales (Visual C++, DirectX, WebView2, Tienda de Windows) para hacer imposible que el usuario rompa su sistema operativo por accidente.

---

## 2. Arquitectura de Detección de Software

Pristine analiza de forma concurrente y ultrarrápida (< 20 ms) tres fuentes principales del sistema operativo:

### A. Registro Win32 Nativo (64-bit y 32-bit)
Se inspeccionan las siguientes colmenas del registro:
- `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall` (Programas globales 64-bit).
- `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall` (Programas 32-bit).
- `HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall` (Programas instalados a nivel de usuario, ej. VS Code, Chrome User, Telegram).

Para cada subclave se extraen con validación de tipo:
- `DisplayName`: Nombre legible del programa.
- `DisplayVersion`: Versión instalada.
- `Publisher`: Desarrollador o entidad firmante.
- `InstallDate`: Fecha de instalación en formato normalizado.
- `EstimatedSize`: Tamaño en disco reportado en MB.
- `UninstallString` / `QuietUninstallString`: Comando oficial del desinstalador del proveedor.
- `SystemComponent`: Bandera que identifica dependencias de Windows.

### B. Aplicaciones Modernas de Windows 11 (UWP / Appx Packages)
Pristine consulta los paquetes aprovisionados en el sistema mediante la API de Appx para identificar bloatware preinstalado por Microsoft y fabricantes OEM:
- `Microsoft.BingNews` (Noticias y Feed de Intereses).
- `Microsoft.BingWeather` (El Tiempo).
- `Microsoft.GetHelp` (Obtener Ayuda).
- `Microsoft.Getstarted` (Sugerencias / Consejos).
- `Microsoft.MicrosoftSolitaireCollection` (Colección de Solitario).
- `Microsoft.People` (Contactos).
- `Microsoft.Todos` (Microsoft To-Do).
- `Microsoft.WindowsFeedbackHub` (Centro de Comentarios).
- `Microsoft.YourPhone` (Enlace Móvil / Phone Link).
- `MicrosoftTeams` (Teams personal integrado).
- `Clipchamp.Clipchamp` (Editor de video Clipchamp).
- `Microsoft.549981C3F5F10` (Cortana).
- `Microsoft.GamingApp` / `Microsoft.XboxGamingOverlay` (Xbox Game Bar, opcional para no gamers).

---

## 3. Matriz de Clasificación y Seguridad

Pristine etiqueta cada paquete con un nivel de riesgo y categoría:

| Tipo | Identificador | Comportamiento en Pristine |
| :--- | :--- | :--- |
| **Bloatware Recomendado** | `bloatware` | Apps preinstaladas no críticas que consumen RAM y red. Desinstalación recomendada en 1 clic. |
| **Aplicación de Usuario** | `user` | Software instalado por el usuario (navegadores, suites ofimáticas, IDEs, clientes de juegos). |
| **Sistema Protegido** | `system` | Runtimes de C++, bibliotecas de Windows, Microsoft Store. **Bloqueados para desinstalación** para garantizar cero roturas de sistema. |

---

## 4. Proceso de Desinstalación Atómica y Motor de Limpieza de Residuos Huérfanos

A diferencia de los desinstaladores convencionales que dejan gigabytes de basura en disco y cientos de claves huérfanas en el registro, Pristine implementa un flujo de limpieza profunda de 4 fases:

### Fase 1: Confirmación Visual y Control de Seguridad
- El usuario visualiza la ficha técnica completa del programa (nombre, desarrollador, versión, arquitectura, tamaño aproximado y comando).
- Si la aplicación es un componente crítico del sistema (DirectX, VC++ Runtimes, WebView2, Tienda de Windows), el botón de desinstalación se desactiva permanentemente para prevenir fallos catastróficos.
- Opcional: Generación previa de un punto de restauración VSS (`create_system_restore_point`).

### Fase 2: Ejecución Aislada de la Rutina Oficial
- **Paquetes UWP de Windows 11:** Se invoca de forma limpia PowerShell sin perfiles de usuario (`-NoProfile -Command Remove-AppxPackage -Package '<PackageFullName>' -AllUsers`), eliminando el paquete para todos los usuarios del equipo.
- **Programas Win32 Tradicionales:** Se ejecuta de forma prioritaria la desinstalación silenciosa (`QuietUninstallString`) si está disponible, o el desinstalador estándar (`UninstallString`) con flags de aislamiento.

### Fase 3: Escaneo Quirúrgico de Residuos (`scan_app_residuals`)
Inmediatamente tras la desinstalación (o a petición del usuario mediante el botón **"Buscar Residuos"**), el motor nativo de Rust ejecuta un escaneo multi-objetivo:
1. **Sanitización de Tokens (`extract_search_tokens`):**
   - Extrae palabras clave significativas del nombre de la aplicación, eliminando versiones, arquitecturas (`x64`, `x86`), sufijos genéricos y palabras comunes (`setup`, `installer`, `pack`, etc.).
   - Aplica una **Lista Negra Global Estricta** (`GLOBAL_BLACKLIST_TOKENS`) que prohíbe de forma inviolable buscar o tocar términos como `windows`, `system32`, `syswow64`, `microsoft`, `defender`, `explorer`, `driver`, `intel`, `amd`, `nvidia`, `temp`, `desktop`, `documents`, `programdata`, `appdata`, etc.
2. **Inspección de 6 Ubicaciones Clave del Sistema de Archivos:**
   - `%APPDATA%` (Archivos de configuración de roaming).
   - `%LOCALAPPDATA%` (Cachés, datos locales y bases de datos SQLite).
   - `%LOCALAPPDATA%\Programs` (Instalaciones modernas a nivel de usuario).
   - `%ProgramData%` (Datos compartidos de aplicaciones globales).
   - `C:\Program Files` (Carpetas residuales vacías o con logs tras desinstalar).
   - `C:\Program Files (x86)` (Carpetas residuales de software de 32 bits).
   - Cálculo recursivo exacto del tamaño en bytes para informar al usuario de cuánto espacio recuperará.
3. **Inspección de Colmenas del Registro Win32:**
   - `HKEY_CURRENT_USER\Software\<AppName>`
   - `HKEY_LOCAL_MACHINE\SOFTWARE\<AppName>`
   - `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\<AppName>`

### Fase 4: Purgado Atómico y Seguro (`clean_residuals`)
- Pristine presenta al usuario una lista detallada con cada carpeta y clave encontrada y el peso total recuperable en MB.
- **Guardas Inviolables de Eliminación:**
  - En disco: La ruta debe existir, ser un directorio, no ser raíz (`C:\`) y contar con al menos 3 componentes de ruta para impedir cualquier eliminación imprudente.
  - En registro: La clave debe tener al menos un prefijo y subclave válida (imposible borrar `Software` o ramas del sistema) y se purga de forma recursiva con la API nativa de Win32 `RegDeleteTreeW`.
- Tras la confirmación, se destruyen todos los rastros sin requerir reinicio y el inventario se actualiza automáticamente.

---

## 5. Escudo de Red y Herramientas Rápidas

Complementando la privacidad del sistema, Pristine incorpora utilidades de red de grado quirúrgico:

### A. Purgado de Caché DNS (`Flush DNS`)
- Vacía la tabla de resolución DNS local de Windows de forma inmediata.
- Útil tras deshabilitar telemetría o modificar el archivo hosts para asegurar que ninguna conexión previa siga en memoria.

### B. Escudo DNS Local vía Archivo `HOSTS` (Sinkhole 0.0.0.0)
- Inyecta un bloque limpio delimitado (`# === BEGIN PRISTINE TELEMETRY SHIELD ===`) en `C:\Windows\System32\drivers\etc\hosts`.
- Mapea más de 40 dominios de telemetría, diagnósticos de Office, Edge y Windows a `0.0.0.0`.
- El tráfico hacia estos servidores muere localmente en 0 microsegundos antes de salir a la tarjeta de red.
- Reversible limpiamente en 1 clic sin alterar las entradas existentes del usuario.

### C. Reinicio Limpio del Explorador de Windows (`Restart Explorer`)
- Reinicia `explorer.exe` sin requerir reinicio del equipo, recargando instantáneamente cambios en el menú inicio y shell.
