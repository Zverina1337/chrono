<script setup lang="ts">
import { shallowRef } from "vue";
import { ITask } from "../model/types";

const form = shallowRef<ITask>();
</script>
<template>
  <form action="" flex="~ col" text="white">
    <label for="project">Проект</label>
    <select name="project" id="project">
      <option value="project-1">project-1</option>
      <option value="project-2">project-2</option>
      <option value="project-3">project-3</option>
    </select>

    <label for="project">Статус</label>
    <select name="status" id="status">
      <option value="status-1">status-1</option>
      <option value="status-2">status-2</option>
      <option value="status-3">status-3</option>
    </select>

    <label for="project">Приоритет</label>
    <select name="priority" id="priority">
      <option value="priority-1">priority-1</option>
      <option value="priority-2">priority-2</option>
      <option value="priority-3">priority-3</option>
    </select>

    <fieldset name="main-info">
      <legend>Основное:</legend>
      <label for="task-name">Название задачи:</label>
      <input type="text" name="task-name" id="task-name" />
      <label for="task-description">Описание:</label>
      <input type="text" name="task-description" id="task-description" />
    </fieldset>

    <fieldset name="time-info">
      <legend>Время:</legend>
      <label for="due-date">Deadline:</label>
      <input type="date" name="due-date" id="due-date" />
      <label for="estimated-minutes">Оценка в минутах:</label>
      <input type="text" name="estimated-minutes" id="estimated-minutes" />
    </fieldset>
  </form>
</template>
