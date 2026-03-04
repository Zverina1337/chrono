# update_priority

Частичное обновление приоритета. Все поля опциональны — передавай только те, которые нужно изменить.

## Tauri-команда

```
update_priority
```

## Входные данные

| Параметр | Тип              | Обязательно | Описание                      |
|----------|------------------|:-----------:|-------------------------------|
| `uuid`   | `string`         | да          | UUID приоритета               |
| `name`   | `string \| null` | нет         | Новое название                |
| `color`  | `string \| null` | нет         | Новый HEX-цвет                |
| `level`  | `number \| null` | нет         | Новый уровень важности        |

## Выходные данные

Обновлённый объект `Priority`.

## Ошибки

| Ошибка               | Когда                               |
|----------------------|-------------------------------------|
| `NotFound`           | Приоритет с таким UUID не существует |
| `Ошибка базы данных` | Проблемы с SQLite-запросом          |

## Пример вызова (TypeScript)

```ts
const updated = await invoke<Priority>("update_priority", {
  uuid: "...",
  name: "Критический",
  color: null,
  level: null
});
```

## Исходный код

- Команда: `src-tauri/src/commands/priority_cmd.rs` → `update_priority`
- Репозиторий: `src-tauri/src/repository/priority_repo.rs` → `update()`
