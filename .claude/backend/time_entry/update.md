# update_time_entry

Частичное обновление записи времени. Используется прежде всего для остановки запущенного таймера.

## Tauri-команда

```
update_time_entry
```

## Входные данные

| Параметр          | Тип                        | Обязательно | Описание                                         |
|-------------------|----------------------------|:-----------:|--------------------------------------------------|
| `uuid`            | `string`                   | да          | UUID записи времени                              |
| `endedAt`         | `(string \| null) \| null` | нет         | Время конца; `null`-значение сбрасывает в NULL   |
| `durationMinutes` | `(number \| null) \| null` | нет         | Продолжительность в минутах                      |
| `description`     | `string \| null`           | нет         | Новое описание                                   |

## Выходные данные

Обновлённый объект `TimeEntry`.

## Ошибки

| Ошибка               | Когда                                    |
|----------------------|------------------------------------------|
| `NotFound`           | Запись с таким UUID не существует        |
| `Ошибка базы данных` | Проблемы с SQLite-запросом               |

## Пример вызова (TypeScript)

```ts
// Остановить таймер
const updated = await invoke<TimeEntry>("update_time_entry", {
  uuid: "...",
  endedAt: "2026-03-05 11:30:00",
  durationMinutes: 90,
  description: null
});
```

## Исходный код

- Команда: `src-tauri/src/commands/time_entry_cmd.rs` → `update_time_entry`
- Репозиторий: `src-tauri/src/repository/time_entry_repo.rs` → `update()`
