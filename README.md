
# Rust API con Actix-web

Una API REST simple construida con Rust y el framework Actix-web, desplegable en Vercel.

## 🚀 Características

- **Framework**: Actix-web (alto rendimiento y madurez)
- **Async Runtime**: Actix-rt (basado en Tokio)
- **Serialización**: Serde + JSON
- **CORS**: Configurado para permitir cualquier origen
- **Logging**: Tracing + Actix-web Logger
- **Estado en memoria**: HashMap para almacenamiento temporal
- **Paginación**: Soporte para limit/offset en listados
- **Middleware**: Logger integrado para debugging

## 📁 Estructura del Proyecto

```
rust-api/
├── src/
│   └── main.rs          # Código principal de la API
├── Cargo.toml           # Dependencias y configuración
├── vercel.json          # Configuración de despliegue en Vercel
├── .env.example         # Variables de entorno de ejemplo
└── README.md            # Esta documentación
```

## 🔧 Endpoints de la API

### Health Check
- `GET /health` - Verifica si el servidor está funcionando

### Información de la API
- `GET /api` - Retorna información sobre la API y sus endpoints

### Gestión de Usuarios
- `GET /api/users` - Obtiene todos los usuarios (con paginación)
- `POST /api/users` - Crea un nuevo usuario
- `GET /api/users/:id` - Obtiene un usuario por ID
- `PUT /api/users/:id` - Actualiza un usuario por ID
- `DELETE /api/users/:id` - Elimina un usuario por ID


## 🛠️ Desarrollo Local

### Prerrequisitos
- Rust 1.70+ 
- Cargo

### Instalar Rust (si no lo tienes)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Ejecutar localmente
```bash
# Clonar el proyecto
git clone <repo>
cd rust-api

# Copiar variables de entorno
cp .env.example .env

# Ejecutar el servidor
cargo run
```

El servidor estará disponible en `http://localhost:3000`

### Probar la API localmente
```bash
# Health check
curl http://localhost:3000/health

# Crear usuario
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"name": "Test User", "email": "test@example.com"}'

# Obtener usuarios
curl http://localhost:3000/api/users
```

## 📊 Formato de Respuestas

Todas las respuestas siguen este formato:

```json
{
  "success": true,
  "data": { ... },
  "message": "Operation successful"
}
```

En caso de error:
```json
{
  "success": false,
  "data": null,
  "message": "Error description"
}
```

## 🔍 Modelo de Datos

### User
```json
{
  "id": "uuid-v4",
  "name": "Nombre del usuario",
  "email": "email@example.com",
  "created_at": "2023-12-07T10:30:00Z"
}
```

### CreateUser (para POST)
```json
{
  "name": "Nombre del usuario",
  "email": "email@example.com"
}
```

### UpdateUser (para PUT)
```json
{
  "name": "Nombre actualizado (opcional)",
  "email": "email@actualizado.com (opcional)"
}
```

## 🚨 Consideraciones

1. **Estado**: La API usa almacenamiento en memoria, los datos se pierden al reiniciar
2. **Producción**: Para producción, considera usar una base de datos real
3. **Seguridad**: El CORS está configurado para permitir cualquier origen (ajustar según necesidad)
4. **Performance**: Actix-web es extremadamente rápido y maduro, ideal para APIs de alto rendimiento


```

## 📚 Recursos Adicionales

- [Documentación de Actix-web](https://docs.rs/actix-web/)
- [Actix Documentation](https://actix.rs/)
- [Vercel Functions Runtime for Rust](https://github.com/vercel/vercel/tree/main/packages/rust)
- [Serde Documentation](https://serde.rs/)
- [Rust Documentation](https://doc.rust-lang.org/)

## 🤝 Contribuir

1. Fork del proyecto
2. Crear una feature branch
3. Commit con cambios
4. Push a la branch
5. Abrir un Pull Request

## 📄 Licencia

MIT License - puedes usar este código como quieras.

---


```