# Authentication

HexBuffer Framework implements authentication via the **`TokenService` outbound port** — allowing hot-swapping of token implementations without touching domain or HTTP handler code.

---

## Default Token Provider: PASETO V4 Local

> **PASETO** (Platform-Agnostic Security Tokens) is the modern successor to JWT. Unlike JWTs, PASETO local tokens are **fully encrypted** (XChaCha20-Poly1305), not just signed. There are no algorithm confusion attacks.

The default adapter is **`PasetoTokenService`** using the `pasetors` crate with the V4 local variant.

### Configuration

In `AppConfig` (or via environment variables):

```toml
[auth]
token_type = "paseto"               # or "jwt"
token_secret = "32-byte-secret-key" # must be exactly 32 bytes
expiration_secs = 86400             # 24 hours
```

### How PASETO V4 Local Works

```
generate_token(user) → PasetoClaims → XChaCha20-Poly1305 Encrypt → "v4.local.<encrypted>"
verify_token(token)  → Decrypt → Parse JSON Claims → Check Expiry → Claims
```

---

## Alternative: JWT (HMAC-SHA256)

Switch to JWT by setting `auth.token_type = "jwt"`. The `JwtTokenService` adapter uses the `jsonwebtoken` crate with `Header::default()` (HS256).

### How JWT Works

```
generate_token(user) → Claims JSON → Base64URL Encode → HMAC-SHA256 Sign → "header.payload.signature"
verify_token(token)  → Signature Verify → Decode Claims → Check Expiry → Claims
```

---

## Claims Payload

Both JWT and PASETO encode the same `Claims` struct:

```rust
pub struct Claims {
    pub sub: String,    // User ID
    pub email: String,
    pub name: String,
    pub exp: i64,       // Expiration (Unix timestamp)
    pub iat: i64,       // Issued at (Unix timestamp)
}
```

---

## HTTP Authentication Flow

### Step 1: Login

```
POST /auth/login
Content-Type: application/json

{ "email": "alice@example.com" }
```

Response:
```json
{
  "token": "v4.local.Jz...",
  "token_type": "Bearer"
}
```

### Step 2: Authenticated Request

```
GET /auth/me
Authorization: Bearer v4.local.Jz...
```

Response:
```json
{
  "sub": "usr_1000",
  "email": "alice@example.com",
  "name": "Alice",
  "exp": 1769596000,
  "iat": 1769509600
}
```

### Error Responses

| Scenario | Status | Body |
| --- | --- | --- |
| Missing `Authorization` header | `401` | `"Missing Authorization header"` |
| Malformed `Bearer` prefix | `401` | `"Invalid Authorization header format"` |
| Expired token | `401` | `"PASETO token has expired"` |
| Invalid/tampered token | `401` | `"PASETO token validation failed: ..."` |

---

## Adding a New Token Adapter

1. Create a new file in `src/adapters/outbound/` (e.g. `paseto_public_token_service.rs`)
2. Implement the `TokenService` outbound port trait
3. Register it in `src/adapters/outbound/mod.rs`
4. Wire it in `main.rs` based on `config.auth.token_type`

No domain, port, or handler code changes required.

---

## Dependency Injection Wiring

In `main.rs`, the token adapter is selected based on configuration:

```rust
let token_service: Arc<dyn TokenService> = if config.auth.token_type == "jwt" {
    Arc::new(JwtTokenService::new(config.auth.token_secret, expiry))
} else {
    // Default: PASETO
    Arc::new(PasetoTokenService::new(&key_bytes, expiry)?)
};
```

---

## Security Comparison

| Feature | JWT (HS256) | PASETO V4 Local |
| --- | --- | --- |
| Token type | Signed | Encrypted |
| Algorithm confusion risk | Yes | No |
| Payload visible without key | Yes (Base64) | No (encrypted) |
| Standard | IETF RFC 7519 | PASETO spec |
| Crate | `jsonwebtoken` | `pasetors` |
| Token prefix | `eyJ...` | `v4.local....` |
