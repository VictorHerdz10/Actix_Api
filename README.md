
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

## 📝 Ejemplos de Uso

### Crear un usuario
```bash
curl -X POST https://your-app.vercel.app/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Juan Pérez",
    "email": "juan@example.com"
  }'
```

### Obtener todos los usuarios
```bash
curl https://your-app.vercel.app/api/users
```

### Obtener usuarios con paginación
```bash
curl "https://your-app.vercel.app/api/users?limit=10&offset=0"
```

### Obtener un usuario específico
```bash
curl https://your-app.vercel.app/api/users/user-id
```

### Actualizar un usuario
```bash
curl -X PUT https://your-app.vercel.app/api/users/user-id \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Juan Pérez Actualizado"
  }'
```

### Eliminar un usuario
```bash
curl -X DELETE https://your-app.vercel.app/api/users/user-id
```

## 🚀 Despliegue en Vercel

### Prerrequisitos
1. Tener una cuenta en [Vercel](https://vercel.com)
2. Instalar Vercel CLI: `npm i -g vercel`
3. Tener Rust instalado localmente (para desarrollo)

### Pasos para desplegar

1. **Clonar o crear el proyecto**
   ```bash
   git clone <tu-repo>
   cd rust-api
   ```

2. **Instalar Vercel CLI (si no lo tienes)**
   ```bash
   npm i -g vercel
   ```

3. **Iniciar sesión en Vercel**
   ```bash
   vercel login
   ```

4. **Desplegar el proyecto**
   ```bash
   vercel
   ```

5. **Seguir las instrucciones**
   - Seleccionar el proyecto (crear uno nuevo si es necesario)
   - Confirmar la configuración
   - Esperar el despliegue

### Configuración Automática

El proyecto ya incluye:
- `vercel.json` con la configuración necesaria
- `Cargo.toml` optimizado para Vercel
- Variables de entorno configuradas

### Variables de Entorno

Puedes configurar variables de entorno en el dashboard de Vercel:

1. Ve a tu proyecto en Vercel
2. Click en "Settings" → "Environment Variables"
3. Agrega las variables necesarias:
   - `RUST_LOG`: Nivel de logging (info, debug, etc.)
   - `PORT`: Puerto del servidor (generalmente 3000)

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

## 🧪 Testing

### Script de prueba rápido
```bash
#!/bin/bash
# test-api.sh

BASE_URL="http://localhost:3000"

echo "🚀 Probando API Rust con Actix-web"
echo "==================================="

# Health check
echo -e "\n1. Health Check:"
curl -s "$BASE_URL/health" | jq .

# API info
echo -e "\n2. API Information:"
curl -s "$BASE_URL/api" | jq .

# Create user
echo -e "\n3. Creating user:"
USER_RESPONSE=$(curl -s -X POST "$BASE_URL/api/users" \
  -H "Content-Type: application/json" \
  -d '{"name": "Juan Pérez", "email": "juan@example.com"}')
echo "$USER_RESPONSE" | jq .

# Extract user ID
USER_ID=$(echo "$USER_RESPONSE" | jq -r '.data.id')

if [ "$USER_ID" != "null" ]; then
  # Get user
  echo -e "\n4. Getting user by ID ($USER_ID):"
  curl -s "$BASE_URL/api/users/$USER_ID" | jq .

  # Update user
  echo -e "\n5. Updating user:"
  curl -s -X PUT "$BASE_URL/api/users/$USER_ID" \
    -H "Content-Type: application/json" \
    -d '{"name": "Juan Pérez Actualizado"}' | jq .

  # Get all users
  echo -e "\n6. Getting all users:"
  curl -s "$BASE_URL/api/users" | jq .
else
  echo "❌ Error creating user"
fi

echo -e "\n✅ Pruebas completadas!"
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

**Nota**: Esta API es un ejemplo educativo. Para producción, considera:
- Usar una base de datos real (PostgreSQL, MySQL, etc.)
- Implementar autenticación y autorización
- Configurar CORS de forma más restrictiva
- Agregar más validaciones y manejo de errores
- Implementar rate limiting
```