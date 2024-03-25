import { useSuspenseQuery } from "@tanstack/react-query";

const fetch = <T>(
	key: string,
	url: RequestInfo | URL,
	body?: string,
): T | Error =>
	useSuspenseQuery({
		queryKey: [key],
		queryFn: () =>
			window
				.fetch(url, {
					method: body ? "POST" : "GET",
					body,
					headers: body ? { "Content-Type": "application/json" } : undefined,
				})
				.then((r) => r.json())
				.catch((e) => e),
	}).data;

export default fetch;
