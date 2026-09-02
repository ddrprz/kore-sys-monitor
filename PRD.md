# PRD.md - Documento de Requisitos del Producto (`kore-sys-monitor`)

## 1. Visión General del Producto
**`kore-sys-monitor`** es un monitor de sistema multiplataforma (Windows, macOS y Linux) basado en terminal (TUI) diseñado en **Rust**. Proporciona métricas de hardware y red en tiempo real, junto con un gestor interactivo de procesos, ofreciendo una alternativa ultra-rápida, ligera y estéticamente superior a herramientas tradicionales como `top` o `htop`.

### Objetivos Clave
- **Bajo Consumo de Recursos**: Operar con `< 15 MB` de RAM y `< 1%` de uso continuo de CPU.
- **Portabilidad Nativa**: Funcionar sin modificaciones en Linux, macOS y Windows usando el mismo codebase.
- **Experiencia de Usuario Premium**: Interfaz fluida a 60 FPS, cero parpadeos, temas modernos y controles intuitivos.

---

## 2. Público Objetivo y Casos de Uso

### Público Objetivo
- **Desarrolladores de Software**: Monitoreo de consumo de recursos durante pruebas y builds.
- **Administradores de Sistemas y DevOps**: Inspección rápida por SSH o servidores locales.
- **Power Users / Entusiastas de Terminal**: Usuarios que prefieren herramientas eficientes basadas en CLI/TUI.

### Casos de Uso Principales
1. Identificar y finalizar procesos "colgados" o fuera de control que consuman excesiva CPU/Memoria.
2. Supervisar la velocidad de descarga/subida de red durante transferencias grandes.
3. Verificar la disponibilidad y ocupación de espacio en discos y particiones montadas.
4. Inspeccionar la carga distribuida entre los núcleos individuales de la CPU.

---

## 3. Requisitos Funcionales (Functional Requirements)

| ID | Módulo | Descripción | Prioridad |
| :--- | :--- | :--- | :--- |
| **FR-1** | **Header Info & IP** | Muestra Dirección IP Principal, Hostname, Kernel, Versión de SO, Arquitectura y Tiempo de actividad (Uptime). | Alta (MVP) |
| **FR-2** | **CPU Global & History** | Visualización en tiempo real del uso global de CPU (%) con gráfico histórico Sparkline. | Alta (MVP) |
| **FR-3** | **CPU Per-Core** | Desglose por barras de carga individual para cada núcleo o hilo lógico de la CPU. | Media |
| **FR-4** | **Memoria & Swap** | Medidores (Gauges) de RAM usada, disponible y Swap consumido con porcentajes. | Alta (MVP) |
| **FR-5** | **Almacenamiento & Temporales** | Pestaña dedicada `[3] Storage` con particiones, salud SMART y sección de archivos temporales (%TEMP%, Windows Temp, Prefetch, etc.) con peso total y cantidad de archivos en segundo plano (`t`). | Alta (MVP) |
| **FR-6** | **Red Simplificada & Speed Test**| Pestaña dedicada `[4] Network` con tipo de conexión (WiFi/Cable), Nombre de red (SSID), Gateway, IP y Speed Test (`e`). | Alta |
| **FR-7** | **Tabla de Procesos** | Lista interactiva de procesos con PID, Usuario, Nombre, % CPU, % Memoria y Estado. | Alta (MVP) |
| **FR-8** | **Filtro de Procesos** | Búsqueda dinámica en tiempo real al presionar `/` para filtrar por nombre o PID. | Alta |
| **FR-9** | **Ordenamiento** | Cambio interactivo de columna de ordenación (CPU, Memoria, PID, Nombre) con `s` y `r`. | Alta |
| **FR-10**| **Terminación (Kill)**| Ventana modal de confirmación al presionar `k` para enviar señal `SIGTERM` / `SIGKILL`. | Alta |
| **FR-11**| **Pestañas & Nav** | Navegación entre 6 vistas (Overview, Procesos, Storage, Network, CPU Detail, GPU Detail) con `Tab` o `1-6`.| Alta |
| **FR-12**| **Ayuda Modal** | Ventana emergente al presionar `?` detallando los atajos de teclado y ayuda. | Baja |
| **FR-13**| **GPU Telemetry** | Pestaña `[6] GPU Detail` con telemetría multi-proveedor (NVIDIA, AMD, Intel) para VRAM, clocks, ventiladores, temperatura y potencia. | Alta |
| **FR-14**| **Speed Test** | Prueba interactiva en segundo plano de latencia (Ping), velocidad de bajada (↓ Mbps) y subida (↑ Mbps) con `e`. | Alta |

---

## 4. Requisitos No Funcionales (Non-Functional Requirements)

### NFR-1: Rendimiento y Eficiencia
- **Tiempo de Inicio**: Inicio e interactividad en menos de 100ms.
- **Footprint en Memoria**: Consumo de memoria RAM `< 15 MB`.
- **Uso de CPU**: Consumo propio de CPU en reposo `< 1.0%`.
- **Renderizado**: Actualización a ~60 FPS para eventos de UI (teclado/scroll) y tasa de recolección de métricas a 500ms - 1000ms.

### NFR-2: Multiplataforma y Compatibilidad
- Soporte transparente de compilación para `x86_64` y `aarch64` en Windows 10/11, macOS (Intel & Apple Silicon) y distribuciones Linux (glibc y musl).
- Compatibilidad con emuladores de terminal modernos (Alacritty, Kitty, WezTerm, iTerm2, Windows Terminal, Tmux).

### NFR-3: Estabilidad y Seguridad
- **Restauración del Terminal**: En caso de pánico o cierre inesperado, el programa debe restaurar el modo `raw` de la terminal (`disable_raw_mode` y `LeaveAlternateScreen`) para evitar dejar la terminal inusable.
- **Permisos Gráciles**: Manejo adecuado cuando no se tienen permisos de superusuario para finalizar procesos de otros usuarios.

---

## 5. Criterios de Aceptación y Métricas de Éxito

1. **Binario Compacto**: El ejecutable compilado en modo `release` (con `strip`) no debe superar los 5 MB.
2. **Navegación Fluida**: El desplazamiento por la lista de procesos debe ser instantáneo y sin retraso perceptible.
3. **Manejo de Cierre Limpio**: Salida garantizada con `q` o `Ctrl+C` dejando el cursor y el buffer de la terminal intactos.
4. **Cero Dependencias C complejas**: Fácil instalación vía `cargo install kore-sys-monitor`.

---

## 6. Plan de Fases / Roadmap

```
  Fase 1: MVP Core
  ├── Configuración del proyecto, ratatui + sysinfo
  ├── Header de Sistema & Gauges de Memoria/Swap
  └── Tabla inicial de Procesos sin ordenamiento

  Fase 2: Interactividad & Gestor de Procesos
  ├── Ordenamiento dinámico por columna (s / r)
  ├── Filtro/Búsqueda de procesos en tiempo real (/)
  └── Modal de confirmación para eliminar proceso (k)

  Fase 3: Métricas Avanzadas
  ├── Gráficos Sparkline de CPU global y per-core
  └── Monitoreo de red RX/TX en tiempo real

  Fase 4: Pulido & Release
  ├── Sistema de Temas (Cyber Cyan, Dracula, Catppuccin)
  └── Testeo multiplataforma y CI/CD de compilación
```
