import { defineStore } from "pinia";
import { Ref, ref } from "vue";
import { ISection, ISectionActions } from "./types";

export const useSectionStore = defineStore("section", () => {
  const sections: Ref<Array<ISection>> = ref([
    {
      uuid: crypto.randomUUID(),
      name: "К работе",
    },
    {
      uuid: crypto.randomUUID(),
      name: "В работе",
    },
    {
      uuid: crypto.randomUUID(),
      name: "Сделано",
    },
  ]);

  const addSection: ISectionActions["addSection"] = (name) => {
    const newSection: ISection = {
      uuid: crypto.randomUUID(),
      name,
    };
    sections.value = [...sections.value, newSection];
  };

  const removeSection: ISectionActions["removeSection"] = (uuid) => {
    sections.value = sections.value.filter((section) => section.uuid !== uuid);
  };

  const editSection: ISectionActions["editSection"] = (uuid, sectionData) => {
    const section = sections.value.findIndex((section) => section.uuid === uuid);
    if (section !== -1) {
      sections.value[section] = { ...sections.value[section], ...sectionData };
    }
  };

  return { sections, addSection, removeSection, editSection };
});
