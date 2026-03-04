# update_subtask

Частичное обновление подзадачи. Все поля опциональны.

## Tauri-команда

```
update_subtask
```

## Входные данные

| Параметр      | Тип               | Обязательно | Описание                    |
|---------------|-------------------|:-----------:|-----------------------------|
| `uuid`        | `string`          | да          | UUID подзадачи              |
| `name`        | `string \| null`  | нет         | Новое название              |
| `isCompleted` | `boolean \| null` | нет         | Переключить выполнение      |
| `position`    | `number \| null`  | нет         | Новая позиция               |

## Выходные данные

Обновлённый объект `Subtask`.

## Ошибки

| Ошибка               | Когда                                |
|----------------------|--------------------------------------|
| `NotFound`           | Подзадача с таким UUID не существует |
| `Ошибка базы данных` | Проблемы с SQLite-запросом           |

## Пример вызова (TypeScript)

```ts
// Отметить подзадачу выполненной
const updated = await invoke<Subtask>("update_subtask", {
  uuid: "...",
  isCompleted: true,
  name: null,
  position: null
});
```

## Исходный код

- Команда: `src-tauri/src/commands/subtask_cmd.rs` → `update_subtask`
- Репозиторий: `src-tauri/src/repository/subtask_repo.rs` → `update()`
