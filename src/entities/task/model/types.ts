import * as z from "zod";
import { TaskSchema, createTaskSchema } from "./validation";

export type ITask = z.infer<typeof TaskSchema>;
export type ITaskCreateSchema = z.infer<typeof createTaskSchema>;

export interface ITaskActions {
  addTask: (projectUuid: ITask["projectUuid"], taskData: ITaskCreateSchema) => void;
  getTasks: (projectUuid: ITask["projectUuid"]) => ITask[];
  deleteTask: (projectUuid: ITask["projectUuid"], uuid: ITask["uuid"]) => void;
  moveTask: (task: ITask, toProjectUuid: ITask["projectUuid"]) => void;
  swapTask: (task: ITask, targetTask: ITask) => void;
  getTask: (projectUuid: ITask["projectUuid"], uuid: ITask["uuid"]) => ITask | null;
}
