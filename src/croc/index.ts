import { reactive } from "vue";
import { Croc } from "./croc";

export * from "./transfer-state";

export const croc = reactive(new Croc());
