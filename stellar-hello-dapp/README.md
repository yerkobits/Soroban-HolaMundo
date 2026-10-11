# Soroban Hola Mundo

Proyecto de ejemplo que incluye un contrato inteligente Soroban escrito en Rust y una aplicación web hecha con React, TypeScript y Vite para interactuar con él en Stellar Testnet.

> Este proyecto está configurado para pruebas en **Testnet**. No lo uses para gestionar fondos reales ni lo despliegues en Mainnet sin revisar y adaptar el código.

## Estructura

```text
.
├── contracts/
│   └── hello-world/       # Contrato Soroban
├── stellar-hello-dapp/    # Frontend React + TypeScript + Vite
├── Cargo.toml             # Workspace de Rust
└── README.md
```

## Requisitos

Para compilar y desplegar el contrato:

- Rust y Cargo
- Stellar CLI
- El target de Rust `wasm32v1-none`

Para ejecutar el frontend:

- Node.js
- npm

## Contrato Soroban

### Preparar el entorno

Instala el target de compilación WebAssembly:

```bash
rustup target add wasm32v1-none
```

Comprueba que Stellar CLI está instalado:

```bash
stellar --version
```

Configura una identidad y Testnet según las instrucciones oficiales de Stellar. Necesitarás una identidad con fondos de prueba para desplegar e invocar contratos.

### Compilar

Desde la raíz del repositorio:

```bash
stellar contract build
```

Los archivos WASM compilados se generan en:

```text
target/wasm32v1-none/release/
```

### Desplegar en Testnet

Despliega el archivo WASM generado. Sustituye `<archivo>.wasm` por el nombre del archivo correspondiente y `<identidad>` por una identidad configurada en Stellar CLI:

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/<archivo>.wasm \
  --source-account <identidad> \
  --network testnet
```

Guarda el ID del contrato que devuelve el comando; el frontend lo necesita para conectarse al contrato.

### Invocar el contrato desde la CLI

La función de ejemplo `hello` recibe un nombre mediante el argumento `--to`:

```bash
stellar contract invoke \
  --id <ID_DEL_CONTRATO> \
  --source-account <identidad> \
  --network testnet \
  -- hello --to mundo
```

## Frontend

La aplicación web está en `stellar-hello-dapp/`.

### Instalar dependencias

```bash
cd stellar-hello-dapp
npm install
```

### Configurar el contrato

Configura en el frontend el ID del contrato que desplegaste en Testnet. Revisa los archivos de `src/` y busca la constante o variable usada para el ID del contrato:

```bash
grep -RniE 'CONTRACT_ID|contractId|CONTRACT' src
```

Reemplaza el valor de ejemplo por el ID obtenido al desplegar el contrato. Verifica también que la aplicación esté configurada para utilizar Stellar Testnet.

### Ejecutar localmente

```bash
npm run dev
```

Vite mostrará en la terminal la dirección local, normalmente:

```text
http://localhost:5173/
```

Para escuchar en todas las interfaces de red y elegir el puerto 3000:

```bash
npm run dev -- --host 0.0.0.0 --port 3000
```

## Desplegar el frontend en un servidor con PM2

PM2 mantiene el proceso ejecutándose y puede restaurarlo después de reiniciar el servidor. El siguiente procedimiento ejecuta el servidor de desarrollo de Vite en el puerto 3000, como entorno de prueba.

Desde la carpeta `stellar-hello-dapp`:

```bash
npm install
pm2 start npm \
  --name "stellar-hello-dapp" \
  --cwd "$PWD" \
  -- run dev -- --host 0.0.0.0 --port 3000
```

Comprueba el proceso y los logs:

```bash
pm2 status
pm2 logs stellar-hello-dapp
```

Abre desde el navegador:

```text
http://<IP_DEL_SERVIDOR>:3000/
```

Permite conexiones TCP al puerto 3000 tanto en el firewall del servidor como en el firewall o panel de red del proveedor VPS, si corresponde.

Para guardar la lista de procesos de PM2:

```bash
pm2 save
```

Para que PM2 se inicie automáticamente después de reiniciar el servidor:

```bash
pm2 startup
```

Ejecuta el comando que PM2 muestre y luego guarda de nuevo la configuración:

```bash
pm2 save
```

### Actualizar el frontend

Después de modificar el código, reinicia el proceso:

```bash
pm2 restart stellar-hello-dapp
```

Si cambiaste dependencias:

```bash
cd stellar-hello-dapp
npm install
pm2 restart stellar-hello-dapp
```

### Detener o eliminar el proceso

```bash
pm2 stop stellar-hello-dapp
pm2 delete stellar-hello-dapp
```

## Build de producción

El servidor de desarrollo de Vite es útil para pruebas, pero no está pensado como servidor público de producción. Para generar la versión optimizada:

```bash
cd stellar-hello-dapp
npm install
npm run build
```

Los archivos listos para servir se generan en:

```text
stellar-hello-dapp/dist/
```

En producción, sirve el contenido de `dist/` con Nginx o con un servidor estático. Si usas Nginx, configura el sitio para apuntar a esa carpeta y permitir el fallback a `index.html` para las rutas del frontend. PM2 puede seguir usándose para un backend Node, si el proyecto incorpora uno.

## Solución de problemas

### El puerto ya está ocupado

Comprueba qué proceso está escuchando en el puerto:

```bash
ss -ltnp | grep ':3000'
```

Si el puerto elegido está ocupado, selecciona otro, por ejemplo `3001`:

```bash
npm run dev -- --host 0.0.0.0 --port 3001
```

### Revisar logs de PM2

```bash
pm2 logs stellar-hello-dapp --lines 100
```

### Verificar el frontend desde el servidor

```bash
curl -i http://127.0.0.1:3000/
```

Si responde localmente pero no desde el navegador, revisa el firewall del servidor y las reglas de red del proveedor.

### Error al compilar el frontend

```bash
cd stellar-hello-dapp
npm run build
```

Resuelve los errores que muestre TypeScript o Vite antes de iniciar o desplegar la aplicación.

## Licencia

Este proyecto está publicado bajo la licencia MIT. Consulta el archivo `LICENSE`.
