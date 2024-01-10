import { Routes, Route } from "react-router-dom";
import { Nav } from "./components/Nav";
import { App } from "./pages/App";
import { Shelf } from "./pages/Shelf";
import { Store } from "./pages/Store";

export function Router() {
  return (
    <>
      <Nav />
      <main className="container">
        <Routes>
          <Route path="/" element={<Store />} />
          <Route path="/store" element={<Store />} />
          <Route path="/shelf" element={<Shelf />} />
          <Route path="/app/:id" element={<App />} />
        </Routes>
      </main>
    </>
  );
}
