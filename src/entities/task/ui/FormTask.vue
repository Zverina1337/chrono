<script setup lang="ts">
import { reactive } from "vue";
import Button from "@/shared/ui/Button.vue";
import BaseInput from "@/shared/ui/BaseInput.vue";
import { defaultCreateTaskSchema, createTaskSchema } from "../model/validation";
import * as z from "zod";
import BaseTextarea from "@/shared/ui/BaseTextarea.vue";
import BaseSelect from "@/shared/ui/BaseSelect.vue";

const form = reactive(defaultCreateTaskSchema());
const validate = () => {
  const result = z.safeParse(createTaskSchema, form);
  console.log(result);
};
</script>
<template>
  <form flex="~ col" text="white" gap="4">
    <BaseSelect
      v-model="form.projectUuid"
      name="task-project"
      label="Проект"
      :options="[
        { value: 'project-1', label: 'project-1' },
        { value: 'project-2', label: 'project-2' },
        { value: 'project-3', label: 'project-3' },
      ]"
    />
    <BaseSelect
      v-model="form.statusUuid"
      label="Статус"
      name="task-status"
      :options="[
        { value: 'status-1', label: 'status-1' },
        { value: 'status-2', label: 'status-2' },
        { value: 'status-3', label: 'status-3' },
      ]"
    />
    <BaseSelect
      v-model="form.priorityUuid"
      label="Приоритет"
      name="task-priority"
      :options="[
        { value: 'priority-1', label: 'priority-1' },
        { value: 'priority-2', label: 'priority-2' },
        { value: 'priority-3', label: 'priority-3' },
      ]"
    />
    <BaseInput v-model="form.name" name="task-name" label="Название задачи:" />
    <BaseTextarea v-model="form.description" name="task-descritpion" label="Описание:" />
    <BaseInput v-model="form.dueDate" name="task-due-date" label="Срок окончания:" type="date" />
    <BaseInput
      v-model.number="form.estimatedMinutes"
      name="task-estimated-minutes"
      label="Оценка в минутах:"
    />
    <Button @click="validate" type="button" mt="5">Создать</Button>
  </form>
</template>
