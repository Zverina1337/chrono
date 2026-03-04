# create_priority

Создание нового приоритета в проекте.

## Tauri-команда

```
create_priority
```

## Входные данные

| Параметр      | Тип      | Обязательно | Описание                     |
|---------------|----------|:-----------:|------------------------------|
| `projectUuid` | `string` | да          | UUID проекта                 |
| `name`        | `string` | да          | Название приоритета          |
| `color`       | `string` | да          | HEX-цвет (напр. `#ef4444`)  |
| `level`       | `number` | да          | Числовой уровень важности    |

## Выходные данные

Объект `Priority` (см. `get_by_project.md`).

## Ошибки

| Ошибка               | Когда                      |
|----------------------|----------------------------|
| `Ошибка базы данных` | Проблемы с SQLite-запросом |

## Пример вызова (TypeScript)

```ts
const priority = await invoke<Priority>("create_priority", {
  projectUuid: "550e8400-...",
  name: "Критический",
  color: "#dc2626",
  level: 4
});
```

## Исходный код

- Команда: `src-tauri/src/commands/priority_cmd.rs` → `create_priority`
- Репозиторий: `src-tauri/src/repository/priority_repo.rs` → `create()`
