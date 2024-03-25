import type { User } from "@/types/vrchat";
import { VrcAvatar } from "./avatar";

export const UserView = ({ user }: { user: User }) => (
	<div className="w-60 grid grid-cols-3">
		<VrcAvatar user={user} className="w-20 h-20" />
		{user.statusDescription ? (
			<div className="m-auto col-span-2 grid place-items-center">
				<div className="font-bold text-base">{user.displayName}</div>
				<div className="font-thin text-xs">{user.statusDescription}</div>
			</div>
		) : (
			<div className="m-auto col-span-2 font-bold">{user.displayName}</div>
		)}
	</div>
);
