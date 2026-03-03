# create_status

Создание нового статуса (канбан-колонки) в проекте.

## Tauri-команда

```
create_status
```

## Входные данные

Объект `CreateStatus`:

```json
{
  "projectUuid": "550e8400-...",
  "name": "На ревью",
  "color": "#8b5cf6"
}
```

| Поле          | Тип      | Обязательное | По умолчанию | Описание              |
|---------------|----------|--------------|--------------|-----------------------|
| `projectUuid` | `string` | Да           | —            | UUID проекта          |
| `name`        | `string` | Да           | —            | Название статуса      |
| `color`       | `string` | Нет          | `#6366f1`    | HEX-цвет             |

## Выходные данные

Объект `Status` — созданный статус.

## Логика

1. Генерируется UUID v4
2. `position` = MAX(position) + 1 среди статусов этого проекта
3. INSERT + возврат объекта

## Ошибки

| Ошибка              | Когда                    |
|---------------------|--------------------------|
| `Ошибка базы данных` | Проблемы с SQLite-запросом |

## Пример вызова (TypeScript)

```ts
const status = await invoke<Status>("create_status", {
  data: { projectUuid: "...", name: "На ревью" }
});
```

## Исходный код

- Команда: `src-tauri/src/commands/status_cmd.rs` → `create_status`
- Репозиторий: `src-tauri/src/repository/status_repo.rs` → `create()`
