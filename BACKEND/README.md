# Wallet Backend - MVP Completo

Backend completo en Rust (Axum) con **lógica implementada** para wallet con Account Abstraction y sistema de Reputación.

## Estado: TOTALMENTE FUNCIONAL

- Generación de keypairs Stellar (ed25519)
- Integración con Friendbot y Horizon API
- Sistema de Reputación con fórmula real
- Conversión con CoinGecko
- Transferencias bancarias con validación
- Account Abstraction simulada

## Instalación Rápida

```bash
# 1. Instalar Rust (si no lo tienes)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Clonar y entrar al proyecto
cd backend-complete

# 3. Copiar variables de entorno
cp .env.example .env

# 4. Instalar SQLx CLI
cargo install sqlx-cli --no-default-features --features sqlite

# 5. Crear base de datos y ejecutar migraciones
sqlx database create
sqlx migrate run

# 6. Compilar y ejecutar
cargo build
cargo run

# 7. Verificar
curl http://localhost:4000/api/health
```

## Flujo de Prueba Completo

```bash
# 1. Generar wallet con AA
curl -X POST http://localhost:4000/api/wallet/generate \
  -H "Content-Type: application/json" \
  -d '{"aa_mode": true, "reveal_secret": false}'

# Guardar el public_key de la respuesta
export PUBKEY="G..."

# 2. Fundear cuenta (testnet)
curl -X POST http://localhost:4000/api/wallet/fund \
  -H "Content-Type: application/json" \
  -d "{\"public_key\": \"$PUBKEY\"}"

# 3. Ver balance
curl http://localhost:4000/api/wallet/$PUBKEY/balance

# 4. Ver reputación
curl http://localhost:4000/api/reputation/$PUBKEY

# 5. Convertir XLM a USDC/MXN
curl -X POST http://localhost:4000/api/convert/to-usdc \
  -H "Content-Type: application/json" \
  -d '{"from_token": "XLM", "amount": "100"}'

# 6. Intentar transferencia bancaria
curl -X POST http://localhost:4000/api/bank/transfer \
  -H "Content-Type: application/json" \
  -d "{\"public_key\": \"$PUBKEY\", \"amount_fiat\": 1000.0, \"currency\": \"MXN\", \"bank_account\": \"1234567890\"}"

# 7. Ver todas las transferencias (admin)
curl http://localhost:4000/api/admin/transfers
```

## Endpoints Implementados

### Wallet

- `POST /api/wallet/generate` - Generar wallet con AA
- `POST /api/wallet/fund` - Fundear via Friendbot
- `GET /api/wallet/:pubkey/balance` - Ver balance
- `POST /api/wallet/:pubkey/send` - Enviar transacción
- `POST /api/aa/relayer` - Relayer de AA

### Reputación

- `GET /api/reputation/:pubkey` - Obtener score de reputación

### Conversión

- `POST /api/convert/to-usdc` - Convertir a USDC
- `GET /api/rates?from=X&to=Y` - Obtener tasas

### Banco

- `POST /api/bank/transfer` - Crear transferencia (valida reputación)
- `GET /api/admin/transfers` - Listar transferencias

## Servicios Implementados

### 1. Stellar Service

- Fund account via Friendbot
- Get balance from Horizon
- Submit transactions
- Check account existence

### 2. Wallet Service

- Generar keypairs con encoding Stellar
- Fundear wallets
- Enviar transacciones
- Query balance

### 3. Reputation Service

**Fórmula:**

```.md
base_score = 10
tx_bonus = min(tx_count * 2, 40)
volume_bonus = min(log10(total_volume) * 10, 30)
age_bonus = min(account_age_days / 10, 20)
trust_score = min(total, 100)

Niveles:
0-30: Unverified
31-60: Verified L1
61-80: Verified L2
81-100: Trusted
```

### 4. Convert Service

- Integración real con CoinGecko
- Conversión XLM/ETH/BTC → USDC → MXN
- Fallback a rates mock

### 5. Bank Service

- Validación de reputación antes de procesar
- Máscara de cuentas bancarias
- Registro completo de transfers

### 6. AA Service

- HashMap en memoria para signers
- Relay de transacciones simulado

## Variables de Entorno

Ver `.env.example` para todas las variables disponibles.

Principales:

```.env
PORT=4000
DATABASE_URL=sqlite://./wallet.db
STELLAR_HORIZON_URL=https://horizon-testnet.stellar.org
FRIENDBOT_URL=https://friendbot.stellar.org
REPUTATION_THRESHOLD=50
COINGECKO_API_URL=https://api.coingecko.com/api/v3
```

## Desarrollo

```bash
# Compilar en modo debug
cargo build

# Ejecutar con logs detallados
RUST_LOG=debug cargo run

# Formatear código
cargo fmt

# Lints
cargo clippy

# Recrear DB
rm wallet.db
sqlx database create
sqlx migrate run
```

## Deploy

### Railway

```bash
railway login
railway init
railway up
```

### Heroku

```bash
heroku create wallet-backend
git push heroku main
```

## Troubleshooting

### Error: "no matching package"

Ya está corregido en Cargo.toml. Si ves este error, asegúrate de tener la última versión.

### Error: "sqlx macro compile error"

```bash
cargo clean
sqlx database create
sqlx migrate run
cargo build
```

### Error: "Friendbot already funded"

Normal si la cuenta ya fue fundeada. Verifica balance en Stellar Explorer.

## Próximos Pasos

1. Agregar tests unitarios
2. Implementar JWT auth
3. Conectar frontend React
4. Deploy a producción
5. Integrar Circle/Stripe real
6. Implementar KYC/AML

## Notas

- **Testnet only**: No usar fondos reales
- **AA simulado**: En producción usar HSM/KMS
- **Seeds en memoria**: Solo para demo, no seguro para producción
- **CoinGecko**: Respeta rate limits (50 calls/min)
