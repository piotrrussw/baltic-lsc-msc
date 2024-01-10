import React from "react";
import ReactDOM from "react-dom/client";
import { QueryClient, QueryClientProvider } from "react-query";
import { Router } from "./Router";
import { AppProvider } from "./store/StoreProvider";

import "./css/font.css";
import "./css/normalize.css";
import "./css/mustard-ui.min.css";
import "./css/main.css";

const queryClient = new QueryClient();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <AppProvider>
        <Router />
      </AppProvider>
    </QueryClientProvider>
  </React.StrictMode>
);
