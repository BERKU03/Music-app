# MusicApp Core

## Descripción
MusicApp Core es el núcleo de una aplicación de reproducción de música multiplataforma, escrita en **Rust**, pensada para integrarse con una interfaz gráfica usando **Tauri**.  

El core maneja:
- Canciones locales (`Song`)  
- Playlists (`Playlist`)  
- Sincronización futura con Apple Music (`MatchResult`)  

El objetivo es **separar la lógica de negocio de la UI**, permitiendo un desarrollo limpio y escalable.

---

## Estructura del proyecto

```text
src/
├── main.rs           # Punto de entrada del programa
├── library/          # Biblioteca de canciones
│   ├── mod.rs
│   ├── song.rs       # Struct Song
│   └── library.rs    # Struct Library
├── playlists/        # Playlists y tracks
│   ├── mod.rs
│   └── playlist.rs
└── matching/         # Matching con fuentes externas
    ├── mod.rs
    └── match_result.rs
