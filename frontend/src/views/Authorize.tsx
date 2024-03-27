import fetch from "@/fetch";
import Settings from "./Settings";
import type { ReactNode } from "react";

type Config = {
	authorized: boolean;
};

export default function ({ children }: { children: ReactNode }) {
	const config = fetch<Config>("i", "i");

	if (["localhost", "127.0.0.1", "::1"].includes(location.hostname)) {
		if (config instanceof Error) {
			return (
				<div>
					<div>{config.name}</div>
					<div>{config.message}</div>
				</div>
			);
		}

		if (!config.authorized) return <Settings />;
	}

	return children;
}
