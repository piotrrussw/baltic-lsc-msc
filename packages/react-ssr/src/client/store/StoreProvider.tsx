import { FC, ReactNode, createContext, useContext, useReducer } from "react";
import { App, Shelf, Filter } from "../types";

type State = {
  apps: App[];
  shelf: Shelf[];
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

type FiltersAction = {
  type: "SET_FILTERS";
  payload: Filter;
};

type Action = AppsAction | ShelfAction | FiltersAction;

const getInitialState = (apps?: App[], shelf?: Shelf[]): State => ({
  apps: apps || [],
  shelf: shelf || [],
  filters: { sort: "date", search: "" },
});

const AppContext = createContext<{ state: State; dispatch: React.Dispatch<Action> } | undefined>(undefined);

const reducer = (state: State, action: Action) => {
  switch (action.type) {
    case "SET_APPS":
      return { ...state, apps: action.payload };
    case "SET_SHELF":
      return { ...state, shelf: action.payload };
    case "SET_FILTERS":
      return { ...state, filters: action.payload };
    default:
      return state;
  }
};

type AppProviderProps = {
  children: ReactNode;
  apps?: App[];
  shelf?: Shelf[];
};

export const AppProvider: FC<AppProviderProps> = ({ apps, shelf, children }) => {
  const [state, dispatch] = useReducer(reducer, getInitialState(apps, shelf));

  return <AppContext.Provider value={{ state, dispatch }}>{children}</AppContext.Provider>;
};

export const useStore = () => {
  const context = useContext(AppContext);
  if (!context) {
    throw new Error("useAppState must be used within an AppProvider");
  }

  return context;
};
