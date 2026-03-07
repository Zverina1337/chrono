import * as z from "zod";
import { ITaskCreateSchema } from "./types";

export const TaskSchema = z.object({
  uuid: z.uuid({ version: "v4" }),
  statusUuid: z.uuid({ version: "v4" }).nullable(),
  projectUuid: z.uuid({ version: "v4" }),
  priorityUuid: z.uuid({ version: "v4" }).nullable(),
  name: z.string().max(255),
  description: z.string().max(255),
  dueDate: z.string().max(255).nullable(),
  startDate: z.string().max(255).nullable(),
  estimatedMinutes: z.number().nullable(),
  position: z.int32(),
  createdAt: z.iso.datetime(),
  updatedAt: z.iso.datetime(),
});

export const createTaskSchema = TaskSchema.omit({
  uuid: true,
  createdAt: true,
  updatedAt: true,
  position: true,
});

export const defaultCreateTaskSchema = (): ITaskCreateSchema => ({
  statusUuid: null,
  projectUuid: "",
  priorityUuid: null,
  name: "",
  description: "",
  dueDate: null,
  startDate: null,
  estimatedMinutes: null,
});
