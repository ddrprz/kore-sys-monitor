# DESIGN.md - Guía de Diseño y UI/UX de `kore-sys-monitor`

## 1. Filosofía de Diseño
**`kore-sys-monitor`** busca ofrecer una experiencia visual **ultra-moderna, fluida y de alto impacto** dentro de la terminal, inspirándose en herramientas de referencia como `btop++` y `htop`, con una estética cyberpunk/dark elegante.

- **Legibilidad y Jerarquía Visual**: Uso cromático dinámico basado en umbrales (Verde < 60%, Amarillo 60-85%, Rojo > 85%).
- **Adaptabilidad Responsiva (Dynamic Breakpoints)**: Reorganización inteligente del layout según las dimensiones de la terminal (Small < 80x24, Medium 80-40x120, Ultra-wide > 120 cols).
- **Tipografía y Caracteres Unicode**: Uso de bloques Braille (`⡀⣀⣠⣤⣤⣶`), barras de nivel sólido (`█▓▒░`) y bordes suaves de línea doble o redondeados (`╭─╮│╰─╯`).
- **Cero Parpadeo (Flicker-Free)**: Renderizado optimizado con doble buffer en Ratatui y procesamiento asíncrono no bloqueante.

---

## 2. Temas y Paleta de Colores (Color Palettes)

`kore-sys-monitor` soportará temas dinámicos con un tema principal **Cyberpunk Cyan**:

### Tema Principal: *Cyber Cyan (Default)*
| Token / Rol | Color ANSI / Hex | Aplicación |
| :--- | :--- | :--- |
| **Primary Accent** | Cyan (`#00E5FF` / `Color::Cyan`) | Títulos de paneles, pestaña activa, bordes enfocados. |
| **Secondary Accent** | Neon Magenta (`#FF4081` / `Color::Magenta`) | Métricas de Swap, Subida de Red (TX), highlights. |
| **Success / Low Load** | Emerald Green (`#00E676` / `Color::Green`) | CPU/RAM < 60%, Descarga de Red (RX), estado saludable. |
| **Warning / Med Load** | Bright Amber (`#FFD600` / `Color::Yellow`) | Uso de CPU/RAM entre 60% y 85%, espacio en disco > 75%. |
| **Critical / High Load** | Crimson Red (`#FF1744` / `Color::Red`) | Uso > 85%, errores del sistema, modal de matar proceso. |
| **Background / Panels** | Reset / Dark (`#121212` / `Color::Reset`) | Fondo principal de terminal. |
| **Borders & Muted Text**| Charcoal Gray (`#546E7A` / `Color::DarkGray`)| Bordes inactivos, leyendas y marcas de tiempo. |

### Temas Adicionales Planificados:
1. **Catppuccin Mocha**: Tonos pastel (Lavender, Mauve, Sapphire, Peach).
2. **Dracula Theme**: Violeta, Rosado, Cyan y Verde pastel.
3. **Monochrome Matrix**: Tonos de Verde Fósforo clásico.

---

## 3. Mockup ASCII & Layout Grid

### A. Estructura General (Screen Wireframe)

```
╭─ kore-sys-monitor v0.1.0 ──────────────────── Host: arch-linux │ Kernel: 6.10.3 │ Uptime: 4h 12m ─╮
│ [1] Overview  │  [2] Processes  │  [3] Storage & Net  │  [4] CPU Detail                        │
├────────────────────────────────────────────────────────────────────────────────────────────────┤
│ ┌─ CPU Global [ 38.5% ] ─────────────────────┐ ┌─ Memory & Swap ──────────────────────────────┐ │
│ │ 3.8GHz  [████████████░░░░░░░░░░░░░░] 38%   │ │ RAM:  11.4GB / 31.8GB [████████░░░░░░░] 35.8% │ │
│ │ History:  ▂▃▅▇█▇▅▄▃▂  ▂▃▄▅▆▇█▇▅▄▃          │ │ Swap:  0.2GB /  8.0GB [█░░░░░░░░░░░░░░]  2.5% │ │
│ └────────────────────────────────────────────┘ └──────────────────────────────────────────────┘ │
│ ┌─ Disks & Mounts ───────────────────────────┐ ┌─ Network Bandwidth ──────────────────────────┐ │
│ │ Mount    Type  Total   Used    Free   %    │ │ RX:  1.25 MB/s  ⡀⣀⣠⣤⣦⣶⣦⣤⣄⣀             │ │
│ │ /        ext4  500GB  120GB   380GB [24%]  │ │ TX:  142 KB/s   ⡀⣀⣠⣤⣶⣦⣤⣄⣀                │ │
│ │ /home    btrfs   1TB  450GB   550GB [45%]  │ │ Total RX: 4.8 GB │ Total TX: 612 MB           │ │
│ └────────────────────────────────────────────┘ └──────────────────────────────────────────────┘ │
│ ┌─ Top Processes (Filter: none) ─────────────────────────────────────────────────────────────┐ │
│ │  PID   USER      NAME            CPU%   MEM%   STATE   COMMAND                               │ │
│ │> 1420  ddrprz    firefox         14.2%  8.5%   Running /usr/lib/firefox/firefox              │ │
│ │  3105  root      dockerd          4.1%  2.1%   Running /usr/bin/dockerd                      │ │
│ │  892   ddrprz    kore-sys-monitor 1.5%  0.4%   Running ./target/release/kore-sys-monitor     │ │
│ └─────────────────────────────────────────────────────────────────────────────────────────────┘ │
├────────────────────────────────────────────────────────────────────────────────────────────────┤
│ [Tab] Cambiar Vista │ [/] Buscar │ [k] Matar Proceso │ [s] Ordenar │ [r] Invertir │ [q] Salir  │
╰────────────────────────────────────────────────────────────────────────────────────────────────╯
```

