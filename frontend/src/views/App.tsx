import { Toaster } from "@/components/ui/toaster";
import { Sidebar } from "../components/ui/sidebar";
import RouterView from "../router/RouterView";
import { Provider } from "jotai";
import { Suspense } from "react";
import Settings from "./Settings";

export default () => (
	<>
		<div className="p-10 grid grid-cols-5">
			<Provider>
				<div className="w-full bg-white dark:bg-slate-900">
					<Sidebar />
				</div>
				<div className="col-span-4">
					<Suspense fallback={<Settings />}>
						<RouterView />
					</Suspense>
				</div>
			</Provider>
		</div>
		<Toaster />
	</>
);
