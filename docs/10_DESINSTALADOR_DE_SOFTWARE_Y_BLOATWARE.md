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

## 4. Proceso de Desinstalación Atómica y Limpieza de Residuos

1. **Confirmación con Modal Nativo:** El usuario visualiza los detalles del programa (nombre, versión, tamaño, publicador).
2. **Punto de Seguridad VSS (Opcional recomendado):** Creación de un snapshot de Windows previo.
3. **Ejecución del Desinstalador Oficial:**
   - Para aplicaciones UWP: Invocación de `Remove-AppxPackage -Package <PackageFullName> -AllUsers`.
   - Para aplicaciones Win32: Ejecución del comando de desinstalación silenciosa (`QuietUninstallString`) o estándar (`UninstallString`).
4. **Inspección de Residuos:** Detección de carpetas huérfanas en `%AppData%`, `%LocalAppData%` y claves obsoletas tras la desinstalación.

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
