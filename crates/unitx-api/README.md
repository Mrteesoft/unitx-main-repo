# unitx-api

Axum-based HTTP service that exposes the unitx conversion engine. Ships temperature, distance, and live currency endpoints with validation and structured errors.

## Running

```bash
cargo run -p unitx-api
```

Optional environment variables:

- `RUST_LOG=unitx_api=debug` for verbose logging.
- No API keys needed; rates come from the public ECB feed.

## Sample request

```bash
curl -X POST http://localhost:8080/convert/currency \
  -H "Content-Type: application/json" \
  -d '{"value":"100.00","from":"USD","to":"EUR"}'
```

Response:

```json
{
  "category": "currency",
  "from": "USD",
  "to": "EUR",
  "input": "100.00",
  "output": "92.00"
}
```

Actual output depends on the live rate returned by the upstream provider.
