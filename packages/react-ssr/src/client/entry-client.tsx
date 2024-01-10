import React from "react";
import { createRoot, hydrateRoot } from "react-dom/client";
import { QueryClient, QueryClientProvider, Hydrate } from "react-query";
import { Router } from "./Router";
import { AppProvider } from "./store/StoreProvider";
import { BrowserRouter } from "react-router-dom";
import { App, Shelf } from "./types";

import "./css/font.css";
import "./css/normalize.lib.css";
import "./css/mustard-ui.lib.css";
import "./css/main.css";

const queryClient = new QueryClient();
// @ts-ignore __REACT_QUERY_STATE__
const dehydratedState = window.__REACT_QUERY_STATE__;

const container = document.getElementById("root");
const apps = dehydratedState?.queries?.[0]?.state?.data as App[] | undefined;
const shelf = dehydratedState?.queries?.[1]?.state?.data as Shelf[] | undefined;

const FullApp = () => (
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <Hydrate state={dehydratedState}>
        <AppProvider apps={apps} shelf={shelf}>
          <BrowserRouter>
            <Router />
          </BrowserRouter>
        </AppProvider>
      </Hydrate>
    </QueryClientProvider>
  </React.StrictMode>
);

if (import.meta.hot || !container?.innerText) {
  const root = createRoot(container!);
  root.render(<FullApp />);
} else {
  hydrateRoot(container!, <FullApp />);
}
