# update_status

Частичное обновление статуса.

## Tauri-команда

```
update_status
```

## Входные данные

| Параметр | Тип            | Описание            |
|----------|----------------|---------------------|
| `uuid`   | `string`       | UUID статуса        |
| `data`   | `UpdateStatus` | Поля для обновления |

Объект `UpdateStatus` (все поля опциональные):

```json
{
  "name": "Новое название",
  "color": "#ff0000",
  "position": 5
}
```

| Поле       | Тип              | Описание            |
|------------|------------------|---------------------|
| `name`     | `string \| null` | Новое название      |
| `color`    | `string \| null` | Новый цвет          |
| `position` | `number \| null` | Новая позиция       |

## Выходные данные

Объект `Status` — обновлённый статус.

## Ошибки

| Ошибка                         | Когда                            |
|--------------------------------|----------------------------------|
| `Не найдено: Статус {uuid}...` | Статус с таким UUID не существует |
| `Ошибка базы данных`            | Проблемы с SQLite-запросом        |

## Пример вызова (TypeScript)

```ts
const updated = await invoke<Status>("update_status", {
  uuid: "...",
  data: { position: 0 }
});
```

## Исходный код

- Команда: `src-tauri/src/commands/status_cmd.rs` → `update_status`
- Репозиторий: `src-tauri/src/repository/status_repo.rs` → `update()`
