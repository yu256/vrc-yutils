import { useSuspenseQuery } from "@tanstack/react-query";

const fetch = <T>(
	key: string,
	url: RequestInfo | URL,
	body?: string | object,
): T | Error =>
	useSuspenseQuery({
		queryKey: [key],
		queryFn: () =>
			window
				.fetch(url, {
					method: body ? "POST" : "GET",
					body: typeof body === "object" ? JSON.stringify(body) : body,
					headers:
						typeof body === "object"
							? { "Content-Type": "application/json" }
							: undefined,
				})
				.then((r) => r.json())
				.catch((e) => e),
	}).data;

export default fetch;
