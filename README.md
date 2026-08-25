# kore-sys-monitor 🚀

**`kore-sys-monitor`** es un monitor de sistema en tiempo real con interfaz de usuario en terminal (TUI) extremadamente ligero, moderno y multiplataforma (Linux, macOS y Windows). Diseñado en **Rust** con **Ratatui** y **sysinfo**, ofrece métricas en tiempo real sobre el rendimiento del hardware, memoria RAM/Swap, red, discos y un gestor interactivo de procesos.

---

## ✨ Características Principal

- 🚀 **Bajo Consumo de Recursos**: Poca memoria RAM (< 15 MB) y uso mínimo de CPU.
- 🎨 **Sistema de Temas Dinámicos**: Alterna entre **Cyber Cyan**, **Catppuccin Mocha**, **Dracula** y **Monochrome Matrix** en tiempo real presionando `t`.
- 📐 **Diseño Responsivo (Breakpoints)**: Reorganización inteligente del layout para pantallas compactas, estándar y ultra-wide (3 columnas).
- 🔍 **Búsqueda & Filtrado de Procesos**: Presiona `/` para filtrar procesos en tiempo real por nombre, PID o comando.
- 📊 **Ordenamiento Interactivo**: Presiona `s` para cambiar la columna de ordenación (CPU, MEM, PID, Nombre) y `r` para invertir el orden.
- ⚡ **Gestor de Procesos Integrado**: Modal interactivo de confirmación (`Del` / `K`) para terminar procesos (`SIGTERM`/`SIGKILL`).
- 📈 **Detalle Extendido de CPU**: Pestaña dedicada `[4] CPU Detail` con desgloses por núcleo, métricas promedio/mínimas/máximas e historial Sparkline.
- 🛡️ **Restauración Garantizada del Terminal**: Custom panic hook y destructores limpios que aseguran que el terminal nunca quede desconfigurado.

---

## 🎹 Atajos de Teclado (Keybindings)

| Tecla / Combinación | Acción |
| :--- | :--- |
| `Tab` / `Shift+Tab` | Avanzar / Retroceder entre pestañas (`Overview`, `Processes`, `Storage & Net`, `CPU Detail`). |
| `1`, `2`, `3`, `4` | Selección directa de pestaña. |
| `j` / `Down` | Mover selección al siguiente proceso. |
| `k` / `Up` | Mover selección al proceso anterior. |
| `PgUp` / `PgDown` | Desplazamiento por páginas de procesos. |
| `Home` / `End` | Ir al primer / último proceso de la lista. |
| `/` | Entrar en modo Búsqueda/Filtro de procesos. |
| `s` | Cambiar columna de ordenación (CPU% → MEM% → PID → Name). |
| `r` | Invertir el sentido de ordenación (Ascendente / Descendente). |
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
