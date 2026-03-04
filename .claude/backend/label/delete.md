# delete_label

Удаление глобального лейбла. Каскадно удаляет все привязки к задачам.

## Tauri-команда

```
delete_label
```

## Входные данные

| Параметр | Тип      | Описание     |
|----------|----------|--------------|
| `uuid`   | `string` | UUID лейбла  |

## Выходные данные

`null` (успех).

## Побочные эффекты

- Удаляются все записи в `task_labels` с `label_uuid = uuid` (ON DELETE CASCADE).

## Ошибки

| Ошибка               | Когда                             |
|----------------------|-----------------------------------|
| `NotFound`           | Лейбл с таким UUID не существует |
| `Ошибка базы данных` | Проблемы с SQLite-запросом        |

## Пример вызова (TypeScript)

```ts
await invoke("delete_label", { uuid: "..." });
```

## Исходный код

- Команда: `src-tauri/src/commands/label_cmd.rs` → `delete_label`
- Репозиторий: `src-tauri/src/repository/label_repo.rs` → `delete()`
