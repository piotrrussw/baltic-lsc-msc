import { useQuery } from "react-query";
import { getAppList, GET_APP_LIST_QUERY_KEY, getShelf, GET_SHELF_QUERY_KEY } from "../api";
import { App, Shelf } from "../types";
import { AppsList } from "../components/AppsList";
import { Filters } from "../components/Filters";
import { useStore } from "../store/StoreProvider";

export function Store() {
  const { dispatch } = useStore();

  const { isLoading: isAppLoading } = useQuery<App[]>([GET_APP_LIST_QUERY_KEY], getAppList, {
    onSuccess: (response) => {
      dispatch({ type: "SET_APPS", payload: response });
    },
    cacheTime: 300_000,
    staleTime: 300_000,
  });

  const { isLoading: isShelfLoading } = useQuery<Shelf[]>([GET_SHELF_QUERY_KEY], getShelf, {
    onSuccess: (response) => {
      dispatch({ type: "SET_SHELF", payload: response });
    },
    cacheTime: 300_000,
    staleTime: 300_000,
  });

  const isLoading = isShelfLoading || isAppLoading;

  return (
    <main>
      <Filters />
      <AppsList />
    </main>
  );
}
