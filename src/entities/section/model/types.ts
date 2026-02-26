export interface ISection {
  uuid: string;
  name: string;
}

export interface ISectionActions {
  addSection: (name: ISection["name"]) => void;
  removeSection: (uuid: ISection["uuid"]) => void;
  editSection: (uuid: ISection["uuid"], sectionData: Partial<Omit<ISection, "uuid">>) => void;
}
