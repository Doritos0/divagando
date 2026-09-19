# AGENTS.md - Contexto y Guía para Asistentes e IA

## 🎯 Propósito del Proyecto
**Divagando** es una aplicación de escritorio simple creada como ejercicio práctico para **aprender Rust de manera autodidacta**. 
La funcionalidad principal de la aplicación es ser un **Generador de Citas / Frases Célebres**, mostrando inicios famosos de obras literarias (como *Cien años de soledad*, *Anna Karenina*, etc.) con una interfaz nativa limpia.

---

## 🛠️ Stack Tecnológico y Arquitectura

- **Lenguaje:** Rust (Edición 2024 / Stable).
- **Interfaz de Usuario (UI):** Nativa de escritorio basada en **GTK4 / Libadwaita** utilizando **Blueprint** (`blueprint-compiler` / `.blp`) para el marcado declarativo de la interfaz gráfica (sin HTML, CSS ni JS).
- **Base de Datos & ORM:** SQLite gestionado mediante **Diesel ORM** (con la característica `bundled` activa).
- **Variables de Entorno:** `.env` gestionado con la *crate* `dotenvy`.
- **Estructura del Proyecto:**
  ```text
  divagando/
  ├── .env                # Configuración de BD_NAME
  ├── Cargo.toml          # Dependencias y manifiesto
  ├── AGENTS.md           # Contexto e instrucciones para IAs
  ├── docs/               # Guías y documentación del proyecto
  ├── src/
  │   ├── main.rs         # Punto de entrada de la aplicación
  │   ├── config/         # Módulo de configuración
  │   │   ├── mod.rs
  │   │   └── db.rs      # Conexión y caché (OnceLock) a SQLite
  │   ├── models.rs       # Estructuras de datos / Diesel models
  │   └── schema.rs      # Esquema autogenerado por Diesel
  └── ui/                 # Diseños de interfaz en Blueprint (.blp)
  ```

---

## 🤖 Reglas de Interacción para Agentes de IA

1. **Rol del Agente (Profesor / Mentor):**
   - El objetivo principal del usuario es **aprender Rust por sí mismo**.
   - **NO escribir código fuente de Rust** en las respuestas a menos que el usuario lo solicite explícitamente para una tarea o módulo específico (como configuración o setup).
   - Guía al usuario explicando sintaxis, reglas del *borrow checker*, *ownership*, *lifetimes*, patrones de diseño e interpretación de errores del compilador.

2. **Criterios de Código y Diseño:**
   - Evitar la sobre-ingeniería.
   - Preferir soluciones simples, legibles y directas.
   - Mantener el enfoque nativo sin integrar entornos web.
