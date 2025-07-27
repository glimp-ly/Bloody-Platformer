# Bloody Platformer 🩸

> Action platformer roguelite desarrollado con Rust (Bevy) y Go

>[!WARNING]
> On hiatus due to other projects

![Gameplay screenshot](Pendiente)

## Características
- Movimiento fluido con físicas realistas
- Sistema de combate ágil
- Generación procedural de niveles
- Backend en Go para estadísticas

## Tecnologías
- **Game Engine**: Bevy 0.13
- **Backend**: Go 1.22
- **Físicas**: Rapier2D
- **CI/CD**: GitHub Actions

## Cómo ejecutar
```bash
# Servidor backend
cd go-backend && go run main.go

# Cliente del juego
cargo run