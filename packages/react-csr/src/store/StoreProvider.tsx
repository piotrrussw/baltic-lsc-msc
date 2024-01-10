import { FC, ReactNode, createContext, useContext, useReducer } from "react";
import { App, Shelf, DataShelf, Filter } from "../types";

type State = {
  apps: App[];
  shelf: Shelf[];
  dataShelf: DataShelf[];
  filters: Filter;
};

type AppsAction = {
  type: "SET_APPS";
  payload: App[];
};

type ShelfAction = {
  type: "SET_SHELF";
  payload: Shelf[];
};

type DataShelfAction = {
  type: "SET_DATA_SHELF";
  payload: DataShelf[];
};

type FiltersAction = {
  type: "SET_FILTERS";
  payload: Filter;
};

type Action = AppsAction | ShelfAction | FiltersAction | DataShelfAction;

const initialState: State = {
  apps: [],
  shelf: [],
  dataShelf: [],
  filters: { sort: "date", search: "" },
};

const AppContext = createContext<{ state: State; dispatch: React.Dispatch<Action> } | undefined>(undefined);

const reducer = (state: State, action: Action) => {
  switch (action.type) {
    case "SET_APPS":
      return { ...state, apps: action.payload };
    case "SET_SHELF":
      return { ...state, shelf: action.payload };
    case "SET_DATA_SHELF":
      return { ...state, dataShelf: action.payload };
    case "SET_FILTERS":
      return { ...state, filters: action.payload };
    default:
      return state;
  }
};

type AppProviderProps = {
  children: ReactNode;
};

export const AppProvider: FC<AppProviderProps> = ({ children }) => {
  const [state, dispatch] = useReducer(reducer, initialState);

  return <AppContext.Provider value={{ state, dispatch }}>{children}</AppContext.Provider>;
};

export const useStore = () => {
  const context = useContext(AppContext);
  if (!context) {
    throw new Error("useAppState must be used within an AppProvider");
  }

  return context;
};
