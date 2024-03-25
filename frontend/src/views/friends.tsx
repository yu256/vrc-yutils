import type { User, Users } from "@/types/vrchat";
import {
	Accordion,
	AccordionContent,
	AccordionItem,
	AccordionTrigger,
} from "@/components/ui/accordion";
import { useAtomValue } from "jotai";
import { offline, online, web } from "@/atoms";
import { UserView } from "@/components/user";

export default function () {
	const users = [
		["online", useAtomValue(online)],
		["web", useAtomValue(web)],
		["offline", useAtomValue(offline)],
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
