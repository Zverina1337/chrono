<script setup lang="ts">
import { reactive } from "vue";
import Button from "@/shared/ui/Button.vue";
import BaseInput from "@/shared/ui/BaseInput.vue";
import {
  defaultCreateProjectSchema,
  CreateProjectSchema,
} from "../model/validation";
import * as z from "zod";
import BaseTextarea from "@/shared/ui/BaseTextarea.vue";
import { IProjectCreate } from "../model/types";

const emits = defineEmits<{ submit: [form: IProjectCreate] }>();

const form = reactive(defaultCreateProjectSchema());
const submit = () => {
  const result = z.safeParse(CreateProjectSchema, form);
  console.log(result);

  if (result.success) {
    emits("submit", result.data);
  }
};
</script>
<template>
  <form class="flex flex-col gap-4 text-white">
    <BaseInput
      v-model="form.name"
      name="task-name"
      label="Название задачи:"
    />
    <BaseTextarea
      v-model="form.description"
      name="task-descritpion"
      label="Описание:"
    />

    <Button class="mt-5" type="button" @click="submit"
      >Создать</Button
    >
  </form>
</template>
