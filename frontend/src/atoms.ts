import { atom } from "jotai";
import type { Config } from "./types/config";
import type { Users } from "./types/vrchat";

export const config: Config = JSON.parse(
	localStorage.getItem("config") ?? "{}",
);

const fetch = (
	url: RequestInfo | URL,
	body?: BodyInit | null,
): Promise<Users | Error> =>
	window
		.fetch(url, {
			method: body ? "POST" : "GET",
			body,
			headers: { "Content-Type": "application/json" },
		})
		.then((r) =>
			r.ok
				? r.json()
				: new Error(
						`failed to fetch: ${url}
					 	 statusCode: ${r.status}`,
					),
		)
		.catch((e) => e);

const users = config.alternativeServer
	? fetch(config.alternativeServer.url, config.alternativeServer.auth)
	: fetch(`${location.href}friends`);

const get =
	<T extends keyof Users>(type: T) =>
	() =>
		users.then((users) => {
			if (users instanceof Error) return users;
			return users[type];
		});

export const myself = atom(get("myself"));
export const online = atom(get("online"));
export const web = atom(get("web"));
export const offline = atom(get("offline"));
