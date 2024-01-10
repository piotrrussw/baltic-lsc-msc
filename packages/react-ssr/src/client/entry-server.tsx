import React from "react";
import ReactDOMServer from "react-dom/server";
import { StaticRouter } from "react-router-dom/server";
import { dehydrate, Hydrate, QueryClient, QueryClientProvider } from "react-query";
import { Router } from "./Router";
import { AppProvider } from "./store/StoreProvider";
import {
  GET_APP_LIST_QUERY_KEY,
  GET_DATA_SHELF_QUERY_KEY,
  GET_SHELF_QUERY_KEY,
  getApp,
  getAppList,
  getDataShelf,
  getShelf,
} from "./api";

import "./css/font.css";
import "./css/normalize.lib.css";
import "./css/mustard-ui.lib.css";
import "./css/main.css";

const getPrefetchList = (url: string) => {
  if (url === "/shelf") {
    return [
      {
        queryKey: [GET_DATA_SHELF_QUERY_KEY],
        queryFn: getDataShelf,
      },
    ];
  }

  if (url.startsWith("/app/")) {
    const id = url.split("/").pop();
    const path = `?appUid=${id}`;

    return [
      {
        queryKey: [path],
        queryFn: getApp(path),
      },
    ];
  }

  return [
    {
      queryKey: [GET_APP_LIST_QUERY_KEY],
      queryFn: getAppList,
    },
    {
      queryKey: [GET_SHELF_QUERY_KEY],
      queryFn: getShelf,
    },
  ];
};

export async function render(url: string) {
  const queryClient = new QueryClient();
  const prefetchList = getPrefetchList(url);
  const jobs = prefetchList.map(({ queryKey, queryFn }) => queryClient.prefetchQuery(queryKey, queryFn));

  await Promise.all(jobs);

  const dehydratedState = dehydrate(queryClient);

  return {
    html: ReactDOMServer.renderToString(
      <React.StrictMode>
        <QueryClientProvider client={queryClient}>
          <Hydrate state={dehydratedState}>
            <AppProvider>
              <StaticRouter location={url}>
                <Router />
              </StaticRouter>
            </AppProvider>
          </Hydrate>
        </QueryClientProvider>
      </React.StrictMode>
    ),
    dehydratedState,
    clear: queryClient.clear(),
  };
}
