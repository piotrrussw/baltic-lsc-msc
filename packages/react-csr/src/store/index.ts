import { atom } from "jotai";

import { App, Shelf, Filter } from "../types";

export const appsAtom = atom<App[]>([]);

export const shelfAtom = atom<Shelf[]>([]);

export const filtersAtom = atom<Filter>({ sort: "date", search: "" });
