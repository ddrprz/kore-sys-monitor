# AGENTS.md - System Monitor (`kore-sys-monitor`)

## 1. Descripción
**`kore-sys-monitor`** es un monitor de sistema en tiempo real con interfaz de usuario en terminal (TUI) extremadamente ligero, moderno y multiplataforma (Windows, macOS y Linux). Diseñado con **Rust** y **Ratatui**, ofrece métricas detalladas sobre el rendimiento del hardware, uso de memoria, redes, discos y un gestor interactivo de procesos.

---

## 2. Stack Tecnológico
- **Lenguaje**: Rust (Edición 2024)
- **TUI Framework**: [`ratatui`](https://crates.io/crates/ratatui) (v0.30.2+)
- **Backend / Terminal Handling**: [`crossterm`](https://crates.io/crates/crossterm) (v0.29.0+)
- **Métricas del Sistema**: [`sysinfo`](https://crates.io/crates/sysinfo) (Mapeo multiplataforma de métricas de OS/Hardware)
- **Manejo de Eventos**: Asíncrono / Loop de eventos no bloqueante con hilos y canales (`std::sync::mpsc` o `tokio` si aplica)

---

## 3. Funcionalidades Clave
1. **Header & Información del Sistema**:
   - Hostname, Kernel, tiempo de actividad (Uptime), arquitectura CPU y versión de SO.
2. **Monitor de CPU**:
   - Gráfico/Sparkline histórico de uso global de CPU.
   - Barras de nivel por núcleo individual (Core load).
3. **Monitor de Memoria y Swap**:
   - Medidores (Gauges) de RAM consumida vs. disponible y memoria Swap.
   - Histórico visual de consumo.
4. **Discos e I/O de Almacenamiento**:
   - Tabla de particiones/puntos de montaje, espacio total, libre, usado (%) y tipo de sistema de archivos.
5. **Red y Ancho de Banda**:
   - Monitoreo en tiempo real de velocidad de descarga (RX) y subida (TX) con gráficos Sparkline.
6. **Gestor de Procesos Interactivo**:
   - Tabla con PID, Nombre, % CPU, % Memoria y Estado.
   - Ordenamiento por columna (CPU, Memoria, PID, Nombre).
   - Búsqueda/Filtrado dinámico en tiempo real (`/`).
   - Terminación/Signal de procesos (`k`).
7. **Navegación e Interfaz**:
   - Diseño modular con pestañas (Tabs) o paneles enfocables mediante teclado.
   - Tema oscuro elegante con alto contraste y compatibilidad con terminales ANSI / TrueColor.

---

## 4. AI Guidelines (Directrices para Agentes de IA)

Al editar o extender este código, los asistentes de IA deben seguir estrictamente las siguientes reglas:

1. **Compatibilidad Multiplataforma**:
   - Evitar llamadas directas al sistema operativo (`std::process::Command` específico de Unix/Linux).
   - Utilizar las abstracciones provistas por `sysinfo` para garantizar la compatibilidad transparente con Windows, Linux y macOS.

2. **Restauración y Limpieza del Terminal**:
   - Asegurar que cualquier error o pánico (`panic!`) o salida normal del programa **restaure siempre** el terminal a su estado original (`crossterm::terminal::disable_raw_mode()` y `LeaveAlternateScreen`).
   - Implementar un *custom panic hook* o destructores con `Drop` que limpien la pantalla.

3. **Rendimiento y No Bloqueo**:
   - El bucle principal de renderizado de la UI **nunca debe bloquearse** esperando I/O o cálculos pesados de `sysinfo`.
   - Las lecturas pesadas deben realizarse de forma periódica o en segundo plano manteniendo la tasa de refresco (Tick Rate) constante (ej. 250ms - 1000ms).

4. **Arquitectura Modular**:
   - Mantener separadas las responsabilidades:
     - `app.rs`: Estado de la aplicación y lógica de navegación.
     - `system.rs`: Módulo de recolección de métricas.
     - `event.rs`: Captura y propagación de eventos de teclado/redimensionamiento.
     - `ui/`: Módulos visuales dedicados para cada componente (CPU, Memoria, Procesos, Header, etc.).

5. **Manejo de Errores Robustos**:
   - Evitar `.unwrap()` o `.expect()` innecesarios en tiempo de ejecución. Retornar `Result<T, E>` y manejar fallos grácilmente.
