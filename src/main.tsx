import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import Note from "./pages/note";

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
  },
  {
    path: "/note",
    element: <Note />,
  },
]);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={router} />
  </React.StrictMode>
);
