import React from "react";
import ReactDOM from "react-dom/client";
import App from "./views/App.tsx";
import "./globals.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

const client = new QueryClient();

// biome-ignore lint/style/noNonNullAssertion: <explanation>
ReactDOM.createRoot(document.getElementById("root")!).render(
	<React.StrictMode>
		<QueryClientProvider client={client}>
			<App />
		</QueryClientProvider>
	</React.StrictMode>,
);
