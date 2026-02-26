export interface ISection {
  uuid: string;
  name: string;
}

export interface ITask {
  uuid: string;
  sectionUuid: ISection["uuid"];
  name: string;
  description: string;
}

export interface ISectionActions {
  addSection: (name: ISection["name"]) => void;
  removeSection: (uuid: ISection["uuid"]) => void;
  editSection: (uuid: ISection["uuid"], sectionData: Partial<Omit<ISection, "uuid">>) => void;
}

export interface ITaskActions {
  addTask: (
    sectionUuid: ITask["sectionUuid"],
    taskData: Omit<ITask, "uuid" | "sectionUuid">,
  ) => void;
  getTasks: (sectionUuid: ITask["sectionUuid"]) => ITask[];
}
