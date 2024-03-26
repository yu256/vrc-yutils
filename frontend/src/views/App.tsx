import { Toaster } from "@/components/ui/toaster";
import { Sidebar } from "../components/ui/sidebar";
import RouterView from "../router/RouterView";
import { Provider } from "jotai";
import { Suspense } from "react";
import Settings from "./Settings";
import Authorize from "./Authorize";

export default () => (
	<div className="p-10">
		<Authorize>
			<Provider>
				<div className="grid grid-cols-5">
					<div className="w-full bg-white dark:bg-slate-900">
						<Sidebar />
					</div>
					<div className="col-span-4">
						<Suspense fallback={<Settings />}>
							<RouterView />
						</Suspense>
					</div>
				</div>
			</Provider>
		</Authorize>
		<Toaster />
	</div>
);
