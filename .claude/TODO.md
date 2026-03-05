# TODO

## Drag'n'Drop Composable

### Задача
Вынести всю логику Drag'n'Drop из компонентов в composable `useDragAndDrop`.

### Требования
- Composable хранит **Map<string, boolean>** где ключ — это ID элемента (секции/задачи), значение — состояние dragging (true/false)
- Можно получить состояние конкретного элемента по его ID: `isDragging(id)` → `boolean`
- Можно установить состояние: `setDragging(id, value)`
- Composable должен предоставлять обработчики событий (dragstart, dragend, dragenter, dragleave, drop)
- Использовать на `ItemSection` и `ListSection` (и потенциально на `ItemTask`)

### Зачем Map вместо одного boolean
- Позволяет отслеживать состояние каждого элемента независимо
- Несколько элементов могут быть в состоянии drag одновременно (например, при быстрых перетаскиваниях)
- Можно стилизовать конкретный drop-target, не затрагивая остальные

### Примерный API composable
```
useDragAndDrop() → {
  draggingMap: Map<string, boolean>    // реактивная карта состояний
  isDragging(id: string): boolean      // получить состояние по ID
  setDragging(id: string, val: boolean) // установить состояние
  onDragStart(id, event)               // обработчик dragstart
  onDragEnd(id, event)                 // обработчик dragend
  onDragEnter(id, event)               // обработчик dragenter
  onDragLeave(id, event)               // обработчик dragleave
  onDrop(id, event, callback)          // обработчик drop с пользовательским callback
}
```

### Где использовать
- `ItemSection.vue` — убрать локальный `isDragging`, заменить на composable
- `ItemTask.vue` — убрать drag-логику, подключить composable
- `ListSection.vue` — при необходимости для section-level drag
