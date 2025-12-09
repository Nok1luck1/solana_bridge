================================================================================
🎯 SOLANA & ANCHOR DEVELOPMENT - ПОЛНЫЙ ГАЙД ШПАРГАЛКА
================================================================================

📚 ОГЛАВЛЕНИЕ:
1. Базовые концепции Solana
2. Anchor основы
3. Безопасность и проверки
4. PDA (Program Derived Address)
5. UncheckedAccount и внешние интеграции
6. Compute Units и оптимизация
7. Rent и управление памятью
8. Error handling
9. Тестирование
10. Production best practices
11. Вопросы с собеседований

================================================================================

1. 📖 БАЗОВЫЕ КОНЦЕПЦИИ SOLANA
================================================================================

✅ Ключевые отличия от Ethereum:
- Аккаунто-ориентированная модель (не UTXO)
- Программы (не смарт-контракты)
- Rent (аренда за хранение данных)
- Parallel execution

✅ Типы аккаунтов:
1. System-owned (исполняемые программы)
2. Program-owned (данные программ)
3. User-owned (кошельки пользователей)

✅ Поля аккаунта:
- lamports: баланс
- data: сырые данные
- owner: программа-владелец
- executable: является ли программой
- rent_epoch: эпоха ренты

================================================================================

2. 🏗️ ANCHOR ОСНОВЫ
================================================================================

#[program]
Модуль программы - точка входа.

#[derive(Accounts)]
Структура валидации аккаунтов.

#[account]
Структура данных аккаунта.

#[error_code]
Кастомные ошибки программы.

#[instruction(...)]
Доступ к параметрам инструкции в валидации.

✅ Важные constraints:
- init - инициализация аккаунта
- mut - аккаунт для записи
- seeds/bump - PDA валидация
- has_one - проверка связи полей
- address - проверка адреса
- constraint - кастомная проверка

✅ Box:
- Используется для больших структур (>500 байт)
- Anchor автоматически делает deref
- Не нужно разыменовывать вручную
- Пример: `pub config: Box<Account<'info, Config>>`

================================================================================

3. 🛡️ БЕЗОПАСНОСТЬ И ПРОВЕРКИ
================================================================================

❌ НИКОГДА не делай:
- Хардкодить админа в константе
- Доверять данным из UncheckedAccount без проверок
- Проверять подпись после выполнения логики
- Использовать unchecked арифметику

✅ Всегда проверяй:
1. Владельца аккаунта (owner)
2. Подпись (is_signer)
3. PDA seeds
4. Discriminator для Anchor аккаунтов
5. Размер данных (data_len)

✅ Паттерн Checks-Effects-Interactions:
1. Checks - все проверки
2. Effects - изменение состояния
3. Interactions - CPI вызовы

================================================================================

4. 🔐 PDA (PROGRAM DERIVED ADDRESS)
================================================================================

PDA - адрес, сгенерированный программой, не имеющий приватного ключа.

✅ Использование:
- Хранение данных программы
- Подпись транзакций от имени программы
- Связка связанных аккаунтов

✅ Генерация:
let (pda, bump) = Pubkey::find_program_address(
    &[b"seed", authority.key().as_ref()],
    program_id
);

✅ В Anchor:
#[account(
    seeds = [b"config", authority.key().as_ref()],
    bump = config.bump
)]
pub config: Account<'info, Config>;

================================================================================

5. 🌉 UNCHECKEDACCOUNT И ВНЕШНИЕ ИНТЕГРАЦИИ
================================================================================

UncheckedAccount - аккаунт, который Anchor не проверяет автоматически.

✅ Когда использовать:
- Интеграция с внешними программами (оракулы, DEX)
- Работа с данными неизвестной структуры
- Оптимизация compute units

✅ Обязательные проверки:
#[account(
    constraint = oracle.owner == pyth::ID,
    constraint = oracle.data_len() >= MIN_SIZE,
    constraint = !oracle.executable
)]
pub oracle: UncheckedAccount<'info>;

✅ Безопасный парсинг:
1. Проверяй версию данных
2. Добавляй bounds checking
3. Валидируй бизнес-логикой
4. Имей fallback механизмы

================================================================================

6. ⚡ COMPUTE UNITS И ОПТИМИЗАЦИЯ
================================================================================

Compute Units (CU) - единицы вычислений, лимит ~200K CU за транзакцию.

✅ Методы оптимизации:
- Используй batch processing
- Избегай ненужных циклов
- Кэшируй expensive операции
- Используй zero-copy аккаунты

