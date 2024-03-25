import { Sidebar } from "../components/ui/sidebar";
import { useRoute } from "../router/router";
import RouterView from "../router/RouterView";
import { Provider } from "jotai";
import { Suspense } from "react";

export default function () {
	const [route, setRoute] = useRoute();

	return (
		<div className="p-10 grid grid-cols-5">
			<Provider>
				<Sidebar
					className="w-full bg-white dark:bg-slate-900"
					current={route}
					setRoute={setRoute}
				/>
				<div className="col-span-4">
					<Suspense fallback={<div>Loading...</div>}>
						<RouterView route={route} />
					</Suspense>
				</div>
			</Provider>
		</div>
	);
}
