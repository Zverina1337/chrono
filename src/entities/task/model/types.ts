import * as z from "zod";
import { TaskSchema, createTaskSchema } from "./validation";

export type ITask = z.infer<typeof TaskSchema>;
export type ITaskCreateSchema = z.infer<typeof createTaskSchema>;

export interface ITaskActions {
  addTask: (
    sectionUuid: ITask["statusUuid"],
    taskData: Omit<ITask, "uuid" | "sectionUuid">,
  ) => void;
  getTasks: (sectionUuid: ITask["statusUuid"]) => ITask[];
  deleteTask: (sectionUuid: ITask["statusUuid"], uuid: ITask["uuid"]) => void;
  moveTask: (task: ITask, toSectionUuid: ITask["statusUuid"]) => void;
  swapTask: (task: ITask, targetTask: ITask) => void;
  getTask: (sectionUuid: ITask["statusUuid"], uuid: ITask["uuid"]) => ITask | null;
}