✅ Zero-copy (#[account(zero_copy)]):
- Прямой доступ к памяти
- Нет сериализации/десериализации
- Только #[repr(C)] структуры

✅ Мониторинг CU:
let start = solana_program::log::sol_log_compute_units();
// ... операция
let used = solana_program::log::sol_log_compute_units() - start;

================================================================================

7. 💰 RENT И УПРАВЛЕНИЕ ПАМЯТЬЮ
================================================================================

Rent - плата за хранение данных на блокчейне.

✅ Rent exemption:
- Минимальный баланс для вечного хранения
- Рассчитывается: Rent::get()?.minimum_balance(space)

✅ Расчет space:
#[account]
pub struct User {
    pub balance: u64,      // 8
    pub owner: Pubkey,     // 32
    pub timestamp: i64,    // 8
}
// Общий размер: 8 (discriminator) + 8 + 32 + 8 = 56 байт

✅ Используй INIT_SPACE:
let space = 8 + User::INIT_SPACE;
let lamports_required = Rent::get()?.minimum_balance(space);

================================================================================

8. 🚨 ERROR HANDLING
================================================================================

✅ Кастомные ошибки с параметрами:
#[error_code]
pub enum MyError {
    #[msg("Insufficient: have {0}, need {1}")]
    Insufficient(u64, u64),
}

✅ Возврат ошибок:
err!(MyError::Insufficient(balance, amount))

✅ Error propagation в CPI:
token::transfer(cpi_ctx, amount).map_err(|_| MyError::TransferFailed)?;

✅ Логирование:
msg!("[ERROR] User {} failed: {:?}", user.key(), error);

================================================================================

9. 🧪 ТЕСТИРОВАНИЕ
================================================================================

✅ Anchor тесты (TypeScript):
describe("program", () => {
  it("initializes", async () => {
    await program.methods
      .initialize()
      .accounts({ ... })
      .rpc();
    
    const account = await program.account.config.fetch(pda);
    assert(account.authority.equals(wallet.publicKey));
  });
});

✅ Unit тесты (Rust):
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculation() {
        let result = calculate(2, 3);
        assert_eq!(result, 5);
    }
}

✅ Интеграционные тесты:
- Тестирование полного flow
- Мокирование оракулов
- Тестирование edge cases

================================================================================

10. 🏭 PRODUCTION BEST PRACTICES
================================================================================

✅ Security:
1. Всегда checked arithmetic
2. Проверяй timestamp свежести данных
3. Circuit breaker для внешних вызовов
4. Rate limiting
5. Multi-sig для критичных операций

✅ Мониторинг:
1. Логируй все важные события
2. Отслеживай CU usage
3. Мониторь ошибки
4. Alerting при аномалиях

✅ Upgradeability:
1. Versioned accounts
2. Migration instructions
3. Backward compatibility
4. Graceful degradation

✅ Gas оптимизация:
1. Минимизируй количество аккаунтов
2. Используй PDA вместо отдельных аккаунтов
3. Batch операции
4. Оптимизируй сериализацию

================================================================================

11. 💼 ВОПРОСЫ С СОБЕСЕДОВАНИЙ
================================================================================

❓ "Как работает rent exemption?"
✅ Аккаунт освобождается от rent если баланс ≥ minimum_balance(space)

❓ "Что такое account writability?"
✅ Только writable аккаунты могут изменять data/lamports

❓ "Как защититься от reentrancy в Solana?"
✅ Checks-Effects-Interactions, атомарность транзакций

❓ "Разница между SystemProgram.create_account и anchor init?"
✅ Anchor init добавляет discriminator и проверки

❓ "Как сделать upgradeable программу?"
✅ Buffer account + program upgrade authority

❓ "Что такое CPI reallocation?"
✅ Программа может изменить размер аккаунта при вызове

❓ "Как работает parallel execution?"
✅ Solana выполняет независимые транзакции параллельно

❓ "Что такое zk-proofs на Solana?"
✅ light-protocol, zk-proofs для приватности

================================================================================

12. 🛠️ ПОЛЕЗНЫЕ КОМАНДЫ
================================================================================

# Разработка
anchor build      # сборка
anchor test       # тесты
anchor deploy     # деплой
anchor verify     # верификация

# Клиент
anchor run test   # запуск тестов
anchor idl parse  # парсинг IDL
anchor init       # инициализация проекта

# Solana CLI
solana balance    # баланс
solana airdrop    # аирдроп
solana config     # конфигурация
solana program    управление программами

================================================================================

13. 📚 РЕСУРСЫ ДЛЯ ИЗУЧЕНИЯ
================================================================================

Официальная документация:
- https://www.anchor-lang.com/
- https://docs.solana.com/
- https://spl.solana.com/

Практические руководства:
- Solana Cookbook: https://solanacookbook.com/
- Anchor Examples: https://github.com/coral-xyz/anchor/tree/master/examples

Security:
- Solana Security Best Practices
- Neodyme Security Blog

Код production проектов:
- Jupiter Aggregator
- Marinade Finance
- Solend Protocol
- Raydium Protocol

================================================================================

14. ⚠️ ЧАСТЫЕ ОШИБКИ И ИХ РЕШЕНИЕ
================================================================================

1. "Error: Signature verification failed"
   ✅ Проверь is_signer и правильность PDA signer

2. "Error: Account not associated with this program"
   ✅ Проверь owner аккаунта

3. "Error: Not enough compute units"
   ✅ Оптимизируй код, используй batch processing

4. "Error: Account already initialized"
   ✅ Проверь discriminator или используй reinit

5. "Error: Constraint has-one was violated"
   ✅ Проверь соответствие полей в constraint

6. "Error: The program could not deserialize"
   ✅ Проверь структуру данных и discriminator

================================================================================

🎯 ЗАПОМНИ ГЛАВНОЕ:
1. Solana ≠ Ethereum - учи отличия
2. Безопасность прежде всего - проверяй всё
3. Anchor упрощает, но не заменяет понимание
4. Тестируй всё - от unit до integration
5. Оптимизируй compute units и rent
6. UncheckedAccount = твоя ответственность
7. PDA - мощный инструмент, используй правильно

================================================================================