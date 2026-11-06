# Неделя 10 — CPI и вывод из vault

**Задание:** реализовать вывод — share-токены сжигаются, vault возвращает токены пользователю через Token program с подписью PDA.

Продолжение сквозного проекта Liquidity Vault:
[неделя 8 — каркас](https://github.com/zandarbek402-dotcom/Week-8) →
[неделя 9 — депозит](https://github.com/zandarbek402-dotcom/Week9) →
**неделя 10 — CPI и вывод**.

## Как работает вывод

```
 withdraw(shares, min_assets_out)
        │
        ├─ 1. проверки: shares > 0, у пользователя достаточно shares
        ├─ 2. gross = shares × (total_assets + 1) / (total_shares + 1)
        │      fee  = gross × fee_bps / 10 000      (остаётся в vault)
        │      out  = gross − fee  ≥ min_assets_out  (slippage)
        │
        ├─ 3. CPI #1  Token::burn
        │      user_share_account ──🔥──  authority = owner (подпись пользователя)
        │
        ├─ 4. CPI #2  Token::transfer_checked
        │      vault_token_account ──► user_token_account
        │      authority = config PDA  → подпись через seeds ["config", mint, bump]
        │
        ├─ 5. учёт: user_position.shares / deposited, config.total_deposited
        └─ 6. emit!(WithdrawEvent)
```

## CPI и подпись PDA

Весь код CPI собран в [`token_cpi.rs`](programs/liquidity_vault/src/token_cpi.rs):

| Функция | CPI | Кто подписывает | Как |
|---|---|---|---|
| `burn_shares` | `token::burn` | пользователь | `CpiContext::new` → `invoke` (подпись берётся из транзакции) |
| `transfer_from_vault` | `token::transfer_checked` | **config PDA** | `CpiContext::new_with_signer` → `invoke_signed` |
| `mint_shares` (депозит) | `token::mint_to` | **config PDA** | `CpiContext::new_with_signer` → `invoke_signed` |

У PDA нет приватного ключа. Программа передаёт seeds `["config", underlying_mint, bump]`;
runtime пересчитывает адрес из `seeds + program_id` и, если он совпадает с authority
vault-аккаунта, считает PDA подписантом. Поэтому вывести токены из vault может
**только эта программа** и только по правилам `withdraw`.

```rust
let bump = [config.bump];
let seeds: &[&[u8]] = &[CONFIG_SEED, mint_key.as_ref(), &bump];
token::transfer_checked(
    CpiContext::new_with_signer(token_program, TransferChecked { from: vault, mint, to: user, authority: config }, &[seeds]),
    amount,
    decimals,
)?;
```

## Безопасность

- `UserPosition` PDA: seeds `["user", config, owner]` + `has_one = owner` → нельзя выводить из чужой позиции.
- `vault_token_account`: seeds `["vault", config]` + `has_one` в config + `token::authority = config` → нельзя подменить vault.
- `share_mint`: seeds `["shares", config]` + `has_one` → нельзя подменить mint.
- `token::mint` / `token::authority = owner` для токен-аккаунтов пользователя.
- **Сначала расчёт, потом burn и transfer** — цена берётся до изменения состояния.
- Округление вниз → в пользу vault; комиссия остаётся в vault (выгода оставшимся держателям).
- `min_assets_out` — защита от проскальзывания.
- **Пауза не блокирует вывод** — она останавливает только новые депозиты, средства пользователей никогда не заблокированы.
- `close_position` — закрыть пустую позицию и вернуть rent (только если `shares == 0`).

## Что нового по сравнению с неделей 9

| Файл | Изменение |
|---|---|
| [`token_cpi.rs`](programs/liquidity_vault/src/token_cpi.rs) | CPI-хелперы: burn (подпись пользователя), transfer и mint (подпись PDA) |
| [`instructions/withdraw.rs`](programs/liquidity_vault/src/instructions/withdraw.rs) | вывод: `min_assets_out`, burn + `transfer_checked`, пропорциональное уменьшение `deposited`, событие |
| [`instructions/preview_withdraw.rs`](programs/liquidity_vault/src/instructions/preview_withdraw.rs) | read-only: сколько токенов (после комиссии) дадут `shares` |
| [`instructions/close_position.rs`](programs/liquidity_vault/src/instructions/close_position.rs) | закрытие пустой позиции, возврат rent |
| [`math.rs`](programs/liquidity_vault/src/math.rs) | `fee_for`, `proportional` + новые unit-тесты |
| [`errors.rs`](programs/liquidity_vault/src/errors.rs) | `InsufficientLiquidity`, `PositionNotEmpty` |
| [`tests/liquidity_vault.ts`](tests/liquidity_vault.ts) | 13 интеграционных тестов вывода |

## Тесты

Интеграционные (`anchor test`):
1. vault token account принадлежит config PDA
2. `preview_withdraw` показывает сумму после комиссии
3. вывод: shares сожжены, supply уменьшился, токены вернулись, fee осталась в vault, в логах 2 CPI в Token program, `WithdrawEvent`
4. slippage → `SlippageExceeded`
5. больше shares, чем есть → `InsufficientShares`
6. 0 shares → `ZeroAmount`
7. вывод из чужой позиции → `ConstraintSeeds`
8. подмена vault-аккаунта → `ConstraintHasOne`
9. вывод работает во время паузы
10. полный вывод — все shares сожжены
11. нельзя закрыть непустую позицию → `PositionNotEmpty`
12. пустая позиция закрывается, rent возвращается
13. vault платёжеспособен: оставшиеся shares полностью обеспечены

Unit-тесты (`cargo test --lib`): 14 тестов математики — все проходят.

## Запуск

Требуется: Rust, Solana CLI 1.18+, Anchor 0.30.1, Node.js.

```bash
npm install
anchor keys sync
anchor build
anchor test
cargo test --lib
```

## Дальше (неделя 11+)

- CPI в пул ликвидности: `add_liquidity` после депозита, `remove_liquidity` при выводе, если в vault мало свободных токенов (`InsufficientLiquidity`).
