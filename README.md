# weather-api
Weather API

Esta es una API simple para obtener información sobre el clima de diferentes ciudades. Proporciona endpoints para listar, crear, editar y eliminar registros de clima.
Dependencias

Para ejecutar y utilizar esta API, necesitarás tener instalado lo siguiente:

    Rust (versión 2021 o superior)
    PostgreSQL (versión 9.5 o superior)

Además, la API utiliza las siguientes dependencias de Rust:

    actix-cors (versión 0.6.4) - Soporte para CORS (Cross-Origin Resource Sharing) en Actix web.
    actix-web (versión 4.2.1) - Framework para desarrollar aplicaciones web en Rust.
    chrono (versión 0.4.23) - Tipos de fecha y hora en Rust con soporte para serde.
    dotenv (versión 0.15.0) - Carga de variables de entorno desde un archivo .env.
    env_logger (versión 0.10.0) - Configuración de registro basada en variables de entorno.
    serde (versión 1.0.152) - Serialización y deserialización de datos en Rust con soporte para derive.
    serde_json (versión 1.0.91) - Trabajo con datos JSON en Rust.
    sqlx (versión 0.7.1) - Capa de abstracción para interactuar con bases de datos en Rust.
    uuid (versión 1.2.2) - Trabajo con UUIDs en Rust con soporte para serde.
    thiserror (versión 1.0.43) - Macros para crear errores personalizados en Rust.

Configuración

Sigue estos pasos para configurar y ejecutar la API:

    Clona este repositorio en tu máquina local:
    git clone git@github.com:SanchezrEdwin01/weather-api.git

    Navega al directorio del proyecto:
    cd weather-api

    Crea un archivo .env en el directorio raíz del proyecto y especifica las siguientes variables de entorno:
    DATABASE_URL=postgresql://username:password@localhost:5432/weather_db

    Asegúrate de reemplazar username y password con las credenciales adecuadas para tu base de datos PostgreSQL.

    Y como recomendación ten una base de datos llamada weather_db para evitar complicaciones

    Instala las dependencias del proyecto ejecutando el siguiente comando:
    cargo build --release

    Ejecuta las migraciones de la base de datos para configurar el esquema:
    cargo run --bin migrate

    Esto creará las tablas necesarias en la base de datos.

    Inicia el servidor de la API con el siguiente comando:
    cargo run --bin server

        El servidor se ejecutará en http://localhost:8000 por defecto.

Uso

Una vez que el servidor esté en ejecución, puedes utilizar los siguientes endpoints para interactuar con la API:

    GET /api/weather: Obtiene una lista de registros de clima. Puedes agregar los siguientes parámetros de consulta opcionales:
        page: El número de página para la paginación de resultados.
        limit: El límite de elementos por página.

    POST /api/weather: Crea un nuevo registro de clima. Debes enviar los siguientes datos en el cuerpo de la solicitud en formato JSON:

    {
      "city": "Nombre de la ciudad",
      "temperature": "Temperatura actual"
    }

Los campos adicionales como description, humidity y wind_speed son opcionales.

  PATCH /api/weather/{id}:
  
Actualiza un registro de clima existente. Debes proporcionar el ID del registro en la URL y enviar los datos actualizados en el cuerpo de la solicitud en formato JSON.
Puedes incluir los siguientes campos en el cuerpo de la solicitud:

    {
      "city": "Nuevo nombre de la ciudad",
      "temperature": "Nueva temperatura",
      "description": "Nueva descripción",
      "humidity": "Nueva humedad",
      "wind_speed": "Nueva velocidad del viento"
    }

DELETE /api/weather/{id}: Elimina un registro de clima existente. Debes proporcionar el ID del registro en la URL.

Ejemplos de consumo

Aquí tienes algunos ejemplos de cómo consumir la API utilizando herramientas como cURL o Postman:

    Obtener una lista de registros de clima:
    
      curl -X GET "http://localhost:8000/api/weather?page=1&limit=10"

    Crear un nuevo registro de clima:
      curl -X POST -H "Content-Type: application/json" -d
      '{
        "city": "New York",
        "temperature": "25°C"
        }'"http://localhost:8000/api/weather"

    Actualizar un registro de clima existente:
      curl -X PATCH -H "Content-Type: application/json" -d '{
      "temperature": "30°C",
      "humidity": "60%"
      }' "http://localhost:8000/api/weather/{id}"

    Eliminar un registro de clima existente:
      curl -X DELETE "http://localhost:8000/api/weather/{id}"
      
Recuerda reemplazar {id} con el ID correspondiente al registro de clima que deseas actualizar o eliminar.

También puedes utilizar herramientas como Postman para consumir la API y realizar pruebas.

¡Eso es todo! Ahora deberías tener toda la información necesaria para ejecutar la API y consumirla utilizando los endpoints proporcionados. Si tienes alguna pregunta adicional o necesitas ayuda adicional, no dudes en preguntar.

¡Que tengas éxito con tu proyecto!
