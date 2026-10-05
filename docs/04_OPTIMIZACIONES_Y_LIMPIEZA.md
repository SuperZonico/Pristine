# 🧹 04. Motor de Optimización, Limpieza Inteligente y Latencia

> *"Optimizar no es vaciar carpetas a lo loco; es liberar recursos estratégicos y minimizar latencia sin sabotear las cachés útiles del sistema."*

---

## 1. Mitos vs. Realidades de la Limpieza en Windows 11

Una de las mayores fallas de los programas de limpieza populares (como CCleaner o scripts de foros) es el borrado indiscriminado de archivos que Windows necesita para rendir bien:

| Elemento | ¿Qué hacen los limpiadores malos? | ¿Por qué es un grave error? | ¿Qué hace Pristine? |
| :--- | :--- | :--- | :--- |
| **Carpeta `Prefetch`** | Borran todos los archivos `.pf`. | El gestor de memoria de Windows usa Prefetch para pre-cargar páginas de ejecutables en RAM antes de que se pidan. Borrarlo hace que las apps abran **más lento** y aumente el desgaste del SSD. | **No se borra Prefetch.** Pristine educa al usuario sobre este mito. |
| **Caché de Miniaturas (Thumbs)** | Borran `thumbcache_*.db` constantemente. | Obliga al Explorador de Windows a re-escanear y renderizar cada miniatura de foto/video cada vez que abres una carpeta, consumiendo 100% de CPU. | Solo se ofrece purgar si el usuario reporta miniaturas corruptas o negras. |
| **Archivos Temporales Bloqueados** | Intentan forzar el borrado de archivos en uso en `%TEMP%`. | Provoca cuelgues en instaladores activos o programas en ejecución en segundo plano. | **Filtro Seguro de 24 Horas**: Pristine solo elimina temporales con timestamp de acceso superior a 24 horas y que no posean bloqueo de descriptor de archivo (`ERROR_SHARING_VIOLATION`). |

---

## 2. Áreas de Limpieza Segura de Pristine

### A. Almacén de Componentes de Windows (`WinSxS`)
- **El Peligro:** Intentar borrar manualmente archivos de `C:\Windows\WinSxS` destruye el sistema operativo y provoca BSOD inmediato.
- **La Solución Pristine:** Automatización mediante la API nativa de DISM de forma segura y supervisada:
  ```powershell
  Dism.exe /Online /Cleanup-Image /StartComponentCleanup /ResetBase
  ```
  Esto descarta versiones obsoletas de parches de Windows anteriores, liberando típicamente entre **4 GB y 12 GB** de espacio real en disco sin riesgo alguno.

### B. Optimización de Entrega de Windows Update (P2P Cache)
- **El Problema:** Windows 11 comparte por defecto tus actualizaciones de Windows con otros PCs desconocidos en Internet (Delivery Optimization), devorando ancho de banda y acumulando gigabytes en `C:\Windows\SoftwareDistribution\DeliveryOptimization`.
- **Acción Pristine:**
  - Purgado seguro de la caché acumulada.
  - Configuración de la directiva `DODownloadMode` a `0` (Solo HTTP directo de Microsoft) o `1` (Solo red local doméstica), eliminando la subida a Internet.

### C. Caché de Shaders de GPU (DirectX / NVIDIA / AMD)
- **Cuándo limpiarla:** Tras actualizar drivers de la tarjeta gráfica, es común experimentar micro-stutters o caídas de FPS causadas por shaders antiguos compilados con versiones previas del driver.
- **Acción Pristine:**
  - Detección de carpetas de caché de sombreadores (`%LOCALAPPDATA%\NVIDIA\DXCache`, `%LOCALAPPDATA%\AMD\DxCache`, `D3DSCache`).
  - Purga controlada cuando no hay juegos ni aceleración 3D activa.

### D. Volcados y Reportes de Error de Windows (`WER`)
- Purgado seguro de minidumps antiguos en `C:\ProgramData\Microsoft\Windows\WER\ReportArchive` y `C:\Windows\Minidump` que superen los 30 días de antigüedad.

---

## 3. Optimizaciones de Latencia de Sistema (DPC / ISR y Gaming)

Para usuarios que juegan a nivel competitivo, producen música o editan video en tiempo real, los picos de latencia DPC (*Deferred Procedure Calls*) causan pérdida de paquetes de audio y micro-congelamientos (*stutter*).

### A. Mitigación de Throttling Multimedia de Red
Por defecto, Windows limita el tráfico de red de fondo no multimedia cuando se reproduce audio o video para priorizar la reproducción local:
- `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion\Multimedia\SystemProfile`
  - `NetworkThrottlingIndex` = `0xFFFFFFFF` (Desactiva el estrangulamiento artificial de red para permitir máximo throughput constante).
  - `SystemResponsiveness` = `0` (Garantiza que el planificador de hilos no limite la prioridad de procesos no multimedia al 20%).

### B. Desactivación de Algoritmo Nagle en Interfaces de Juego (TCP NoDelay)
- Para conexiones de baja latencia en juegos multijugador, Pristine permite activar de forma granular `TcpAckFrequency = 1` y `TCPNoDelay = 1` en los adaptadores de red activos seleccionados por el usuario.

### C. Modo de Ahorro Transparente CompactOS
- Utilizando la API nativa de Windows `WOF` (Windows Overlay Filter), Pristine permite activar compresión XPRESS4K/LZX transparente para el sistema operativo en unidades SSD con espacio reducido:
  ```powershell
  compact.exe /CompactOS:always
  ```
  Esto reduce el tamaño de Windows en ~4 GB a 8 GB con un impacto de CPU prácticamente imperceptible en procesadores modernos gracias a la descompresión ultrarrápida.

---

## 4. Estado de Energía y Modos de Rendimiento

Pristine expone al usuario la gestión de planes de energía ocultos de Windows 11:
1. **Desbloqueo del Plan "Máximo Rendimiento" (Ultimate Performance)**:
   - Creado originalmente para servidores y estaciones de trabajo de alta gama, desactiva micro-latencias de cambio de estado de CPU (C-states agresivos).
2. **Control Inteligente de Hibernación (`hiberfil.sys`)**:
   - Si el usuario tiene un PC de sobremesa con 32 GB o 64 GB de RAM y no utiliza el modo de hibernación rápida, el archivo `hiberfil.sys` puede ocupar hasta el 40-70% de la RAM en el SSD.
   - Pristine permite cambiar a `powercfg /h /type reduced` (conserva inicio rápido ocupando solo ~4GB) o desactivarlo completamente si el usuario lo prefiere, recuperando decenas de gigabytes.
