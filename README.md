# kore-sys-monitor 🚀

**`kore-sys-monitor`** es un monitor de sistema en tiempo real con interfaz de usuario en terminal (TUI) extremadamente ligero, moderno y multiplataforma (Linux, macOS y Windows). Diseñado en **Rust** con **Ratatui** y **sysinfo**, ofrece métricas en tiempo real sobre el rendimiento del hardware, memoria RAM/Swap, red, discos y un gestor interactivo de procesos.

---

## ✨ Características Principales

- 🚀 **Bajo Consumo de Recursos**: Poca memoria RAM (< 15 MB) y uso mínimo de CPU gracias a la recolección optimizada y caché de hardware.
- 🎨 **Sistema de Temas Dinámicos**: Alterna entre **Cyber Cyan**, **Catppuccin Mocha**, **Dracula** y **Monochrome Matrix** en tiempo real presionando `t`.
- 📐 **Diseño Responsivo (Breakpoints & 2-Líneas)**: Reorganización inteligente del layout y auto-wrap multi-línea en almacenamiento y redes para evitar cortes de información en cualquier tamaño de pantalla.
- 🔍 **Búsqueda & Filtrado de Procesos**: Presiona `/` para filtrar procesos en tiempo real por nombre, PID o comando.
- 📊 **Ordenamiento Interactivo**: Presiona `s` para cambiar la columna de ordenación (CPU, MEM, PID, Nombre) y `r` para invertir el orden.
- ⚡ **Gestor de Procesos Integrado**: Modal interactivo de confirmación (`Del` / `K`) para terminar procesos (`SIGTERM`/`SIGKILL`).
- 🚀 **Network Speed Test Integrado**: Presiona `e` para ejecutar pruebas en tiempo real de Ping/Latencia, Velocidad de Bajada (↓ Mbps) y Subida (↑ Mbps) sin bloquear el hilo principal.
- 📈 **Detalle Extendido de CPU**: Pestaña dedicada `[4] CPU Detail` con desgloses por núcleo, métricas promedio/mínimas/máximas e historial Sparkline.
- 🎮 **Telemetría Avanzada de GPU**: Pestaña dedicada `[5] GPU Detail` y resumen en Overview con soporte multi-proveedor (NVIDIA, AMD, Intel) para VRAM, clocks de núcleo/memoria, ventiladores, potencia (W), resolución/refresco y motores de cómputo/video.
- 💾 **Almacenamiento y Red**: Pestaña `[3] Storage & Net` con clasificación de medios (NVMe/SSD/HDD), telemetría SMART CrystalDisk, modelos de adaptadores de red, IP/GW/DNS y gráficos de tráfico RX/TX.
- 🛡️ **Restauración Garantizada del Terminal**: Custom panic hook y destructores limpios que aseguran que el terminal nunca quede desconfigurado.

---

## 🎹 Atajos de Teclado (Keybindings)

| Tecla / Combinación | Acción |
| :--- | :--- |
| `Tab` / `Shift+Tab` | Avanzar / Retroceder entre pestañas (`Overview`, `Processes`, `Storage & Net`, `CPU Detail`, `GPU Detail`). |
| `1`, `2`, `3`, `4`, `5` | Selección directa de pestaña. |
| `j` / `Down` | Mover selección al siguiente proceso. |
| `k` / `Up` | Mover selección al proceso anterior. |
| `PgUp` / `PgDown` | Desplazamiento por páginas de procesos (10 en 10). |
| `Home` / `End` | Ir al primer / último proceso de la lista. |
| `/` | Entrar en modo Búsqueda/Filtro de procesos. |
| `s` | Cambiar columna de ordenación (CPU% → MEM% → PID → Name). |
| `r` | Invertir el sentido de ordenación (Ascendente / Descendente). |
| `e` | Iniciar prueba de velocidad de red en tiempo real (**Speed Test**). |
| `t` | Cambiar el tema visual dinámicamente. |
| `Del` / `Delete` / `K` | Abrir modal de confirmación para terminar el proceso seleccionado. |
| `?` | Abrir / Cerrar ventana modal de ayuda. |
| `q` / `Ctrl+C` | Salir limpiamente de la aplicación. |

---

## 🛠️ Instalación y Compilación

### Requisitos
- [Rust](https://www.rust-lang.org/) (edición 2024 o reciente)

### Compilación Ejecución Local

```bash
# Clonar el repositorio
git clone https://github.com/ddrprz/kore-sys-monitor.git
cd kore-sys-monitor

# Ejecutar en modo desarrollo
cargo run

# Compilar binario de producción altamente optimizado (< 5 MB)
cargo build --release
```

---

## 🧪 Pruebas Unitarias

Para ejecutar la suite de pruebas unitarias automáticas:

```bash
cargo test
```

---

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Consulta el archivo [LICENSE](LICENSE) para más información.
