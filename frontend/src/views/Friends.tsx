import type { User, Users } from "@/types/vrchat";
import {
	Accordion,
	AccordionContent,
	AccordionItem,
	AccordionTrigger,
} from "@/components/ui/accordion";
import { offline, online, web } from "@/atoms";
import { UserView } from "@/components/user";
import { useAtomValue, useErrorToast } from "@/components/ui/use-toast";
import { useMemo } from "react";

export default function () {
	const errorToast = useErrorToast<User[]>();

	// biome-ignore lint/correctness/useExhaustiveDependencies: <explanation>
	const useAtomVal = useMemo(() => useAtomValue(errorToast), []);

	const users = [
		["online", useAtomVal(online)],
		["web", useAtomVal(web)],
		["offline", useAtomVal(offline)],
	] as const satisfies Array<[keyof Exclude<Users, "myself">, User[]]>;

	return (
		<Accordion className="w-full" type="single" collapsible>
			{users.map(([type, users]) => (
				<AccordionItem key={type} value={type}>
					<AccordionTrigger className="font-bold">{type}</AccordionTrigger>
					<AccordionContent className="flex flex-wrap justify-center">
						{users.map((user) => (
							<UserView key={user.id} user={user} />
						))}
					</AccordionContent>
				</AccordionItem>
			))}
		</Accordion>
	);
}
