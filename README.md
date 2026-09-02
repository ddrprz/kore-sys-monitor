# kore-sys-monitor 🚀

**`kore-sys-monitor`** es un monitor de sistema en tiempo real con interfaz de usuario en terminal (TUI) extremadamente ligero, moderno y multiplataforma (Linux, macOS y Windows). Diseñado en **Rust** con **Ratatui** y **sysinfo**, ofrece métricas en tiempo real sobre el rendimiento del hardware, memoria RAM/Swap, red, discos y un gestor interactivo de procesos.

---

## ✨ Características Principales

- 🚀 **Bajo Consumo de Recursos**: Poca memoria RAM (< 15 MB) y uso mínimo de CPU gracias a la recolección optimizada y caché de hardware.
- 🌐 **Header con Dirección IP Activa**: Visualización destacada de la IP principal del sistema directamente en la cabecera.
- 🎨 **Sistema de Temas Dinámicos**: Alterna entre **Cyber Cyan**, **Catppuccin Mocha**, **Dracula** y **Monochrome Matrix** en tiempo real presionando `t`.
- 📐 **Diseño Responsivo (Breakpoints & 2-Líneas)**: Reorganización inteligente del layout y auto-wrap multi-línea en almacenamiento y redes para evitar cortes de información en cualquier tamaño de pantalla.
- 🔍 **Búsqueda & Filtrado de Procesos**: Presiona `/` para filtrar procesos en tiempo real por nombre, PID o comando.
- 📊 **Ordenamiento Interactivo**: Presiona `s` para cambiar la columna de ordenación (CPU, MEM, PID, Nombre) y `r` para invertir el orden.
- ⚡ **Gestor de Procesos Integrado**: Modal interactivo de confirmación (`Del` / `K`) para terminar procesos (`SIGTERM`/`SIGKILL`).
- 💾 **Almacenamiento Dedicado, Temporales & Gráfico Donut**: Pestaña dedicada `[3] Storage` con clasificación de medios (NVMe/SSD/HDD), telemetría S.M.A.R.T., porcentaje de salud, particiones montadas, gráfico visual de pastel/donut de cuotas de espacio y limpieza segura e interactiva de archivos temporales (`%TEMP%`, `Windows Temp`, `Prefetch`, `Crash Dumps`, etc.) en segundo plano.
- 📶 **Red Simplificada & Speed Test**: Pestaña dedicada `[4] Network` con detección clara de **WiFi** o **Cable (Ethernet)**, nombre de la red (SSID), puerta de enlace (**Gateway**), gráficos de tráfico RX/TX y **Speed Test** interactivo (`e`).
- 📈 **Detalle Extendido de CPU**: Pestaña dedicada `[5] CPU Detail` con desgloses por núcleo, métricas promedio/mínimas/máximas e historial Sparkline.
- 🎮 **Telemetría Avanzada de GPU**: Pestaña dedicada `[6] GPU Detail` y resumen en Overview con soporte multi-proveedor (NVIDIA, AMD, Intel) para VRAM, clocks de núcleo/memoria, ventiladores, potencia (W), resolución/refresco y motores de cómputo/video.
- 🛡️ **Restauración Garantizada del Terminal**: Custom panic hook y destructores limpios que aseguran que el terminal nunca quede desconfigurado.

---

## 🎹 Atajos de Teclado (Keybindings)

| Tecla / Combinación | Acción |
| :--- | :--- |
| `Tab` / `Shift+Tab` | Avanzar / Retroceder entre pestañas (`Overview`, `Processes`, `Storage`, `Network`, `CPU Detail`, `GPU Detail`). |
| `1`, `2`, `3`, `4`, `5`, `6` | Selección directa de pestaña (`1:Overview`, `2:Processes`, `3:Storage`, `4:Network`, `5:CPU`, `6:GPU`). |
| `j` / `Down` | Mover selección al siguiente proceso. |
| `k` / `Up` | Mover selección al proceso anterior. |
| `PgUp` / `PgDown` | Desplazamiento por páginas de procesos (10 en 10). |
| `Home` / `End` | Ir al primer / último proceso de la lista. |
| `/` | Entrar en modo Búsqueda/Filtro de procesos. |
| `s` | Cambiar columna de ordenación (CPU% → MEM% → PID → Name). |
| `r` | Invertir el sentido de ordenación (Ascendente / Descendente). |
| `e` | Iniciar prueba de velocidad de red en tiempo real (**Speed Test**). |
| `u` (en Storage) | Re-escanear archivos temporales y caché del sistema en segundo plano. |
| `c` (en Storage) | Abrir modal interactivo para limpieza segura de archivos temporales. |
| `t` | Cambiar el tema visual dinámicamente (Cyber Cyan, Catppuccin, Dracula, etc.). |
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
