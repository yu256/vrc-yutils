import type { User } from "@/types/vrchat";
import { VrcAvatar } from "../avatar";
import { sidebarItems, useRoute, useSetRoute } from "@/router/router";
import { myself } from "@/atoms";
import { type ReactNode, Suspense, useMemo } from "react";
import { useAtomValue, useErrorToast } from "./use-toast";

export const Sidebar = ({
	className,
}: {
	user?: Pick<
		User,
		| "currentAvatarThumbnailImageUrl"
		| "profilePicOverride"
		| "userIcon"
		| "displayName"
		| "status"
	>;
	className?: string;
}) => (
	<div className={className}>
		<SidebarInner>
			<Suspense fallback={<UserView isLoading />}>
				<UserView />
			</Suspense>
		</SidebarInner>
	</div>
);

const SidebarInner = ({ children }: { children: ReactNode }) => {
	const route = useRoute();
	const setRoute = useSetRoute();

	return (
		<aside
			id="sidebar"
			className="fixed left-0 top-0 z-40 h-screen w-[20vw] transition-transform"
			aria-label="Sidebar"
		>
			<div className="flex h-full flex-col overflow-y-auto border-r border-slate-200 bg-white px-3 py-4 dark:border-slate-700 dark:bg-slate-900">
				<div className="mb-10 flex items-center rounded-lg px-3 py-2 text-slate-900 dark:text-white">
					<span className="ml-3 text-base font-semibold">vrc-yutils</span>
				</div>
				<ul className="space-y-2 text-sm font-medium">
					{sidebarItems.map((item) => (
						<button
							key={item}
							type="button"
							onClick={() => setRoute(item)}
							className={
								item === route
									? "flex w-full items-center rounded-lg px-5 py-4 bg-slate-100 dark:hover:bg-slate-700"
									: "flex w-full items-center rounded-lg px-4 py-3 text-slate-900 hover:bg-slate-100 dark:text-white dark:hover:bg-slate-700"
							}
						>
							{item}
						</button>
					))}
					{children}
				</ul>
			</div>
		</aside>
	);
};

const fallBackUser = {
	displayName: "unauthorized",
	status: "offline",
} as const;

const UserView = ({ isLoading = false }) => {
	const errorToast = useErrorToast<User>();

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	const useAtomVal = useMemo(() => useAtomValue(errorToast), []);

	const user = isLoading ? fallBackUser : useAtomVal(myself);

	return (
		<div className="mt-auto grid grid-cols-3 place-items-center">
			<VrcAvatar className="min-w-14 max-h-[5rem]" user={user} showStatus={false} />
			<div className="w-full h-20 text-sm font-medium text-black dark:text-white flex items-center justify-center col-span-2">
				{user.displayName}
			</div>
		</div>
	);
};
