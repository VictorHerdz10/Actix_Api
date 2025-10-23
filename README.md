# 🚀 Rust API con Actix-web

Una API REST moderna construida con Rust y el framework Actix-web, implementando arquitectura MVC y mejores prácticas de seguridad.

## ✨ Características

- **🛠 Framework**: Actix-web 4.4
- **🗄️ Base de datos**: PostgreSQL con SeaORM
- **🔐 Autenticación**: JWT (JSON Web Tokens)
- **🔒 Seguridad**: BCrypt para hash de contraseñas
- **🌐 CORS**: Configuración completa de CORS
- **📊 Logging**: Tracing y logging estructurado
- **🏗️ Arquitectura**: MVC (Modelo-Vista-Controlador)
- **🐳 Docker**: Configuración lista para Docker
- **🚀 Ready for Production**: Configuración para entornos de desarrollo y producción

## 🏗️ Estructura del Proyecto

```
src/
├── main.rs                 # Punto de entrada de la aplicación
├── config/                 # Configuración de base de datos
├── models/                 # Modelos de datos (Entidades SeaORM)
├── controllers/            # Lógica de negocio
├── routes/                 # Definición de rutas
├── middleware/             # Middleware (Auth, CORS)
├── utils/                  # Utilidades (JWT, Password)
└── errors/                 # Manejo de errores
```

## 🚀 Instalación y Configuración

### Prerrequisitos

- Rust 1.70+
- PostgreSQL
- Cargo

### 1. Clonar el repositorio

```bash
git clone https://github.com/VictorHerdz10/Actix_Api.git
cd Actix_Api
```

### 2. Configurar variables de entorno

```bash
# Crear archivo de entorno de desarrollo
cp .env.example .env.development

# Editar con tus valores
nano .env.development
```

### 3. Ejecutar la aplicación

```bash
# Desarrollo
cargo run

# Producción
cargo build --release
./target/release/rust-api
```

## 🐳 Ejecutar con Docker

### Construir y ejecutar con Docker:

```bash
# Construir la imagen
docker build -t actix-api .

# Ejecutar el contenedor
docker run -p 8080:8080 --env-file .env.development actix-api
```

### O usar Docker Compose:

```bash
docker-compose up --build
```

## 📋 Variables de Entorno

Crear un archivo `.env.development` con:

```env
# Servidor
PORT=8080
RUST_LOG=debug
HOST=0.0.0.0

# Base de datos
DATABASE_URL=postgres://usuario:contraseña@localhost:5432/rust_api_dev

# JWT
JWT_SECRET=tu_clave_super_secreta_minimo_32_caracteres
JWT_EXPIRATION_HOURS=24

# Bcrypt
BCRYPT_COST=8
```

## 📚 Endpoints de la API

### 🔓 Endpoints Públicos

- `GET /api/salud` - Verificación del estado del servidor
- `GET /api/info` - Información de la API
- `POST /api/auth/registro` - Registrar nuevo usuario
- `POST /api/auth/login` - Iniciar sesión

### 🔐 Endpoints Protegidos (Requieren JWT)

- `GET /api/usuarios` - Obtener todos los usuarios
- `GET /api/usuarios/{id}` - Obtener usuario por ID
- `PUT /api/usuarios/{id}` - Actualizar usuario
- `DELETE /api/usuarios/{id}` - Eliminar usuario
- `GET /api/auth/perfil` - Obtener perfil del usuario actual

## 🔐 Autenticación

La API utiliza JWT para autenticación. Para acceder a endpoints protegidos:

```http
Authorization: Bearer <tu_token_jwt>
```

### Ejemplo de Registro

```bash
curl -X POST http://localhost:8080/api/auth/registro \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Juan Pérez",
    "email": "juan@ejemplo.com",
    "password": "mi_contraseña_segura"
  }'
```

### Ejemplo de Login

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "juan@ejemplo.com",
    "password": "mi_contraseña_segura"
  }'
```

Respuesta:
```json
{
  "exito": true,
  "mensaje": "Inicio de sesión exitoso",
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "usuario": {
    "id": "uuid-del-usuario",
    "email": "juan@ejemplo.com",
    "nombre": "Juan Pérez"
  }
}
```

## 🛠️ Desarrollo

### Ejecutar en modo desarrollo

```bash
cargo run
```

La aplicación estará disponible en `http://localhost:8080`

### Ejecutar tests

```bash
cargo test
```

### Formatear código

```bash
cargo fmt
```

### Verificar linting

```bash
cargo clippy
```

## 📦 Dependencias Principales

- **actix-web** - Framework web asyncrono
- **sea-orm** - ORM para Rust
- **jsonwebtoken** - Implementación de JWT
- **bcrypt** - Hash de contraseñas
- **serde** - Serialización/Deserialización
- **tracing** - Logging estructurado
- **uuid** - Generación de UUIDs
- **chrono** - Manejo de fechas y horas

## 🗃️ Modelo de Usuario

```rust
struct User {
    id: i32,                    // ID único
    name: String,               // Nombre del usuario
    email: String,              // Email único
    password: String,           // Contraseña hasheada
    created_at: DateTimeUtc,    // Fecha de creación
    updated_at: DateTimeUtc,    // Fecha de actualización
}
```

## 🐳 Dockerfile

El proyecto incluye un Dockerfile optimizado para producción:

```dockerfile
# Build stage
FROM rust:bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Final run stage
FROM debian:bookworm-slim AS runner

WORKDIR /app
COPY --from=builder /app/target/release/rust-api /app/rust-api
CMD ["/app/rust-api"]
```

## 🤝 Contribuir

1. Fork el proyecto
2. Crear una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abrir un Pull Request

## 📝 Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.

## 👨‍💻 Autor

- **Victor Herdz** - [GitHub](https://github.com/VictorHerdz10)

## 🙏 Agradecimientos

- Equipo de Actix-web por el excelente framework
- Comunidad de Rust por la documentación y soporte
- Desarrolladores de SeaORM por el poderoso ORM

---

**¿Problemas o sugerencias?** ¡Abre un issue en el repositorio! 🐛

**⭐ ¿Te gusta este proyecto? ¡Dale una estrella al repositorio!**