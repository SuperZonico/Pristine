# 🛡️ 05. Seguridad, Rollback Atómico y Resiliencia Extrema

> *"El mejor sistema de seguridad no es el que promete no fallar jamás, sino el que cuenta con un plan de rescate infalible y blindado ante cualquier imprevisto."*

---

## 1. El Sistema de Rollback de Pristine

Pristine no asume que nada sea infalible. Ante cualquier eventualidad, el usuario cuenta con **tres capas concéntricas de protección y marcha atrás**:

```
                  ┌──────────────────────────────────────────────┐
                  │      CAPA 1: Diff Journal Transaccional      │
                  │   (Reversión instantánea de valores en 1s)   │
                  └──────────────────────┬───────────────────────┘
                                         │ Si algo falla...
                                         ▼
                  ┌──────────────────────────────────────────────┐
                  │    CAPA 2: Punto de Restauración VSS (Win32) │
                  │     (Restaura el estado completo del OS)     │
                  └──────────────────────┬───────────────────────┘
                                         │ En caso de emergencia...
                                         ▼
                  ┌──────────────────────────────────────────────┐
                  │   CAPA 3: Modo de Rescate WinRE / SafeMode   │
                  │  (Script binario ejecutable desde consola)   │
                  └──────────────────────────────────────────────┘
```

---

## 2. Capa 1: Diff Journal Transaccional en Rust

Cada operación que modifica el registro de Windows o el estado de un servicio genera una entrada inmutable en el diario transaccional de Pristine:

```rust
pub struct RegistryChangeEntry {
    pub key_path: String,
    pub value_name: String,
    pub previous_type: Option<u32>,        // REG_DWORD, REG_SZ, etc. (None si no existía)
    pub previous_data: Option<Vec<u8>>,    // Valor previo exacto en bytes
    pub new_type: u32,
    pub new_data: Vec<u8>,
    pub timestamp_utc: u64,
}

pub struct TransactionSession {
    pub session_id: String,
    pub created_at: u64,
    pub description: String,
    pub changes: Vec<RegistryChangeEntry>,
    pub service_changes: Vec<ServiceStateChange>,
    pub task_changes: Vec<TaskStateChange>,
    pub sha256_checksum: [u8; 32],
}
```

### Propiedades del Diario:
- **Atómico**: Si una sesión de 15 ajustes falla en el ajuste #10, Pristine deshace automáticamente los 9 ajustes previos y devuelve el sistema a su estado intacto.
- **Verificado Criptográficamente**: El archivo de sesión se firma con SHA-256 para evitar que software malicioso externo adultere el historial de reversión.
- **Rollback en 1-Clic**: En la interfaz, el usuario puede pulsar "Deshacer sesión [Fecha/Hora]" y cada clave vuelve a su valor original o es eliminada si no existía.

---

## 3. Capa 2: Puntos de Restauración Nativos (VSS)

Antes de aplicar cualquier lote de optimizaciones o cambios en servicios de sistema, Pristine invoca la API Win32 de restauración:
```cpp
// Invocación segura mediante la API nativa de Windows
SRSetRestorePointW(&RestorePointInfo, &SMgrStatus);
```
- Se crea un punto con la etiqueta formal: `Pristine: Pre-Optimization Snapshot [ID]`.
- Si el servicio de Volume Shadow Copy (VSS) está deshabilitado por el usuario, Pristine advierte claramente antes de proceder y ofrece reactivarlo para máxima seguridad.

---

## 4. Capa 3: Modo de Rescate Autónomo (WinRE & Safe Mode)

Si por alguna razón ajena a Pristine (ejemplo: un corte de luz en pleno proceso de Windows Update o un driver defectuoso de terceros) la máquina experimentara un bucle de arranque:
- Pristine genera en `C:\ProgramData\Pristine\Rescue\` un script de recuperación autocontenido:
  - `Pristine-Emergency-Restore.bat` y un binario estático ligero `pristine-rescue.exe` (~1.5 MB, compilado sin dependencias externas).
- Puede ser ejecutado desde el entorno de recuperación de Windows (**WinRE / Símbolo del sistema de reparación**) o desde el Modo Seguro para restablecer todas las claves de registro modificadas al estado de fábrica de Windows.

---

## 5. Prevención de Vulnerabilidades (Hardening y Cero Exploits)

Para blindar la aplicación frente a atacantes e ingeniería inversa maliciosa:

1. **Mitigación de DLL Hijacking**:
   - Llamada inmediata al iniciar el proceso: `SetDefaultDllDirectories(LOAD_LIBRARY_SEARCH_SYSTEM32)`.
   - Llamada a `SetDllDirectoryW(L"")` para evitar la carga de DLLs fraudulentas ubicadas en el directorio de trabajo actual.
2. **Hardening de IPC (Named Pipes con SDDL Estricto)**:
   - Descriptor de seguridad que concede acceso únicamente al `SYSTEM` y al SID del usuario interactivo actual (`D:(A;;GA;;;SY)(A;;GA;;;BA)(A;;GA;;;OW)`).
   - Bloqueo total de conexiones remotas (`PIPE_REJECT_REMOTE_CLIENTS`).
3. **Protección de Memoria del Binario**:
   - Banderas de compilación activas en `Cargo.toml`:
     - ASLR (*Address Space Layout Randomization*) completo con alta entropía (`/HIGHENTROPYVA`).
     - DEP / NX (*Data Execution Prevention*).
     - Control Flow Guard (`/guard:cf`).
     - Eliminación de símbolos de depuración en compilaciones de producción (`strip = true`).
4. **Validación Exhaustiva de Tipos**:
   - Todo parseo de JSON, IPC o entradas de usuario utiliza esquemas estrictos sin deserialización polimórfica insegura.