### B. Popup / Window Modal (Kill Process Confirmation)

```
┌─ Terminar Proceso ─────────────────────────────────┐
│                                                    │
│  ¿Seguro que deseas enviar SIGKILL al proceso?    │
│                                                    │
│  PID:   1420                                       │
│  Name:  firefox                                    │
│  CPU:   14.2%   │   MEM: 8.5%                      │
│                                                    │
│     [ Y: Confirmar (SIGKILL) ]   [ N / Esc: Cancelar ]│
└────────────────────────────────────────────────────┘
```

---

## 4. Adaptabilidad y Comportamiento Responsivo (Breakpoints)

- **Pantalla Pequeña (< 80 col / < 24 filas)**:
  - Oculta descripciones largas de comandos en la tabla de procesos.
  - Colapsa gráficos detallados de red a texto resumen.
  - El menú de pestañas pasa a modo compacto (`1:Over 2:Proc 3:Disk 4:CPU`).

- **Pantalla Mediana (80-120 col / 24-40 filas)**:
  - Layout estándar de 2 columnas para métricas de hardware + tabla inferior para procesos.

- **Pantalla Ultra-Wide (> 120 col / > 40 filas)**:
  - Layout de 3 columnas: CPU per-core a la izquierda, Memoria y Red al centro, Discos y Procesos a la derecha.

---

## 5. Tabla de Atajos de Teclado Completa (Keybindings)

| Tecla / Combinación | Modo / Ámbito | Acción |
| :--- | :--- | :--- |
| `Tab` / `Shift+Tab` | Global | Avanzar / Retroceder pestaña. |
| `1`, `2`, `3`, `4` | Global | Selección directa de pestaña. |
| `j` / `Down` | Tabla Procesos | Mover selección al siguiente proceso. |
| `k` / `Up` | Tabla Procesos | Mover selección al proceso anterior. |
| `PageDown` / `PageUp`| Tabla Procesos | Desplazamiento por página rápida. |
| `Home` / `End` | Tabla Procesos | Ir al primer / último proceso de la lista. |
| `/` | Tabla Procesos | Entrar en modo Búsqueda/Filtro interactivo. |
| `Esc` | Búsqueda/Modal | Cancelar filtro actual o cerrar ventana modal. |
| `s` | Tabla Procesos | Ciclar columna de ordenación (CPU% → MEM% → PID → NAME). |
| `r` | Tabla Procesos | Invertir el sentido de ordenación (Asc / Desc). |
| `k` (o `Delete`) | Proceso seleccionado | Abrir modal de confirmación para terminar proceso. |
| `?` | Global | Abrir modal con ayuda y lista de atajos. |
| `q` / `Ctrl+C` | Global | Salir de la aplicación restaurando la terminal. |

---

## 6. Detalles Tecnológicos de Renderizado
1. **Widgets de Ratatui**:
   - `ratatui::widgets::Sparkline` para históricos de CPU y tráfico RX/TX de Red.
   - `ratatui::widgets::Gauge` / `LineGauge` para porcentajes de RAM y Swap.
   - `ratatui::widgets::Table` con `TableState` para selección interactiva de procesos.
   - `ratatui::widgets::Clear` + `Block` emergente para renderizado modal con capa superpuesta.
2. **Optimización CPU**:
   - Medición delta de `sysinfo` sin llamar bloqueante a I/O.
   - Guardado de histórico en buffers circulares `VecDeque<u64>` de capacidad fija (e.g., 60 puntos).
