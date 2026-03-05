import { ISection } from "@/entities/section/model/types";

export interface ITask {
  uuid: string;
  statusUuid?: ISection["uuid"];
  projectUuid?: string;
  priorityUuid: string;
  name: string;
  description: string;
  dueDate?: string;
  startDate?: string;
  estimatedMinutes?: number;
}

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
