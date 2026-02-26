export interface ITask {
  uuid: string;
  sectionUuid: ISection["uuid"];
  name: string;
  description: string;
}

export interface ITaskActions {
  addTask: (
    sectionUuid: ITask["sectionUuid"],
    taskData: Omit<ITask, "uuid" | "sectionUuid">,
  ) => void;
  getTasks: (sectionUuid: ITask["sectionUuid"]) => ITask[];
}
