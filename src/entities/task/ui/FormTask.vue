<script setup lang="ts">
import { reactive } from "vue";
import Button from "@/shared/ui/Button.vue";
import BaseInput from "@/shared/ui/BaseInput.vue";
import { defaultCreateTaskSchema, createTaskSchema } from "../model/validation";
import * as z from "zod";
import BaseTextarea from "@/shared/ui/BaseTextarea.vue";
import BaseSelect from "@/shared/ui/BaseSelect.vue";
import { ITaskCreateSchema } from "../model/types";

const emits = defineEmits<{ submit: [form: ITaskCreateSchema] }>();

const form = reactive(defaultCreateTaskSchema());
const submit = () => {
  const result = z.safeParse(createTaskSchema, form);
  console.log(result);

  if (result.success) {
    emits("submit", result.data);
  }
};

let i = 0;
let array1 = [];
let array2 = [];
let array3 = [];

const projectsOptions = new Array(11).fill({}).reduce(() => {
  array1.push({ value: `${crypto.randomUUID()}`, label: `project-${i++}` });
  return array1;
});
const statusOptions = new Array(11).fill({}).reduce(() => {
  array2.push({ value: `${crypto.randomUUID()}`, label: `status-${i++}` });
  return array2;
});
const priorityOptions = new Array(11).fill({}).reduce(() => {
  array3.push({ value: `${crypto.randomUUID()}`, label: `priority-${i++}` });
  return array3;
});
</script>
<template>
  <form flex="~ col" text="white" gap="4">
    <BaseSelect
      v-model="form.projectUuid"
      name="task-project"
      label="Проект"
      :options="projectsOptions"
    />
    <BaseSelect
      v-model="form.statusUuid"
      label="Статус"
      name="task-status"
      :options="statusOptions"
    />
    <BaseSelect
      v-model="form.priorityUuid"
      label="Приоритет"
      name="task-priority"
      :options="priorityOptions"
    />
    <BaseInput v-model="form.name" name="task-name" label="Название задачи:" />
    <BaseTextarea v-model="form.description" name="task-descritpion" label="Описание:" />
    <BaseInput v-model="form.dueDate" name="task-due-date" label="Срок окончания:" type="date" />
    <BaseInput
      v-model.number="form.estimatedMinutes"
      name="task-estimated-minutes"
      label="Оценка в минутах:"
    />
    <Button type="button" mt="5" @click="submit">Создать</Button>
  </form>
</template>
